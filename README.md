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
osu-realm-util

# List stable collection.db
osu-realm-util col [COLLECTION.DB]

# Export lazer collections → new collection.db
osu-realm-util realm2col [CLIENT.REALM] OUT.DB

# Merge lazer collections into existing collection.db
osu-realm-util merge [CLIENT.REALM] EXISTING.DB
```

Default paths: `~/.local/share/osu/client.realm` for Realm,
`~/Documents/osu!/collection.db` for the `col` command.

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
# Dev shell
nix develop

# Build
nix build

# Run
nix run
```

## License

MIT OR Apache-2.0
