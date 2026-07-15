//! Conflict detection over an active set of mods — the thing no other FS tool does
//! well. We re-parse each active mod's `modDesc.xml` and flag collisions across the
//! GIANTS namespaces that actually clash at load time:
//!
//! * `<uniqueType>` — GIANTS' own primitive: only one mod of a type loads (critical)
//! * the six specialization/type registration surfaces — same name, same surface,
//!   means one silently wins (critical)
//! * `<extraSourceFiles>` global Lua with the same basename — they can overwrite
//!   each other (warning)
//!
//! Runs on demand for the active set only, so it stays cheap.

use crate::moddesc;
use crate::scan;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictInput {
    pub tech_name: String,
    pub title: Option<String>,
    pub path: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Conflict {
    pub severity: String, // "critical" | "warning"
    pub kind: String,     // "uniqueType" | "specialization" | "vehicleType" | "script" | …
    pub name: String,
    pub explanation: String,
    pub mods: Vec<String>, // display labels of the mods involved
}

struct Parsed {
    label: String,
    md: moddesc::ModDesc,
}

fn kind_label(kind: &str) -> &str {
    match kind {
        "specialization" => "specialization",
        "placeableSpecialization" => "placeable specialization",
        "handToolSpecialization" => "hand-tool specialization",
        "vehicleType" => "vehicle type",
        "placeableType" => "placeable type",
        "handToolType" => "hand-tool type",
        "action" => "input action",
        "brand" => "brand",
        "storeCategory" => "store category",
        other => other,
    }
}

/// Detect conflicts within the given active set.
pub fn detect(inputs: &[ConflictInput]) -> Vec<Conflict> {
    // Parse each active mod's modDesc in parallel.
    let parsed: Vec<Parsed> = inputs
        .par_iter()
        .filter_map(|i| {
            let xml = scan::read_moddesc_xml(std::path::Path::new(&i.path), &i.kind).ok()?;
            Some(Parsed {
                label: i.title.clone().unwrap_or_else(|| i.tech_name.clone()),
                md: moddesc::parse(&xml),
            })
        })
        .collect();

    // Collision maps: key -> set of mod labels.
    let mut registrations: HashMap<(String, String), Vec<String>> = HashMap::new();
    let mut unique_types: HashMap<String, Vec<String>> = HashMap::new();
    let mut scripts: HashMap<String, Vec<String>> = HashMap::new();

    for p in &parsed {
        for r in &p.md.registrations {
            push_unique(
                registrations.entry((r.kind.clone(), r.name.clone())).or_default(),
                &p.label,
            );
        }
        if let Some(ut) = &p.md.unique_type {
            push_unique(unique_types.entry(ut.clone()).or_default(), &p.label);
        }
        for s in &p.md.scripts {
            let base = s.rsplit('/').next().unwrap_or(s).to_string();
            push_unique(scripts.entry(base).or_default(), &p.label);
        }
    }

    let mut out = Vec::new();

    for (ut, mods) in unique_types {
        if mods.len() > 1 {
            out.push(Conflict {
                severity: "critical".into(),
                kind: "uniqueType".into(),
                name: ut.clone(),
                explanation: format!(
                    "These mods share the uniqueType \"{ut}\" — Farming Simulator loads only one mod of a given uniqueType, so the others will not take effect."
                ),
                mods,
            });
        }
    }

    for ((kind, name), mods) in registrations {
        if mods.len() > 1 {
            let kl = kind_label(&kind).to_string();
            out.push(Conflict {
                severity: "critical".into(),
                kind,
                name: name.clone(),
                explanation: format!(
                    "Multiple mods register the {kl} \"{name}\". Only one registration wins at load — the others' behavior may break."
                ),
                mods,
            });
        }
    }

    for (base, mods) in scripts {
        if mods.len() > 1 {
            out.push(Conflict {
                severity: "warning".into(),
                kind: "script".into(),
                name: base.clone(),
                explanation: format!(
                    "These mods each inject a global Lua script named \"{base}\" via extraSourceFiles. They may override one another depending on load order."
                ),
                mods,
            });
        }
    }

    // Critical first, then by name.
    out.sort_by(|a, b| {
        let sev = |s: &str| if s == "critical" { 0 } else { 1 };
        sev(&a.severity)
            .cmp(&sev(&b.severity))
            .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    out
}

fn push_unique(v: &mut Vec<String>, label: &str) {
    if !v.iter().any(|x| x == label) {
        v.push(label.to_string());
    }
}
