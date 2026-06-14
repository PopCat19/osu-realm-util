// collection.rs
//
// Purpose: Read and write osu!stable collection.db binary files.
//
// This module:
// - Parses the legacy osu! collection database format
// - Handles osu!-string encoding (LEB128 length + UTF-8)
// - Returns structured Collection data
// - Writes collection.db binary output (realm2collectiondb)

#[derive(Debug, Clone)]
pub struct Collection {
    pub name: String,
    pub beatmap_hashes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CollectionDb {
    pub version: i32,
    pub collections: Vec<Collection>,
}

#[derive(Debug)]
pub enum CollectionDbError {
    Io(std::io::Error),
    Truncated(&'static str),
}

impl std::fmt::Display for CollectionDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Truncated(msg) => write!(f, "truncated file: {msg}"),
        }
    }
}

impl std::error::Error for CollectionDbError {}

impl From<std::io::Error> for CollectionDbError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl CollectionDb {
    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Self, CollectionDbError> {
        let data = std::fs::read(path)?;
        Self::parse(&data)
    }

    pub fn save<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), CollectionDbError> {
        std::fs::write(path, self.to_bytes())?;
        Ok(())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.version.to_le_bytes());
        out.extend_from_slice(&(self.collections.len() as i32).to_le_bytes());
        for c in &self.collections {
            write_osu_string(&mut out, &c.name);
            out.extend_from_slice(&(c.beatmap_hashes.len() as i32).to_le_bytes());
            for h in &c.beatmap_hashes {
                write_osu_string(&mut out, h);
            }
        }
        out
    }

    pub fn parse(mut buf: &[u8]) -> Result<Self, CollectionDbError> {
        let version = read_i32(&mut buf).map_err(|_| CollectionDbError::Truncated("version"))?;
        let count = read_i32(&mut buf).map_err(|_| CollectionDbError::Truncated("collection count"))?;
        let mut collections = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let name = read_osu_string(&mut buf)
                .map_err(|_| CollectionDbError::Truncated("collection name"))?;
            let beatmap_count = read_i32(&mut buf)
                .map_err(|_| CollectionDbError::Truncated("beatmap count"))?;
            let mut beatmap_hashes = Vec::with_capacity(beatmap_count as usize);
            for _ in 0..beatmap_count {
                let hash = read_osu_string(&mut buf)
                    .map_err(|_| CollectionDbError::Truncated("beatmap hash"))?;
                beatmap_hashes.push(hash);
            }
            collections.push(Collection { name, beatmap_hashes });
        }
        Ok(Self { version, collections })
    }
}

fn read_i32(buf: &mut &[u8]) -> Result<i32, ()> {
    if buf.len() < 4 {
        return Err(());
    }
    let val = i32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    *buf = &buf[4..];
    Ok(val)
}

fn read_osu_string(buf: &mut &[u8]) -> Result<String, ()> {
    if buf.is_empty() {
        return Err(());
    }
    let tag = buf[0];
    *buf = &buf[1..];
    if tag == 0x00 {
        return Ok(String::new());
    }
    if tag != 0x0b {
        return Err(());
    }
    let len = read_leb128(buf).ok_or(())?;
    if buf.len() < len {
        return Err(());
    }
    let s = String::from_utf8(buf[..len].to_vec()).map_err(|_| ())?;
    *buf = &buf[len..];
    Ok(s)
}

fn write_osu_string(out: &mut Vec<u8>, s: &str) {
    if s.is_empty() {
        out.push(0x00);
        return;
    }
    out.push(0x0b);
    write_leb128(out, s.len());
    out.extend_from_slice(s.as_bytes());
}

fn write_leb128(out: &mut Vec<u8>, mut len: usize) {
    loop {
        let mut byte = (len & 0x7F) as u8;
        len >>= 7;
        if len != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if len == 0 {
            break;
        }
    }
}

fn read_leb128(buf: &mut &[u8]) -> Option<usize> {
    let mut result: usize = 0;
    let mut shift = 0u32;
    loop {
        if buf.is_empty() {
            return None;
        }
        let byte = buf[0];
        *buf = &buf[1..];
        result |= ((byte & 0x7F) as usize) << shift;
        if byte & 0x80 == 0 {
            return Some(result);
        }
        shift += 7;
        if shift >= 35 {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_roundtrip() {
        let data = {
            let mut d = Vec::new();
            d.extend_from_slice(&20250207i32.to_le_bytes());
            d.extend_from_slice(&2i32.to_le_bytes());
            write_osu_string(&mut d, "2024");
            d.extend_from_slice(&2i32.to_le_bytes());
            write_osu_string(&mut d, "deadbeefdeadbeefdeadbeefdeadbeef");
            write_osu_string(&mut d, "cafebabe000000000000000000000000");
            write_osu_string(&mut d, "");
            d.extend_from_slice(&0i32.to_le_bytes());
            d
        };

        let db = CollectionDb::parse(&data).unwrap();
        assert_eq!(db.version, 20250207);
        assert_eq!(db.collections.len(), 2);
        assert_eq!(db.collections[0].name, "2024");
        assert_eq!(db.collections[0].beatmap_hashes.len(), 2);
        assert_eq!(db.collections[1].name, "");

        let reencoded = db.to_bytes();
        let db2 = CollectionDb::parse(&reencoded).unwrap();
        assert_eq!(db2.collections.len(), 2);
        assert_eq!(db2.collections[0].name, "2024");
        assert_eq!(db2.collections[0].beatmap_hashes.len(), 2);
        assert_eq!(db2.collections[1].name, "");
    }

    #[test]
    fn real_file_parse() {
        let db = CollectionDb::open("/home/popcat19/Documents/osu!/collection.db").unwrap();
        assert!(db.version > 0);
        assert!(db.collections.len() > 0);
        for c in &db.collections {
            assert!(!c.name.is_empty() || c.beatmap_hashes.is_empty());
            for h in &c.beatmap_hashes {
                assert_eq!(h.len(), 32);
                assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
            }
        }
    }
}
