//! Input-binding map — parse FS25's `inputBinding.xml` into a complete, searchable
//! view of every control binding, grouped by device, with inputs that drive multiple
//! actions surfaced for review.
//!
//! This is deliberately a VIEW, not a verdict. On a real profile the same physical
//! input legitimately maps to many actions across non-overlapping contexts (mouse-look
//! axes are reused by a dozen contextual actions; a button is a flashlight on foot and
//! headlights in a vehicle). Calling those "conflicts" would be almost all false
//! positives — the exact cry-wolf failure that makes people abandon a tool. So we show
//! where an input is reused and let the player judge, instead of asserting a clash.
//!
//! The in-game help menu only ever shows a handful of binds; this shows all of them —
//! the thing players resort to tracking by hand.

use quick_xml::events::Event;
use quick_xml::Reader;
use serde::Serialize;
use std::collections::BTreeMap;

/// One action↔input binding on a device.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bind {
    pub action: String,
    pub input: String,
}

/// A physical input driving 2+ distinct actions on the same device — flagged for the
/// player to review (often intentional/contextual, hence not called a conflict).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedInput {
    pub input: String,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBindings {
    pub device: String,
    /// Every binding on this device, sorted by action then input.
    pub bindings: Vec<Bind>,
    /// Inputs bound to 2+ distinct actions on this device.
    pub shared: Vec<SharedInput>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BindingReport {
    pub total_actions: usize,
    pub total_bindings: usize,
    pub devices: Vec<DeviceBindings>,
}

fn attr(e: &quick_xml::events::BytesStart, key: &[u8]) -> Option<String> {
    e.attributes().flatten().find(|a| a.key.as_ref() == key).and_then(|a| {
        a.unescape_value().ok().map(|v| v.into_owned())
    })
}

/// Parse `inputBinding.xml` into a per-device binding map. Pure — unit-testable.
pub fn parse(xml: &str) -> BindingReport {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut current_action: Option<String> = None;
    // Preserve document order per device via Vec; track distinct actions/bindings.
    let mut raw: Vec<(String, String, String)> = Vec::new(); // (device, action, input)

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                match e.name().as_ref() {
                    b"actionBinding" => current_action = attr(&e, b"action"),
                    b"binding" => {
                        if let Some(action) = &current_action {
                            let device = attr(&e, b"device").unwrap_or_default();
                            if let Some(input) = attr(&e, b"input") {
                                raw.push((device, action.clone(), input));
                            }
                        }
                    }
                    _ => {}
                }
            }
            // actionBinding closes -> clear context so a stray later binding isn't misfiled.
            Ok(Event::End(e)) if e.name().as_ref() == b"actionBinding" => current_action = None,
            Ok(Event::Eof) => break,
            Err(_) => break, // best-effort: a malformed tail shouldn't lose earlier bindings
            _ => {}
        }
        buf.clear();
    }

    // Group by device, preserving encounter order within each.
    let mut by_device: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    let total_bindings = raw.len();
    let mut distinct_actions = std::collections::HashSet::new();
    for (device, action, input) in raw {
        distinct_actions.insert(action.clone());
        by_device.entry(device).or_default().push((action, input));
    }

    let devices = by_device
        .into_iter()
        .map(|(device, mut pairs)| {
            // Shared inputs: input -> distinct actions, keep those with >= 2.
            let mut by_input: BTreeMap<String, Vec<String>> = BTreeMap::new();
            for (action, input) in &pairs {
                let acts = by_input.entry(input.clone()).or_default();
                if !acts.contains(action) {
                    acts.push(action.clone());
                }
            }
            let shared = by_input
                .into_iter()
                .filter(|(_, acts)| acts.len() >= 2)
                .map(|(input, actions)| SharedInput { input, actions })
                .collect();

            pairs.sort();
            let bindings = pairs.into_iter().map(|(action, input)| Bind { action, input }).collect();
            DeviceBindings { device, bindings, shared }
        })
        .collect();

    BindingReport {
        total_actions: distinct_actions.len(),
        total_bindings,
        devices,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Real-shape sample from an actual inputBinding.xml (2026-07): the compound
    // "MOUSE_BUTTON_* AXIS_*" inputs, the index/component attrs we ignore, and the
    // same input reused across actions.
    const SAMPLE: &str = r#"<?xml version="1.0"?>
<inputBinding version="24">
  <actionBinding action="JUMP">
    <binding device="KB_MOUSE_DEFAULT" input="KEY_space" />
  </actionBinding>
  <actionBinding action="ACTIVATE_HANDTOOL">
    <binding device="KB_MOUSE_DEFAULT" input="KEY_x" index="1"/>
    <binding device="KB_MOUSE_DEFAULT" input="MOUSE_BUTTON_LEFT" index="3"/>
  </actionBinding>
  <actionBinding action="ACTIVATE_HANDTOOL_SECONDARY">
    <binding device="KB_MOUSE_DEFAULT" input="KEY_x" index="1"/>
  </actionBinding>
  <actionBinding action="CROUCH">
    <binding device="GAMEPAD" input="BUTTON_B" />
  </actionBinding>
</inputBinding>"#;

    #[test]
    fn parses_actions_and_devices() {
        let r = parse(SAMPLE);
        assert_eq!(r.total_bindings, 5);
        assert_eq!(r.total_actions, 4); // JUMP, ACTIVATE_HANDTOOL, ..._SECONDARY, CROUCH
        assert_eq!(r.devices.len(), 2); // KB_MOUSE_DEFAULT, GAMEPAD
        let kb = r.devices.iter().find(|d| d.device == "KB_MOUSE_DEFAULT").unwrap();
        assert_eq!(kb.bindings.len(), 4);
    }

    #[test]
    fn flags_input_shared_across_actions_but_not_within_one() {
        let r = parse(SAMPLE);
        let kb = r.devices.iter().find(|d| d.device == "KB_MOUSE_DEFAULT").unwrap();
        // KEY_x drives BOTH ACTIVATE_HANDTOOL and ACTIVATE_HANDTOOL_SECONDARY → shared.
        let key_x = kb.shared.iter().find(|s| s.input == "KEY_x").unwrap();
        assert_eq!(key_x.actions.len(), 2);
        // KEY_space is one action only → not shared.
        assert!(!kb.shared.iter().any(|s| s.input == "KEY_space"));
        // MOUSE_BUTTON_LEFT is one action (ACTIVATE_HANDTOOL) → not shared, even though
        // ACTIVATE_HANDTOOL also binds KEY_x (per-action multi-input isn't a clash).
        assert!(!kb.shared.iter().any(|s| s.input == "MOUSE_BUTTON_LEFT"));
    }

    #[test]
    fn devices_are_isolated() {
        let r = parse(SAMPLE);
        let gp = r.devices.iter().find(|d| d.device == "GAMEPAD").unwrap();
        assert_eq!(gp.bindings.len(), 1);
        assert!(gp.shared.is_empty());
    }

    #[test]
    fn empty_or_garbage_is_safe() {
        assert_eq!(parse("").total_bindings, 0);
        assert_eq!(parse("<inputBinding></inputBinding>").total_bindings, 0);
    }
}
