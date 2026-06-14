// main.rs
//
// Purpose: CLI for osu-realm-util — lazer Realm + stable collection.db tooling.

mod collection;

extern crate realm_codec_rs;
use realm_codec_rs::{RealmFile, Value};
use std::collections::BTreeSet;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "col" => cmd_collections(&args),
        "realm2col" => cmd_realm_to_collection_db(&args, false),
        "merge" => cmd_realm_to_collection_db(&args, true),
        _ => print_usage(),
    }
}

fn print_usage() {
    eprintln!(
        "\
osu-realm-util

Commands:
  col [DB]             list stable collection.db contents
  realm2col [REALM] DB  export lazer BeatmapCollection → new collection.db
  merge [REALM] DB      merge lazer BeatmapCollection into existing collection.db"
    );
}

fn cmd_collections(args: &[String]) {
    let path = args
        .get(2)
        .map(|s| s.as_str())
        .unwrap_or("/home/popcat19/Documents/osu!/collection.db");
    let db = collection::CollectionDb::open(path).expect("failed to open collection.db");
    let total: usize = db.collections.iter().map(|c| c.beatmap_hashes.len()).sum();
    println!(
        "{}: {} collections, {} maps",
        path,
        db.collections.len(),
        total
    );
    for c in &db.collections {
        println!("  {:30} → {}", c.name, c.beatmap_hashes.len());
    }
}

fn cmd_realm_to_collection_db(args: &[String], merge: bool) {
    // Arg layout: cmd [REALM] DB
    let (realm_path, out_path) = match &args[2..] {
        [] => {
            eprintln!("usage: {} DB", if merge { "merge" } else { "realm2col" });
            return;
        }
        [db] => ("/home/popcat19/.local/share/osu/client.realm", db.as_str()),
        [realm, db] => (realm.as_str(), db.as_str()),
        _ => {
            eprintln!("too many arguments");
            return;
        }
    };

    let realm = RealmFile::open(realm_path).expect("failed to open realm file");
    let tbl = realm
        .table("class_BeatmapCollection")
        .expect("BeatmapCollection table not found");

    let name_col = tbl
        .columns
        .iter()
        .position(|(n, _)| n == "Name")
        .unwrap_or(0);
    let hash_col = tbl
        .columns
        .iter()
        .position(|(n, _)| n == "BeatmapMD5Hashes")
        .unwrap_or(2);

    let mut collections: Vec<collection::Collection> = tbl
        .rows
        .iter()
        .filter_map(|row| {
            let name = match row.values.get(name_col)? {
                Value::String(s) if !s.is_empty() => s.clone(),
                _ => return None,
            };
            let hashes: Vec<String> = match row.values.get(hash_col)? {
                Value::String(s) => s.lines().map(|l| l.to_owned()).collect(),
                _ => vec![],
            };
            Some(collection::Collection {
                name,
                beatmap_hashes: hashes,
            })
        })
        .collect();

    if merge {
        let existing =
            collection::CollectionDb::open(out_path).unwrap_or(collection::CollectionDb {
                version: 20250207,
                collections: vec![],
            });

        for ec in &existing.collections {
            if let Some(rc) = collections.iter_mut().find(|c| c.name == ec.name) {
                let lazer_hashes: BTreeSet<&str> =
                    rc.beatmap_hashes.iter().map(|s| s.as_str()).collect();
                let mut extra: Vec<String> = ec
                    .beatmap_hashes
                    .iter()
                    .filter(|h| !lazer_hashes.contains(h.as_str()))
                    .cloned()
                    .collect();
                rc.beatmap_hashes.append(&mut extra);
            } else {
                collections.push(ec.clone());
            }
        }
        // Realm-originated collections already won the merge for same-name entries.
        // Sort for deterministic output.
        collections.sort_by(|a, b| a.name.cmp(&b.name));
    }

    let db = collection::CollectionDb {
        version: 20250207,
        collections,
    };

    db.save(out_path).expect("failed to write collection.db");
    let total: usize = db.collections.iter().map(|c| c.beatmap_hashes.len()).sum();
    println!(
        "{} → {} collections, {} maps",
        out_path,
        db.collections.len(),
        total
    );
}
