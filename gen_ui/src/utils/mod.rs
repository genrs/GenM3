// use std::path::PathBuf;

// use cargo_metadata::MetadataCommand;
use toml_edit::{DocumentMut, InlineTable, Item, Table, Value};

pub fn get_from_doc<U, D, F>(doc: &DocumentMut, key: &str, default: D, f: F) -> U
where
    D: FnOnce() -> U,
    F: FnOnce(&Item) -> U,
{
    doc.get(key).map_or_else(default, f)
}

pub fn get_from_itable<U, D, F>(v: &InlineTable, key: &str, default: D, f: F) -> U
where
    D: FnOnce() -> U,
    F: FnOnce(&Value) -> U,
{
    v.get(key).map_or_else(default, f)
}

pub fn get_from_table<U, D, F>(v: &Table, key: &str, default: D, f: F) -> U
where
    D: FnOnce() -> U,
    F: FnOnce(&Item) -> U,
{
    v.get(key).map_or_else(default, f)
}

// pub fn makepad_resource_dir() -> Option<PathBuf> {
//     let meta = MetadataCommand::new().exec().ok()?;
//     for pkg in meta.packages {
//         if pkg.name.as_str() == "makepad-widgets" {
//             return Some(pkg.manifest_path.parent()?.to_path_buf().into());
//         }
//     }
//     None
// }

pub fn round_2_decimals(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

pub fn round_2_decimals_f32(value: f32) -> f32 {
    (value * 100.0).round() / 100.0
}

/// normalization [0, 1] (归一化), return value in [0, 1]
pub fn normalization(value: f32, min: f32, max: f32) -> f32 {
    if max - min < 0.0 {
        panic!("max must be greater than min");
    }
    // 保证value在[min, max]范围内
    let v = value.clamp(min, max);
    let r2 = (v - min) / (max - min);
    round_2_decimals_f32(r2)
}

/// round value to the nearest step
pub fn round_step(v: f32, step: f32) -> f32 {
    if step == 0.0 {
        return v;
    }
    if v % step == 0.0 {
        return v;
    }
    let up = (v / step).ceil() * step;
    let down = (v / step).floor() * step;
    if (up - v) < (v - down) {
        round_2_decimals_f32(up)
    } else {
        round_2_decimals_f32(down)
    }
}