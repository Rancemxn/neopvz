use std::io::Read;

use flate2::read::ZlibDecoder;
use thiserror::Error;

use crate::MAX_RESOURCE_SIZE;

const COMPILED_COOKIE: u32 = 0xDEADFED4;
const COMPILED_HEADER_SIZE: usize = 8;
const SCHEMA_HASH_SIZE: usize = 4;

#[derive(Debug, Error)]
pub enum CompiledError {
    #[error("compiled definition is smaller than its header")]
    TooSmall,
    #[error("invalid compiled definition cookie: {0:#010x}")]
    InvalidCookie(u32),
    #[error("compiled definition declares an oversized payload: {0}")]
    TooLarge(u32),
    #[error("failed to decompress compiled definition: {0}")]
    Decompress(#[from] std::io::Error),
    #[error("compiled definition size mismatch: declared {declared}, decoded {actual}")]
    SizeMismatch { declared: u32, actual: usize },
    #[error("compiled definition has no schema hash")]
    MissingSchema,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompiledDefinition {
    pub schema_hash: u32,
    pub payload: Vec<u8>,
}

impl CompiledDefinition {
    pub fn decode(data: &[u8]) -> Result<Self, CompiledError> {
        if data.len() < COMPILED_HEADER_SIZE {
            return Err(CompiledError::TooSmall);
        }

        let cookie = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        if cookie != COMPILED_COOKIE {
            return Err(CompiledError::InvalidCookie(cookie));
        }

        let declared = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        if u64::from(declared) > MAX_RESOURCE_SIZE {
            return Err(CompiledError::TooLarge(declared));
        }

        let mut decoded = Vec::new();
        let mut decoder = ZlibDecoder::new(&data[COMPILED_HEADER_SIZE..])
            .take(u64::from(declared) + 1);
        decoder.read_to_end(&mut decoded)?;
        if decoded.len() != declared as usize {
            return Err(CompiledError::SizeMismatch {
                declared,
                actual: decoded.len(),
            });
        }
        if decoded.len() < SCHEMA_HASH_SIZE {
            return Err(CompiledError::MissingSchema);
        }

        let schema_hash =
            u32::from_le_bytes([decoded[0], decoded[1], decoded[2], decoded[3]]);
        Ok(Self {
            schema_hash,
            payload: decoded.split_off(SCHEMA_HASH_SIZE),
        })
    }
}

#[derive(Debug, Error)]
pub enum MusicError {
    #[error("MO3 resource is smaller than its header")]
    TooSmall,
    #[error("invalid MO3 signature")]
    InvalidSignature,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mo3Resource {
    pub version: u8,
    bytes: Vec<u8>,
}

impl Mo3Resource {
    pub fn parse(bytes: Vec<u8>) -> Result<Self, MusicError> {
        if bytes.len() < 4 {
            return Err(MusicError::TooSmall);
        }
        if &bytes[..3] != b"MO3" {
            return Err(MusicError::InvalidSignature);
        }
        Ok(Self {
            version: bytes[3],
            bytes,
        })
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use flate2::{Compression, write::ZlibEncoder};

    use super::*;

    fn compiled_fixture(schema_hash: u32, payload: &[u8]) -> Vec<u8> {
        let mut decoded = schema_hash.to_le_bytes().to_vec();
        decoded.extend_from_slice(payload);
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&decoded).unwrap();
        let compressed = encoder.finish().unwrap();

        let mut fixture = COMPILED_COOKIE.to_le_bytes().to_vec();
        fixture.extend_from_slice(&u32::try_from(decoded.len()).unwrap().to_le_bytes());
        fixture.extend_from_slice(&compressed);
        fixture
    }

    #[test]
    fn decodes_compiled_definition() {
        let definition = CompiledDefinition::decode(&compiled_fixture(7, b"payload")).unwrap();
        assert_eq!(definition.schema_hash, 7);
        assert_eq!(definition.payload, b"payload");
    }

    #[test]
    fn rejects_invalid_compiled_definitions() {
        assert!(matches!(
            CompiledDefinition::decode(&[]),
            Err(CompiledError::TooSmall)
        ));

        let mut invalid_cookie = compiled_fixture(7, b"payload");
        invalid_cookie[0] ^= 1;
        assert!(matches!(
            CompiledDefinition::decode(&invalid_cookie),
            Err(CompiledError::InvalidCookie(_))
        ));

        let mut wrong_size = compiled_fixture(7, b"payload");
        wrong_size[4..8].copy_from_slice(&99_u32.to_le_bytes());
        assert!(matches!(
            CompiledDefinition::decode(&wrong_size),
            Err(CompiledError::SizeMismatch { .. })
        ));

        let mut oversized = COMPILED_COOKIE.to_le_bytes().to_vec();
        oversized.extend_from_slice(
            &u32::try_from(MAX_RESOURCE_SIZE + 1)
                .unwrap()
                .to_le_bytes(),
        );
        assert!(matches!(
            CompiledDefinition::decode(&oversized),
            Err(CompiledError::TooLarge(_))
        ));
    }

    #[test]
    fn validates_mo3_header() {
        let resource = Mo3Resource::parse(b"MO3\x01music".to_vec()).unwrap();
        assert_eq!(resource.version, 1);
        assert_eq!(resource.bytes(), b"MO3\x01music");
        assert!(matches!(
            Mo3Resource::parse(b"OggS".to_vec()),
            Err(MusicError::InvalidSignature)
        ));
    }
}
