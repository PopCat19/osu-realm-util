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
        "ls" => cmd_list_tables(&args),
        "help" | "-h" | "--help" => print_usage(),
        _ => cmd_list_tables(&args),
    }
}

fn print_usage() {
    eprintln!(
        "\
osu-realm-util

Commands:
  ls [REALM]            list tables and columns from client.realm
  col [DB]               list stable collection.db contents
  realm2col [REALM] DB   export lazer BeatmapCollection to new collection.db
  merge [REALM] DB       merge lazer BeatmapCollection into existing collection.db

Environment:
  OSU_REALM_PATH         path to client.realm (default: ~/.local/share/osu/client.realm)
  OSU_COLLECTION_DB      path to stable collection.db (default: ~/Documents/osu!/collection.db)"
    );
}

fn osu_realm_path() -> String {
    std::env::var("OSU_REALM_PATH").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_default();
        format!("{home}/.local/share/osu/client.realm")
    })
}

fn osu_collection_path() -> String {
    std::env::var("OSU_COLLECTION_DB").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_default();
        format!("{home}/Documents/osu!/collection.db")
    })
}

fn cmd_list_tables(args: &[String]) {
    let path_owned;
    let realm_path = match args.get(2) {
        Some(p) => p.as_str(),
        None => {
            path_owned = osu_realm_path();
            &path_owned
        }
    };
    let realm = RealmFile::open(realm_path).expect("failed to open realm file");
    for table in realm.tables() {
        println!(
            "{} ({} rows, {} cols)",
            table.name,
            table.rows.len(),
            table.columns.len()
        );
        if args.len() >= 3 {
            for (name, col_type) in &table.columns {
                println!("  {:30} {:?}", name, col_type);
            }
        }
    }
}

fn cmd_collections(args: &[String]) {
    let default_path;
    let path = if let Some(p) = args.get(2) {
        p.as_str()
    } else {
        default_path = osu_collection_path();
        &default_path
    };
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
    let (realm_path, out_path) = match &args[2..] {
        [] => {
            eprintln!("usage: {} DB", if merge { "merge" } else { "realm2col" });
            return;
        }
        [db] => (osu_realm_path(), db.to_string()),
        [realm, db] => (realm.to_string(), db.to_string()),
        _ => {
            eprintln!("too many arguments");
            return;
        }
    };

    let realm = RealmFile::open(&realm_path).expect("failed to open realm file");
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
            collection::CollectionDb::open(&out_path).unwrap_or(collection::CollectionDb {
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
        collections.sort_by(|a, b| a.name.cmp(&b.name));
    }

    // Sort hashes within each collection for deterministic output
    for c in &mut collections {
        c.beatmap_hashes.sort();
    }

    let db = collection::CollectionDb {
        version: 20250207,
        collections,
    };

    db.save(&out_path).expect("failed to write collection.db");
    let total: usize = db.collections.iter().map(|c| c.beatmap_hashes.len()).sum();
    println!(
        "{} → {} collections, {} maps",
        out_path,
        db.collections.len(),
        total
    );
}
