// main.rs
//
// Purpose: Dump table column schemas and a few sample rows.

mod collection;

fn main() {
    let db = collection::CollectionDb::open("/home/popcat19/Documents/osu!/collection.db").unwrap();
    println!("{} collections (version {})", db.collections.len(), db.version);
    for c in &db.collections {
        println!("  {} → {} beatmaps", c.name, c.beatmap_hashes.len());
    }
}
