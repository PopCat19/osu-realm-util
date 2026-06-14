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

**Update workflow**: run `merge` to pull lazer collections into your
stable `collection.db`. Preserves your existing collections; same-name
collections take lazer's hashes and append any stable-only hashes.

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
nix run github:PopCat19/osu-realm-util
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
