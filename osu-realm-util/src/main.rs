// main.rs
//
// Purpose: CLI entry for osu-realm-util — dumps collections from both formats.

mod collection;

extern crate realm_codec_rs;
use realm_codec_rs::{RealmFile, Value};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 && args[1] == "col" {
        let db = collection::CollectionDb::open("/home/popcat19/Documents/osu!/collection.db").unwrap();
        println!("collection.db: {} collections, {} maps",
            db.collections.len(),
            db.collections.iter().map(|c| c.beatmap_hashes.len()).sum::<usize>());
        for c in &db.collections {
            println!("  {:30} → {}", c.name, c.beatmap_hashes.len());
        }
        return;
    }

    let realm = RealmFile::open("/home/popcat19/.local/share/osu/client.realm").unwrap();

    if let Some(t) = realm.table("class_BeatmapCollection") {
        println!("class_BeatmapCollection (lazer): {} collections", t.rows.len());
        let name_col = t.columns.iter().position(|(n,_)| n=="Name").unwrap_or(0);
        let hash_col = t.columns.iter().position(|(n,_)| n=="BeatmapMD5Hashes").unwrap_or(2);
        for (ri, row) in t.rows.iter().enumerate() {
            let name = match row.values.get(name_col) {
                Some(Value::String(s)) => s.as_str(),
                _ => "?",
            };
            let hash_count = match row.values.get(hash_col) {
                Some(Value::String(s)) => s.lines().count(),
                _ => 0,
            };
            println!("  {ri:2} {name:40} → {hash_count} maps");
        }
    }

    if let Some(t) = realm.table("class_Beatmap") {
        println!("\nclass_Beatmap: {} rows", t.rows.len());
    }
    if let Some(t) = realm.table("class_BeatmapMetadata") {
        println!("class_BeatmapMetadata: {} rows", t.rows.len());
    }
    if let Some(t) = realm.table("class_BeatmapSet") {
        println!("class_BeatmapSet: {} rows", t.rows.len());
    }
}
