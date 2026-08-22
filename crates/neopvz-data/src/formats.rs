use std::io::Read;

use flate2::read::ZlibDecoder;
use thiserror::Error;

use crate::MAX_RESOURCE_SIZE;

const COMPILED_COOKIE: u32 = 0xDEADFED4;
const COMPILED_HEADER_SIZE: usize = 8;
const SCHEMA_HASH_SIZE: usize = 4;
const REANIM_DEFAULT_FIELD_PLACEHOLDER: f32 = -10_000.0;

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
        let mut decoder =
            ZlibDecoder::new(&data[COMPILED_HEADER_SIZE..]).take(u64::from(declared) + 1);
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

        let schema_hash = u32::from_le_bytes([decoded[0], decoded[1], decoded[2], decoded[3]]);
        Ok(Self {
            schema_hash,
            payload: decoded.split_off(SCHEMA_HASH_SIZE),
        })
    }

    pub fn reanimation(&self) -> Result<ReanimatorDefinition, ReanimError> {
        ReanimatorDefinition::parse(&self.payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReanimatorDefinition {
    pub fps: f32,
    pub tracks: Vec<ReanimatorTrack>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReanimatorTrack {
    pub name: String,
    pub transforms: Vec<ReanimatorTransform>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReanimatorTransform {
    pub x: f32,
    pub y: f32,
    pub skew_x: f32,
    pub skew_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub frame: f32,
    pub alpha: f32,
    pub image: Option<String>,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ReanimError {
    #[error("reanimation payload ended unexpectedly")]
    UnexpectedEnd,
    #[error("invalid {field} definition size: expected {expected}, got {actual}")]
    InvalidDefinitionSize {
        field: &'static str,
        expected: i32,
        actual: i32,
    },
    #[error("invalid {field} count: {count}")]
    InvalidCount { field: &'static str, count: i32 },
    #[error("invalid {field} string length: {length}")]
    InvalidStringLength { field: &'static str, length: i32 },
    #[error("invalid UTF-8 in {field}")]
    InvalidUtf8 { field: &'static str },
    #[error("non-finite reanimation value in {field}")]
    InvalidFloat { field: &'static str },
    #[error("reanimation payload has {0} trailing bytes")]
    TrailingBytes(usize),
}

impl ReanimatorDefinition {
    // These sizes are the 32-bit structures serialized by Definition.cpp.
    pub fn parse(payload: &[u8]) -> Result<Self, ReanimError> {
        let mut reader = ReanimReader::new(payload);
        let definition = reader.read_bytes(16)?;
        let track_count = read_i32_at(definition, 4)?;
        let fps = f32_at(definition, 8, "fps")?;
        let track_records = reader.read_array("track", 12, track_count)?;
        let mut tracks = Vec::with_capacity(track_records.len());

        for track_record in track_records {
            let name = reader.read_string("track name")?;
            let transform_count = read_i32_at(track_record, 8)?;
            let transform_records = reader.read_array("transform", 44, transform_count)?;
            let mut transforms = Vec::with_capacity(transform_records.len());
            for transform_record in transform_records {
                let image = reader.read_string("image")?;
                let _font = reader.read_string("font")?;
                let _text = reader.read_string("text")?;
                transforms.push(ReanimatorTransform {
                    x: f32_at(transform_record, 0, "x")?,
                    y: f32_at(transform_record, 4, "y")?,
                    skew_x: f32_at(transform_record, 8, "kx")?,
                    skew_y: f32_at(transform_record, 12, "ky")?,
                    scale_x: f32_at(transform_record, 16, "sx")?,
                    scale_y: f32_at(transform_record, 20, "sy")?,
                    frame: f32_at(transform_record, 24, "f")?,
                    alpha: f32_at(transform_record, 28, "a")?,
                    image: (!image.is_empty()).then_some(image),
                });
            }
            fill_reanimation_missing_data(&mut transforms);
            tracks.push(ReanimatorTrack { name, transforms });
        }

        if reader.remaining() != 0 {
            return Err(ReanimError::TrailingBytes(reader.remaining()));
        }
        Ok(Self { fps, tracks })
    }
}

fn fill_reanimation_missing_data(transforms: &mut [ReanimatorTransform]) {
    let mut previous_x = 0.0;
    let mut previous_y = 0.0;
    let mut previous_skew_x = 0.0;
    let mut previous_skew_y = 0.0;
    let mut previous_scale_x = 1.0;
    let mut previous_scale_y = 1.0;
    let mut previous_frame = 0.0;
    let mut previous_alpha = 1.0;
    let mut previous_image = None;

    for transform in transforms {
        fill_reanimation_field(&mut previous_x, &mut transform.x);
        fill_reanimation_field(&mut previous_y, &mut transform.y);
        fill_reanimation_field(&mut previous_skew_x, &mut transform.skew_x);
        fill_reanimation_field(&mut previous_skew_y, &mut transform.skew_y);
        fill_reanimation_field(&mut previous_scale_x, &mut transform.scale_x);
        fill_reanimation_field(&mut previous_scale_y, &mut transform.scale_y);
        fill_reanimation_field(&mut previous_frame, &mut transform.frame);
        fill_reanimation_field(&mut previous_alpha, &mut transform.alpha);
        if transform.image.is_none() {
            transform.image = previous_image.clone();
        } else {
            previous_image = transform.image.clone();
        }
    }
}

fn fill_reanimation_field(previous: &mut f32, value: &mut f32) {
    if *value == REANIM_DEFAULT_FIELD_PLACEHOLDER {
        *value = *previous;
    } else {
        *previous = *value;
    }
}

struct ReanimReader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> ReanimReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.offset)
    }

    fn read_bytes(&mut self, length: usize) -> Result<&'a [u8], ReanimError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(ReanimError::UnexpectedEnd)?;
        let bytes = self
            .bytes
            .get(self.offset..end)
            .ok_or(ReanimError::UnexpectedEnd)?;
        self.offset = end;
        Ok(bytes)
    }

    fn read_i32(&mut self) -> Result<i32, ReanimError> {
        Ok(i32::from_le_bytes(self.read_bytes(4)?.try_into().unwrap()))
    }

    fn read_string(&mut self, field: &'static str) -> Result<String, ReanimError> {
        let length = self.read_i32()?;
        if length < 0 {
            return Err(ReanimError::InvalidStringLength { field, length });
        }
        let bytes = self.read_bytes(usize::try_from(length).unwrap())?;
        String::from_utf8(bytes.to_vec()).map_err(|_| ReanimError::InvalidUtf8 { field })
    }

    fn read_array(
        &mut self,
        field: &'static str,
        definition_size: i32,
        count: i32,
    ) -> Result<Vec<&'a [u8]>, ReanimError> {
        let actual_size = self.read_i32()?;
        if actual_size != definition_size {
            return Err(ReanimError::InvalidDefinitionSize {
                field,
                expected: definition_size,
                actual: actual_size,
            });
        }
        if count < 0 {
            return Err(ReanimError::InvalidCount { field, count });
        }
        let count = usize::try_from(count).unwrap();
        if count > self.remaining() / usize::try_from(definition_size).unwrap() {
            return Err(ReanimError::UnexpectedEnd);
        }
        (0..count)
            .map(|_| self.read_bytes(usize::try_from(definition_size).unwrap()))
            .collect()
    }
}

fn read_i32_at(bytes: &[u8], offset: usize) -> Result<i32, ReanimError> {
    let end = offset.checked_add(4).ok_or(ReanimError::UnexpectedEnd)?;
    bytes
        .get(offset..end)
        .ok_or(ReanimError::UnexpectedEnd)
        .map(|bytes| i32::from_le_bytes(bytes.try_into().unwrap()))
}

fn f32_at(bytes: &[u8], offset: usize, field: &'static str) -> Result<f32, ReanimError> {
    let end = offset.checked_add(4).ok_or(ReanimError::UnexpectedEnd)?;
    let value = bytes
        .get(offset..end)
        .ok_or(ReanimError::UnexpectedEnd)
        .map(|bytes| f32::from_le_bytes(bytes.try_into().unwrap()))?;
    value
        .is_finite()
        .then_some(value)
        .ok_or(ReanimError::InvalidFloat { field })
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
        oversized.extend_from_slice(&u32::try_from(MAX_RESOURCE_SIZE + 1).unwrap().to_le_bytes());
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

    #[test]
    fn decodes_compiled_reanimation_definition() {
        let mut payload = vec![0; 16];
        payload[4..8].copy_from_slice(&1_i32.to_le_bytes());
        payload[8..12].copy_from_slice(&12.0_f32.to_le_bytes());
        payload.extend_from_slice(&12_i32.to_le_bytes());

        let mut track = vec![0; 12];
        track[8..12].copy_from_slice(&1_i32.to_le_bytes());
        payload.extend_from_slice(&track);
        payload.extend_from_slice(&4_i32.to_le_bytes());
        payload.extend_from_slice(b"walk");
        payload.extend_from_slice(&44_i32.to_le_bytes());

        let mut transform = vec![0; 44];
        transform[0..4].copy_from_slice(&3.0_f32.to_le_bytes());
        transform[16..20].copy_from_slice(&0.8_f32.to_le_bytes());
        transform[20..24].copy_from_slice(&0.9_f32.to_le_bytes());
        transform[24..28].copy_from_slice(&1.0_f32.to_le_bytes());
        transform[28..32].copy_from_slice(&1.0_f32.to_le_bytes());
        payload.extend_from_slice(&transform);
        let image = b"IMAGE_REANIM_ZOMBIE_BODY";
        payload.extend_from_slice(&i32::try_from(image.len()).unwrap().to_le_bytes());
        payload.extend_from_slice(image);
        payload.extend_from_slice(&0_i32.to_le_bytes());
        payload.extend_from_slice(&0_i32.to_le_bytes());

        let definition = CompiledDefinition {
            schema_hash: 7,
            payload,
        }
        .reanimation()
        .unwrap();
        assert_eq!(definition.fps, 12.0);
        assert_eq!(definition.tracks[0].name, "walk");
        assert_eq!(definition.tracks[0].transforms[0].x, 3.0);
        assert_eq!(
            definition.tracks[0].transforms[0].image.as_deref(),
            Some("IMAGE_REANIM_ZOMBIE_BODY")
        );
    }

    #[test]
    fn rejects_reanimation_array_size_mismatch() {
        let mut payload = vec![0; 16];
        payload[4..8].copy_from_slice(&0_i32.to_le_bytes());
        payload[8..12].copy_from_slice(&12.0_f32.to_le_bytes());
        payload.extend_from_slice(&11_i32.to_le_bytes());
        assert!(matches!(
            ReanimatorDefinition::parse(&payload),
            Err(ReanimError::InvalidDefinitionSize { field: "track", .. })
        ));
    }

    #[test]
    fn fills_reanimation_placeholders_from_the_previous_transform() {
        let mut transforms = vec![
            ReanimatorTransform {
                x: 12.0,
                y: 4.0,
                skew_x: 2.0,
                skew_y: 3.0,
                scale_x: 0.8,
                scale_y: 0.9,
                frame: 1.0,
                alpha: 0.7,
                image: Some("IMAGE_REANIM_BODY".to_owned()),
            },
            ReanimatorTransform {
                x: REANIM_DEFAULT_FIELD_PLACEHOLDER,
                y: REANIM_DEFAULT_FIELD_PLACEHOLDER,
                skew_x: REANIM_DEFAULT_FIELD_PLACEHOLDER,
                skew_y: REANIM_DEFAULT_FIELD_PLACEHOLDER,
                scale_x: REANIM_DEFAULT_FIELD_PLACEHOLDER,
                scale_y: REANIM_DEFAULT_FIELD_PLACEHOLDER,
                frame: REANIM_DEFAULT_FIELD_PLACEHOLDER,
                alpha: REANIM_DEFAULT_FIELD_PLACEHOLDER,
                image: None,
            },
        ];

        fill_reanimation_missing_data(&mut transforms);
        assert_eq!(transforms[1], transforms[0]);
    }
}
