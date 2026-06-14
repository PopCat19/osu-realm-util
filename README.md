# osu-realm-util

Utility for reading osu! Realm databases (lazer `client.realm`) and legacy osu!stable `collection.db` files.

## Features

- Parse Realm v24 binary databases into tables, columns, and typed rows
- Read osu!stable `collection.db` collections and beatmap hashes
- Write Realm v9 databases from scratch (`realm-codec` writer)
- Nix flake: devshell + checks (clippy, rustfmt, nixfmt)

## Usage

```
# Lazer client.realm (table listing)
cargo run --release

# osu!stable collection.db
cargo run -- col
```

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
