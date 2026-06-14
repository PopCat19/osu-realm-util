// main.rs
//
// Purpose: Dump table column schemas and a few sample rows.

extern crate realm_codec_rs;
use realm_codec_rs::{RealmFile, Value};

fn main() {
    let realm = RealmFile::open("/home/popcat19/.local/share/osu/client.realm").unwrap();
    for t in realm.tables().iter().filter(|t| !t.rows.is_empty()) {
        println!("=== {}  ({} rows, {} cols) ===", t.name, t.rows.len(), t.columns.len());
        for (ci, (cn, ct)) in t.columns.iter().enumerate() {
            print!("  [{ci:2}] {cn:30} {ct:?}");

            // Show first 3 non-null values for this column
            let mut samples: Vec<&str> = Vec::new();
            for row in t.rows.iter().take(5) {
                if samples.len() >= 3 { break; }
                if let Some(v) = row.values.get(ci) {
                    let s = match v {
                        Value::String(s) => { let t: String = s.chars().take(50).collect(); if s.len() > 50 { format!("\"{t}…\"") } else { format!("\"{t}\"") } }
                        Value::Int(i) => format!("{i}"),
                        Value::Float(f) => format!("{f:.4}"),
                        Value::Bool(b) => format!("{b}"),
                        Value::Timestamp(ts) => format!("{ts}"),
                        Value::Link(l) => format!("link:{l:#x}"),
                        Value::Null => continue,
                        _ => continue,
                    };
                    if !samples.contains(&s.as_str()) { samples.push(Box::leak(s.into_boxed_str())); }
                }
            }
            if !samples.is_empty() {
                print!("  eg: {}", samples.join(", "));
            }
            println!();
        }
        println!();
    }
}
