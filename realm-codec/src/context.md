# realm-codec/src

- `format.rs` — Purpose: Realm v9/v24 binary format constants, node header parsing, and bit-level element extraction
- `lib.rs` — Purpose: Public API for parsing and writing Realm binary database files
- `reader.rs` — Purpose: High-level Realm file reader that parses Groups into Tables with Rows of columnar Values
- `write.rs` — Purpose: Write support: build Realm v9 binary files from scratch
