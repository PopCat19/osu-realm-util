// main.rs
//
// Purpose: CLI entry point for osu-realm-extracter — opens a Realm database
// and dumps table metadata.
//
// This module:
// - Parses the first positional arg as the path to a .realm file
// - Defaults to ~/.local/share/osu/client.realm
// - Reports table names, row counts, column counts, and garbled-column counts

extern crate realm_codec_rs;
use realm_codec_rs::RealmFile;

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/home/popcat19/.local/share/osu/client.realm".into());

    let data = match std::fs::read(&path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("read err: {e}");
            return;
        }
    };
    println!("File size: {} bytes", data.len());

    let realm = match RealmFile::open(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        }
    };

    for t in realm.tables().iter() {
        let cols_ok = t
            .columns
            .iter()
            .filter(|(n, _)| !n.is_empty() && !n.starts_with('\0') && n.len() < 100)
            .count();
        let cols_bad = t.columns.len() - cols_ok;
        println!(
            "{} | {} rows | {} cols ({} garbled)",
            t.name,
            t.rows.len(),
            t.columns.len(),
            cols_bad
        );
    }
}
