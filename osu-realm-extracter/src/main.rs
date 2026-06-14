// main.rs
//
// Purpose: CLI entry point for osu-realm-extracter — opens a Realm database
// and dumps table metadata.
//
// This module:
// - Parses the first positional arg as the path to a .realm file
// - Defaults to ~/.local/share/osu/client.realm
// - Reports table names, row counts, column names, and health status

extern crate realm_codec_rs;
use realm_codec_rs::RealmFile;

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/home/popcat19/.local/share/osu/client.realm".into());

    let realm = match RealmFile::open(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        }
    };

    for t in realm.tables().iter() {
        let cols_bad = t
            .columns
            .iter()
            .filter(|(n, _)| n.is_empty() || n.starts_with('\0') || n.len() >= 100)
            .count();
        let status = if cols_bad > 0 {
            "\u{26a0}  garbled cols"
        } else if t.rows.is_empty()
            && !t.columns.is_empty()
            && t.name != "dotnet_guid_representation_fixed"
        {
            "\u{2717}  empty"
        } else {
            ""
        };
        println!(
            "  {:<36} {:>5} rows  {:>2} cols  {}",
            t.name,
            t.rows.len(),
            t.columns.len(),
            status
        );
    }
}
