# osu-realm-util

Utility for reading osu! Realm databases (lazer `client.realm`) and legacy osu!stable `collection.db` files.

## Features

- Parse Realm v24 binary databases into tables, columns, and typed rows
- Read osu!stable `collection.db` collections, write and merge them
- Export lazer `BeatmapCollection` table to stable `collection.db` format
- Write Realm v9 databases from scratch (`realm-codec` writer)
- Nix flake: devshell + checks (clippy, rustfmt, nixfmt)

## Usage

```
# List lazer client.realm tables
osu-realm-util          # same as 'ls'
osu-realm-util ls [REALM]

# List stable collection.db
osu-realm-util col [DB]
osu-realm-util col --json [DB]   # machine-readable JSON output

# Export lazer collections → new collection.db
osu-realm-util realm2col [CLIENT.REALM] OUT.DB

# Merge lazer collections into existing collection.db
osu-realm-util merge [CLIENT.REALM] EXISTING.DB
```

All commands accept `--help` / `-h` / `help` for usage details.

Default paths use `$HOME`-based resolution. Override with env vars:
- `OSU_REALM_PATH` — path to `client.realm`
- `OSU_COLLECTION_DB` — path to stable `collection.db`

**Update workflow**: run `merge` to pull lazer collections into your
stable `collection.db`. Preserves your existing collections; same-name
collections take lazer's hashes and append any stable-only hashes.

**Important**: This tool is read-only with respect to `client.realm`.
It never writes, mutates, or backs up the Realm database file.
Only `collection.db` is written (by `realm2col` and `merge`).

## Data quality

osu!lazer v2026.102 stores 16 tables in `client.realm` (v24 format).
`osu-realm-util` decodes:

| Table | Rows | Status |
|-------|------|--------|
| Beatmap | 9,546 | 15/23 cols |
| BeatmapCollection | 27 | ✅ 4/4 cols |
| BeatmapDifficulty | 9,546 | 5/6 cols |
| BeatmapMetadata | 9,546 | 9/11 cols |
| BeatmapSet | 2,145 | 8/11 cols |
| BeatmapUserSettings | 40 | 1/1 cols |
| File | 2 | 1/1 cols |
| KeyBinding | 513 | 3/5 cols |
| ModPreset | 1 | 4/6 cols |
| RealmNamedFileUsage | 2 | 2/2 cols |
| RealmUser | 10,408 | 3/3 cols |
| Ruleset | 9,546 | 5/6 cols |
| RulesetSetting | 17 | 4/4 cols |
| Score | 862 | 25/27 cols |
| Skin | 9 | 6/8 cols |

Known limitations:
- UUID PK columns decode as `Int(0)` (2-slot 128-bit format not yet implemented)
- Embedded objects (BeatmapDifficulty struct, RealmUser inline fields)
  are read from leaf clusters but some parent-table columns remain empty
- Link columns show internal integer refs, not resolved table row IDs
- List-of-int columns (e.g. Score.Pauses) decode as Null
- Timestamp columns read seconds only; nanosecond slot not decoded
- Parse time: ~12s on 26 MB `client.realm` (single-threaded, debug build faster)

## Structure

```
osu-realm-util/          monorepo root
├── realm-codec/         shared kernel — Realm v9/v24 binary parser + writer
│   └── src/
│       ├── format.rs    node header parsing, bit-width element extraction
│       ├── lib.rs       public API: RealmFile, RealmTable, Row, Value, errors
│       ├── reader.rs    table tree traversal, B+ tree, string leaf decoding
│       └── write.rs     Realm v9 binary serialization builder
└── osu-realm-util/      bounded context — CLI application
    └── src/
        ├── main.rs      CLI entry: dumps tables, collections
        └── collection.rs  anti-corruption layer — parses legacy collection.db
```

## Nix

```nix
# Run without installing
nix run github:PopCat19/osu-realm-util -- ls
nix run github:PopCat19/osu-realm-util -- col
nix run github:PopCat19/osu-realm-util -- realm2col /tmp/out.db
nix run github:PopCat19/osu-realm-util -- merge ~/Documents/osu!/collection.db

# Install to profile
nix profile install github:PopCat19/osu-realm-util
nix profile upgrade osu-realm-util      # pull latest commit

# Pin to a specific commit (reproducible)
nix profile install github:PopCat19/osu-realm-util/0fa5d52

# Flake input in another flake
inputs.osu-realm-util.url = "github:PopCat19/osu-realm-util";

# Dev shell
nix develop

# Build locally
nix build
```

## License

MIT
