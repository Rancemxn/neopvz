use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use thiserror::Error;

const PAK_MAGIC: u32 = 0xBAC04AC0;
const PAK_VERSION: u32 = 0;
const XOR_KEY: u8 = 0xF7;
const END_FLAG: u8 = 0x80;

#[derive(Debug, Error)]
pub enum PakError {
    #[error("failed to read PAK archive: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid PAK magic: {0:#010x}")]
    InvalidMagic(u32),
    #[error("unsupported PAK version: {0}")]
    UnsupportedVersion(u32),
    #[error("PAK entry name is not UTF-8: {0}")]
    InvalidName(#[from] std::string::FromUtf8Error),
    #[error("duplicate PAK entry: {0}")]
    DuplicateEntry(String),
    #[error("PAK data size overflow")]
    SizeOverflow,
    #[error("PAK index points beyond the archive")]
    TruncatedArchive,
    #[error("PAK entry not found: {0}")]
    MissingEntry(String),
    #[error("PAK entry is too large for this platform: {0}")]
    EntryTooLarge(u64),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PakEntry {
    pub path: String,
    pub offset: u64,
    pub size: u64,
    pub modified: i64,
}

#[derive(Debug)]
pub struct PakArchive {
    path: PathBuf,
    entries: BTreeMap<String, PakEntry>,
}

impl PakArchive {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, PakError> {
        let path = path.as_ref().to_path_buf();
        let mut reader = BufReader::new(File::open(&path)?);

        let magic = read_u32(&mut reader)?;
        if magic != PAK_MAGIC {
            return Err(PakError::InvalidMagic(magic));
        }

        let version = read_u32(&mut reader)?;
        if version != PAK_VERSION {
            return Err(PakError::UnsupportedVersion(version));
        }

        let mut pending = Vec::new();
        loop {
            let flags = read_u8(&mut reader)?;
            if flags & END_FLAG != 0 {
                break;
            }

            let name_len = usize::from(read_u8(&mut reader)?);
            let mut name = vec![0; name_len];
            read_decoded(&mut reader, &mut name)?;
            let name = normalize_pak_path(&String::from_utf8(name)?);
            let size = u64::from(read_u32(&mut reader)?);
            let modified = read_i64(&mut reader)?;
            pending.push((name, size, modified));
        }

        let mut offset = reader.stream_position()?;
        let archive_len = reader.get_ref().metadata()?.len();
        let mut entries = BTreeMap::new();
        for (path, size, modified) in pending {
            let end = offset.checked_add(size).ok_or(PakError::SizeOverflow)?;
            if end > archive_len {
                return Err(PakError::TruncatedArchive);
            }
            let entry = PakEntry {
                path: path.clone(),
                offset,
                size,
                modified,
            };
            if entries.insert(path.clone(), entry).is_some() {
                return Err(PakError::DuplicateEntry(path));
            }
            offset = end;
        }

        Ok(Self { path, entries })
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> impl Iterator<Item = &PakEntry> {
        self.entries.values()
    }

    pub fn read(&self, path: &str) -> Result<Vec<u8>, PakError> {
        let key = normalize_pak_path(path);
        let entry = self
            .entries
            .get(&key)
            .ok_or_else(|| PakError::MissingEntry(path.to_owned()))?;
        let size = usize::try_from(entry.size).map_err(|_| PakError::EntryTooLarge(entry.size))?;
        let mut file = File::open(&self.path)?;
        file.seek(SeekFrom::Start(entry.offset))?;
        let mut data = vec![0; size];
        read_decoded(&mut file, &mut data)?;
        Ok(data)
    }
}

fn normalize_pak_path(path: &str) -> String {
    path.replace('\\', "/")
        .trim_start_matches("./")
        .to_ascii_uppercase()
}

fn read_decoded(reader: &mut impl Read, buffer: &mut [u8]) -> Result<(), std::io::Error> {
    reader.read_exact(buffer)?;
    for byte in buffer {
        *byte ^= XOR_KEY;
    }
    Ok(())
}

fn read_u8(reader: &mut impl Read) -> Result<u8, std::io::Error> {
    let mut value = [0];
    read_decoded(reader, &mut value)?;
    Ok(value[0])
}

fn read_u32(reader: &mut impl Read) -> Result<u32, std::io::Error> {
    let mut value = [0; 4];
    read_decoded(reader, &mut value)?;
    Ok(u32::from_le_bytes(value))
}

fn read_i64(reader: &mut impl Read) -> Result<i64, std::io::Error> {
    let mut value = [0; 8];
    read_decoded(reader, &mut value)?;
    Ok(i64::from_le_bytes(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn fixture(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&PAK_MAGIC.to_le_bytes());
        data.extend_from_slice(&PAK_VERSION.to_le_bytes());
        for (name, contents) in entries {
            data.push(0);
            data.push(u8::try_from(name.len()).unwrap());
            data.extend_from_slice(name.as_bytes());
            data.extend_from_slice(&u32::try_from(contents.len()).unwrap().to_le_bytes());
            data.extend_from_slice(&0_i64.to_le_bytes());
        }
        data.push(END_FLAG);
        for (_, contents) in entries {
            data.extend_from_slice(contents);
        }
        for byte in &mut data {
            *byte ^= XOR_KEY;
        }
        data
    }

    #[test]
    fn reads_entries_by_normalized_path() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("main.pak");
        fs::write(
            &path,
            fixture(&[
                ("properties\\resources.xml", b"<ResourceManifest />"),
                ("sounds\\click.ogg", b"sound"),
            ]),
        )
        .unwrap();

        let archive = PakArchive::load(path).unwrap();
        assert_eq!(archive.entry_count(), 2);
        assert_eq!(
            archive.read("properties/resources.xml").unwrap(),
            b"<ResourceManifest />"
        );
        assert_eq!(archive.read("SOUNDS/CLICK.OGG").unwrap(), b"sound");
    }

    #[test]
    fn rejects_invalid_magic() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("invalid.pak");
        let mut data = fixture(&[]);
        data[0] ^= 1;
        fs::write(&path, data).unwrap();

        let error = PakArchive::load(path).unwrap_err();
        assert!(matches!(error, PakError::InvalidMagic(_)));
    }
}
