//! Two-level mod categorization for the organize/archive layout.
//!
//! Primary signal is the **authoritative FS store `<category>`** read from a mod's
//! storeItem XML (e.g. `tractorsM`, `harvesters`, `plows`) — GIANTS' own taxonomy,
//! mapped to a readable (Category, Subcategory). Mods without store items (scripts,
//! textures) fall back to title/tech-name keyword heuristics. Shown READ-ONLY in
//! the UI first so it can be eyeballed / hand-corrected before any files move.

use crate::moddesc::ModDesc;

fn any(hay: &str, needles: &[&str]) -> bool {
    needles.iter().any(|n| hay.contains(n))
}

/// Turn a camelCase FS category into a readable Title-Case label:
/// `productionPoints` -> "Production Points", `OBJECTMISC` -> "Objectmisc".
/// Splits only at true camelCase boundaries (lower→upper), so runs of capitals
/// aren't spaced out letter-by-letter.
fn decamel(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut spaced = String::new();
    for (i, &ch) in chars.iter().enumerate() {
        if i > 0 && ch.is_uppercase() && chars[i - 1].is_lowercase() {
            spaced.push(' ');
        }
        spaced.push(ch);
    }
    spaced
        .split_whitespace()
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

type Cat = (String, Option<String>);

fn cat(top: &str, sub: Option<&str>) -> Cat {
    (top.to_string(), sub.map(|s| s.to_string()))
}

/// Map a *recognized* FS store `<category>` to (Category, Subcategory), or None
/// when unrecognized (the caller then routes by structure + name).
fn map_store_category(raw: &str) -> Option<Cat> {
    let c = raw.to_lowercase();
    let mapped = match c.as_str() {
        "tractorss" => Some(cat("Tractors", Some("Small"))),
        "tractorsm" => Some(cat("Tractors", Some("Medium"))),
        "tractorsl" | "tractorsxl" | "largetractors" => Some(cat("Tractors", Some("Large"))),
        "harvesters" => Some(cat("Harvesters", Some("Combines"))),
        "forageharvesters" => Some(cat("Harvesters", Some("Forage"))),
        "cottonvehicles" | "cottonharvesters" => Some(cat("Harvesters", Some("Cotton"))),
        "beetvehicles" | "sugarbeetharvesters" | "potatovehicles" | "potatoharvesters" => {
            Some(cat("Harvesters", Some("Root Crop")))
        }
        "grapeharvesters" | "grapevehicles" | "oliveshakers" => Some(cat("Harvesters", Some("Fruit"))),
        "cutters" | "cornheaders" | "draperheaders" | "cottonheaders" | "headers" => {
            Some(cat("Implements", Some("Headers")))
        }
        "trailers" | "tippers" => Some(cat("Implements", Some("Trailers"))),
        "augerwagons" | "foragewagons" | "loaderwagons" => {
            Some(cat("Implements", Some("Forage & Auger")))
        }
        "lowloaders" | "animaltransport" => Some(cat("Implements", Some("Transport"))),
        "plows" | "ploughs" => Some(cat("Implements", Some("Plows"))),
        "cultivators" | "powerharrows" | "disc" | "harrows" | "rollers" | "packer"
        | "subsoilers" | "mulchers" | "stonepickers" => Some(cat("Implements", Some("Tillage"))),
        "sowingmachines" | "seeders" | "planters" | "directsowingmachines" | "directseeders" => {
            Some(cat("Implements", Some("Seeders")))
        }
        "sprayers" | "sprayerfillup" | "weeders" => Some(cat("Implements", Some("Sprayers"))),
        "fertilizerspreaders" | "manurespreaders" | "slurrytanks" | "spreaders" => {
            Some(cat("Implements", Some("Spreaders")))
        }
        "mowers" | "tedders" | "windrowers" | "rakes" => Some(cat("Implements", Some("Hay & Forage"))),
        "balers" | "balewrappers" | "baleloaders" => Some(cat("Implements", Some("Balers"))),
        "frontloaders" | "frontloadertools" | "wheelloadertools" | "telehandlertools"
        | "skidsteertools" | "weights" | "frontloaderattachertools" => {
            Some(cat("Implements", Some("Attachments")))
        }
        "wheelloaders" | "telehandlers" | "skidsteers" | "forklifts" => {
            Some(cat("Vehicles", Some("Loaders")))
        }
        "cars" => Some(cat("Cars & Trucks", Some("Cars"))),
        "trucks" | "pickups" => Some(cat("Cars & Trucks", Some("Trucks"))),
        "decoration" | "decorations" => Some(cat("Decorations", None)),
        "fences" => Some(cat("Decorations", Some("Fences"))),
        "productionpoints" | "factories" => Some(cat("Placeables", Some("Production"))),
        "sellingstations" | "sellingpoints" => Some(cat("Placeables", Some("Selling"))),
        "animalpens" | "animals" | "husbandries" => Some(cat("Placeables", Some("Animals"))),
        "silos" | "storages" => Some(cat("Placeables", Some("Storage"))),
        "sheds" | "gardensheds" | "garages" => Some(cat("Placeables", Some("Sheds"))),
        "greenhouses" => Some(cat("Placeables", Some("Greenhouses"))),
        "farmhouses" => Some(cat("Placeables", Some("Farmhouses"))),
        "windturbines" | "solarpanels" | "generators" => Some(cat("Placeables", Some("Power"))),
        "beehives" => Some(cat("Placeables", Some("Bees"))),
        "pallets" | "bigbags" | "palletsmisc" => Some(cat("Objects", Some("Pallets"))),
        "bales" | "palletbaling" => Some(cat("Objects", Some("Bales"))),
        "dieseltanks" | "watertanks" | "fillabletanks" | "barrels" => Some(cat("Objects", Some("Tanks"))),
        "shippingcontainers" => Some(cat("Objects", Some("Containers"))),
        _ => None,
    };
    if mapped.is_some() {
        return mapped;
    }

    // Reliable substring rules (still Some); anything else -> None for the caller.
    if c.contains("tractor") {
        Some(cat("Tractors", None))
    } else if c.contains("harvest") || c.contains("combine") {
        Some(cat("Harvesters", None))
    } else if c.contains("loader") || c.contains("telehandler") {
        Some(cat("Vehicles", Some("Loaders")))
    } else if c.contains("trailer") || c.contains("tipper") {
        Some(cat("Implements", Some("Trailers")))
    } else if any(
        &c,
        &[
            "plow", "plough", "cultivat", "harrow", "seeder", "sow", "planter", "spray", "spread",
            "mower", "baler", "tedder", "windrow", "cutter", "header",
        ],
    ) {
        Some(cat("Implements", None))
    } else if c.contains("truck") {
        Some(cat("Cars & Trucks", Some("Trucks")))
    } else {
        None
    }
}

/// Infer (Category, Subcategory). `store_category` is the authoritative FS store
/// category when the mod has a storeItem; otherwise None.
pub fn categorize(
    md: &ModDesc,
    store_category: Option<&str>,
    tech_name: &str,
    title: Option<&str>,
) -> Cat {
    if md.is_map {
        return cat("Maps", None);
    }
    let has_placeable = md.registrations.iter().any(|r| r.kind == "placeableType");

    if let Some(sc) = store_category {
        if let Some(mapped) = map_store_category(sc) {
            return mapped;
        }
        // Unrecognized store category: route by name + structure; readable sub.
        let low = sc.to_lowercase();
        let sub = decamel(sc);
        return if any(&low, &["pallet", "tank", "barrel", "container", "bigbag", "bag", "fillable", "belt"]) {
            cat("Objects", Some(&sub))
        } else if any(
            &low,
            &["placeable", "point", "station", "shed", "silo", "fence", "house", "pen", "greenhouse", "hive", "generator", "solar", "wind", "panel", "building", "decoration", "light", "sign", "garden", "stable", "barn"],
        ) {
            cat("Placeables", Some(&sub))
        } else if has_placeable {
            cat("Placeables", Some(&sub))
        } else {
            cat("Vehicles", Some(&sub))
        };
    }

    // No store item → scripts / textures / gameplay. Keyword heuristics.
    let hay = format!("{} {}", tech_name, title.unwrap_or("")).to_lowercase();
    let has_spec = md
        .registrations
        .iter()
        .any(|r| r.kind == "specialization" || r.kind.ends_with("Specialization"));

    if any(&hay, &["texture", "skin", "colorpack", "color pack", "appearance", "livery", "decal", "wrap"]) {
        return cat("Textures", None);
    }
    if any(&hay, &["soundpack", "sound pack", "engine sound", "exhaust sound"]) {
        return cat("Sounds", None);
    }
    if any(&hay, &["decor", "sign ", "fence", "flag", "bench", "streetlamp", "billboard", "statue"]) {
        return cat("Decorations", None);
    }
    if has_placeable
        || any(&hay, &["placeable", "building", "shed", "barn", "silo ", "garage", "warehouse", "greenhouse", "stable", "factory", "production", "sellpoint", "farmhouse"])
    {
        return cat("Placeables", None);
    }
    if any(&hay, &["cheat", "moneycheat", "unlimited", "godmode", "freemoney"]) {
        return cat("Cheats", None);
    }
    // Realism sub-types (money / time / speed / …)
    if any(&hay, &["realism", "realistic", "hardcore", "difficulty", "seasons", "economy", "price", "money", "wage", "daylength", "speed"]) {
        let sub = if any(&hay, &["money", "economy", "price", "wage", "cost", "income"]) {
            Some("Money")
        } else if any(&hay, &["time", "daylength", "hour", "clock"]) {
            Some("Time")
        } else if any(&hay, &["speed", "mph", "kmh", "faster"]) {
            Some("Speed")
        } else {
            None
        };
        return cat("Realism", sub);
    }
    if !md.scripts.is_empty() || has_spec {
        return cat("Scripts & Tools", None);
    }
    cat("Other", None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moddesc::{ModDesc, Registration};

    fn md() -> ModDesc {
        ModDesc::default()
    }

    #[test]
    fn maps_first() {
        let mut m = md();
        m.is_map = true;
        assert_eq!(categorize(&m, None, "FS25_X", Some("X")), cat("Maps", None));
    }

    #[test]
    fn store_category_drives_vehicles() {
        assert_eq!(categorize(&md(), Some("tractorsM"), "FS25_Fendt", Some("Fendt 1050")), cat("Tractors", Some("Medium")));
        assert_eq!(categorize(&md(), Some("tractorsL"), "FS25_X", Some("X")), cat("Tractors", Some("Large")));
        assert_eq!(categorize(&md(), Some("harvesters"), "FS25_X", Some("X")), cat("Harvesters", Some("Combines")));
        assert_eq!(categorize(&md(), Some("plows"), "FS25_X", Some("X")), cat("Implements", Some("Plows")));
        assert_eq!(categorize(&md(), Some("cars"), "FS25_X", Some("X")), cat("Cars & Trucks", Some("Cars")));
    }

    #[test]
    fn keyword_fallback_without_store() {
        let mut script = md();
        script.scripts.push("foo.lua".into());
        assert_eq!(categorize(&script, None, "FS25_Helper", Some("Helper")), cat("Scripts & Tools", None));
        assert_eq!(categorize(&md(), None, "FS25_RealMoney", Some("Realistic Money")), cat("Realism", Some("Money")));

        let mut place = md();
        place.registrations.push(Registration { kind: "placeableType".into(), name: "x".into() });
        assert_eq!(categorize(&place, None, "FS25_Barn", Some("Barn")), cat("Placeables", None));
    }
}
