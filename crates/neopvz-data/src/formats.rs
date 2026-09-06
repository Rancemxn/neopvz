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

    pub fn particles(&self) -> Result<ParticleDefinition, ReanimError> {
        ParticleDefinition::parse(&self.payload)
    }

    pub fn trail(&self) -> Result<TrailDefinition, ReanimError> {
        TrailDefinition::parse(&self.payload)
    }
}

/// Serialized 32-bit size of `TrailDefinition`.
const TRAIL_DEFINITION_SIZE: usize = 0x38;

/// A source `TrailDefinition` from a `*.trail.compiled` resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TrailDefinition {
    pub image: Option<String>,
    pub max_points: i32,
    pub min_point_distance: f32,
    pub trail_flags: i32,
    pub trail_duration: ParameterTrack,
    pub width_over_length: ParameterTrack,
    pub width_over_time: ParameterTrack,
    pub alpha_over_length: ParameterTrack,
    pub alpha_over_time: ParameterTrack,
}

impl TrailDefinition {
    /// Decodes the `gTrailDefMap` cache layout, whose trailing records follow
    /// `gTrailDefFields` declaration order rather than struct offset order.
    pub fn parse(payload: &[u8]) -> Result<Self, ReanimError> {
        let mut reader = ReanimReader::new(payload);
        let record = reader.read_bytes(TRAIL_DEFINITION_SIZE)?;
        let image = reader.read_string("trail image")?;
        let mut trail = Self {
            image: (!image.is_empty()).then_some(image),
            max_points: read_i32_at(record, 0x4)?,
            min_point_distance: f32_at(record, 0x8, "MinPointDistance")?,
            trail_flags: read_i32_at(record, 0xC)?,
            ..Self::default()
        };

        for (offset, slot) in [
            (0x18, &mut trail.width_over_length),
            (0x20, &mut trail.width_over_time),
            (0x28, &mut trail.alpha_over_length),
            (0x30, &mut trail.alpha_over_time),
            (0x10, &mut trail.trail_duration),
        ] {
            *slot = read_track(&mut reader, record, offset)?;
        }

        if reader.remaining() != 0 {
            return Err(ReanimError::TrailingBytes(reader.remaining()));
        }
        Ok(trail)
    }
}

/// Serialized 32-bit size of `TodEmitterDefinition`.
const EMITTER_DEFINITION_SIZE: i32 = 0x164;
/// Serialized 32-bit size of `ParticleField`.
const PARTICLE_FIELD_SIZE: i32 = 0x14;
/// Serialized 32-bit size of `FloatParameterTrackNode`.
const TRACK_NODE_SIZE: usize = 20;

/// One stage of a source `FloatParameterTrack`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrackNode {
    pub time: f32,
    pub low: f32,
    pub high: f32,
    pub curve: i32,
    pub distribution: i32,
}

/// A source `FloatParameterTrack`: a parameter's value range over time.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParameterTrack {
    pub nodes: Vec<TrackNode>,
}

impl ParameterTrack {
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

/// A source `ParticleField`: one physical influence on emitted particles.
#[derive(Clone, Debug, PartialEq)]
pub struct ParticleField {
    pub field_type: i32,
    pub x: ParameterTrack,
    pub y: ParameterTrack,
}

/// A source `TodEmitterDefinition`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EmitterDefinition {
    pub image: Option<String>,
    pub image_row: i32,
    pub image_col: i32,
    pub image_frames: i32,
    pub animated: i32,
    pub particle_flags: i32,
    pub emitter_type: i32,
    pub name: String,
    pub on_duration: String,
    pub system_duration: ParameterTrack,
    pub cross_fade_duration: ParameterTrack,
    pub spawn_rate: ParameterTrack,
    pub spawn_min_active: ParameterTrack,
    pub spawn_max_active: ParameterTrack,
    pub spawn_max_launched: ParameterTrack,
    pub emitter_radius: ParameterTrack,
    pub emitter_offset_x: ParameterTrack,
    pub emitter_offset_y: ParameterTrack,
    pub emitter_box_x: ParameterTrack,
    pub emitter_box_y: ParameterTrack,
    pub emitter_path: ParameterTrack,
    pub emitter_skew_x: ParameterTrack,
    pub emitter_skew_y: ParameterTrack,
    pub particle_duration: ParameterTrack,
    pub system_red: ParameterTrack,
    pub system_green: ParameterTrack,
    pub system_blue: ParameterTrack,
    pub system_alpha: ParameterTrack,
    pub system_brightness: ParameterTrack,
    pub launch_speed: ParameterTrack,
    pub launch_angle: ParameterTrack,
    pub particle_fields: Vec<ParticleField>,
    pub system_fields: Vec<ParticleField>,
    pub particle_red: ParameterTrack,
    pub particle_green: ParameterTrack,
    pub particle_blue: ParameterTrack,
    pub particle_alpha: ParameterTrack,
    pub particle_brightness: ParameterTrack,
    pub particle_spin_angle: ParameterTrack,
    pub particle_spin_speed: ParameterTrack,
    pub particle_scale: ParameterTrack,
    pub particle_stretch: ParameterTrack,
    pub collision_reflect: ParameterTrack,
    pub collision_spin: ParameterTrack,
    pub clip_top: ParameterTrack,
    pub clip_bottom: ParameterTrack,
    pub clip_left: ParameterTrack,
    pub clip_right: ParameterTrack,
    pub animation_rate: ParameterTrack,
}

/// A source `TodParticleDefinition`: the emitter set of one particle system.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParticleDefinition {
    pub emitters: Vec<EmitterDefinition>,
}

impl ParticleDefinition {
    /// Decodes the `gParticleDefMap` cache layout written by `Definition.cpp`.
    ///
    /// `DefMapReadFromCache` walks fields in declaration order, not in struct
    /// offset order, so the trailing records are read in that same order here.
    pub fn parse(payload: &[u8]) -> Result<Self, ReanimError> {
        let mut reader = ReanimReader::new(payload);
        let definition = reader.read_bytes(8)?;
        let emitter_count = read_i32_at(definition, 4)?;
        let emitter_records =
            reader.read_array("emitter", EMITTER_DEFINITION_SIZE, emitter_count)?;

        let mut emitters = Vec::with_capacity(emitter_records.len());
        for record in emitter_records {
            emitters.push(read_emitter(&mut reader, record)?);
        }

        if reader.remaining() != 0 {
            return Err(ReanimError::TrailingBytes(reader.remaining()));
        }
        Ok(Self { emitters })
    }
}

fn read_emitter(
    reader: &mut ReanimReader<'_>,
    record: &[u8],
) -> Result<EmitterDefinition, ReanimError> {
    let image = reader.read_string("emitter image")?;
    let mut emitter = EmitterDefinition {
        image: (!image.is_empty()).then_some(image),
        image_row: read_i32_at(record, 0x8)?,
        image_col: read_i32_at(record, 0x4)?,
        image_frames: read_i32_at(record, 0xC)?,
        animated: read_i32_at(record, 0x10)?,
        particle_flags: read_i32_at(record, 0x14)?,
        emitter_type: read_i32_at(record, 0x18)?,
        name: reader.read_string("emitter name")?,
        ..EmitterDefinition::default()
    };
    emitter.system_duration = read_track(reader, record, 0x24)?;
    emitter.on_duration = reader.read_string("emitter on duration")?;

    for (offset, slot) in [
        (0x2C, &mut emitter.cross_fade_duration),
        (0x34, &mut emitter.spawn_rate),
        (0x3C, &mut emitter.spawn_min_active),
        (0x44, &mut emitter.spawn_max_active),
        (0x4C, &mut emitter.spawn_max_launched),
        (0x54, &mut emitter.emitter_radius),
        (0x5C, &mut emitter.emitter_offset_x),
        (0x64, &mut emitter.emitter_offset_y),
        (0x6C, &mut emitter.emitter_box_x),
        (0x74, &mut emitter.emitter_box_y),
        (0x8C, &mut emitter.emitter_path),
        (0x7C, &mut emitter.emitter_skew_x),
        (0x84, &mut emitter.emitter_skew_y),
        (0x94, &mut emitter.particle_duration),
        (0xAC, &mut emitter.system_red),
        (0xB4, &mut emitter.system_green),
        (0xBC, &mut emitter.system_blue),
        (0xC4, &mut emitter.system_alpha),
        (0xCC, &mut emitter.system_brightness),
        (0x9C, &mut emitter.launch_speed),
        (0xA4, &mut emitter.launch_angle),
    ] {
        *slot = read_track(reader, record, offset)?;
    }

    emitter.particle_fields = read_particle_fields(reader, record, 0xD4)?;
    emitter.system_fields = read_particle_fields(reader, record, 0xDC)?;

    for (offset, slot) in [
        (0xE4, &mut emitter.particle_red),
        (0xEC, &mut emitter.particle_green),
        (0xF4, &mut emitter.particle_blue),
        (0xFC, &mut emitter.particle_alpha),
        (0x104, &mut emitter.particle_brightness),
        (0x10C, &mut emitter.particle_spin_angle),
        (0x114, &mut emitter.particle_spin_speed),
        (0x11C, &mut emitter.particle_scale),
        (0x124, &mut emitter.particle_stretch),
        (0x12C, &mut emitter.collision_reflect),
        (0x134, &mut emitter.collision_spin),
        (0x13C, &mut emitter.clip_top),
        (0x144, &mut emitter.clip_bottom),
        (0x14C, &mut emitter.clip_left),
        (0x154, &mut emitter.clip_right),
        (0x15C, &mut emitter.animation_rate),
    ] {
        *slot = read_track(reader, record, offset)?;
    }

    Ok(emitter)
}

fn read_particle_fields(
    reader: &mut ReanimReader<'_>,
    record: &[u8],
    offset: usize,
) -> Result<Vec<ParticleField>, ReanimError> {
    let count = read_i32_at(record, offset + 4)?;
    let field_records = reader.read_array("particle field", PARTICLE_FIELD_SIZE, count)?;
    let mut fields = Vec::with_capacity(field_records.len());
    for field_record in field_records {
        fields.push(ParticleField {
            field_type: read_i32_at(field_record, 0x0)?,
            x: read_track(reader, field_record, 0x4)?,
            y: read_track(reader, field_record, 0xC)?,
        });
    }
    Ok(fields)
}

/// Reads one `DT_TRACK_FLOAT` record; `offset` is the track's struct offset,
/// whose `mCountNodes` at `+4` must match the serialized node count.
fn read_track(
    reader: &mut ReanimReader<'_>,
    record: &[u8],
    offset: usize,
) -> Result<ParameterTrack, ReanimError> {
    let declared = read_i32_at(record, offset + 4)?;
    let count = reader.read_i32()?;
    if count != declared {
        return Err(ReanimError::InvalidDefinitionSize {
            field: "parameter track node count",
            expected: declared,
            actual: count,
        });
    }
    if count < 0 {
        return Err(ReanimError::InvalidCount {
            field: "parameter track",
            count,
        });
    }
    let count = usize::try_from(count).unwrap();
    if count > reader.remaining() / TRACK_NODE_SIZE {
        return Err(ReanimError::UnexpectedEnd);
    }
    let mut nodes = Vec::with_capacity(count);
    for _ in 0..count {
        let node = reader.read_bytes(TRACK_NODE_SIZE)?;
        nodes.push(TrackNode {
            time: f32_at(node, 0, "track node time")?,
            low: f32_at(node, 4, "track node low")?,
            high: f32_at(node, 8, "track node high")?,
            curve: read_i32_at(node, 12)?,
            distribution: read_i32_at(node, 16)?,
        });
    }
    Ok(ParameterTrack { nodes })
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

    /// Builds the `DT_TRACK_FLOAT` trailing record and its in-struct node count.
    fn track_fixture(nodes: &[TrackNode]) -> (Vec<u8>, Vec<u8>) {
        let mut trailing = i32::try_from(nodes.len()).unwrap().to_le_bytes().to_vec();
        for node in nodes {
            trailing.extend_from_slice(&node.time.to_le_bytes());
            trailing.extend_from_slice(&node.low.to_le_bytes());
            trailing.extend_from_slice(&node.high.to_le_bytes());
            trailing.extend_from_slice(&node.curve.to_le_bytes());
            trailing.extend_from_slice(&node.distribution.to_le_bytes());
        }
        // `FloatParameterTrack` is { mNodes, mCountNodes }; only the count is read.
        let mut in_struct = vec![0u8; 4];
        in_struct.extend_from_slice(&i32::try_from(nodes.len()).unwrap().to_le_bytes());
        (in_struct, trailing)
    }

    fn write_i32(record: &mut [u8], offset: usize, value: i32) {
        record[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }

    fn write_track(record: &mut [u8], offset: usize, nodes: &[TrackNode], trailing: &mut Vec<u8>) {
        let (in_struct, bytes) = track_fixture(nodes);
        record[offset..offset + in_struct.len()].copy_from_slice(&in_struct);
        trailing.extend_from_slice(&bytes);
    }

    fn write_string(trailing: &mut Vec<u8>, value: &str) {
        trailing.extend_from_slice(&i32::try_from(value.len()).unwrap().to_le_bytes());
        trailing.extend_from_slice(value.as_bytes());
    }

    /// Emitter float tracks before the field arrays, in declaration order.
    const EMITTER_TRACK_OFFSETS: [usize; 21] = [
        0x2C, 0x34, 0x3C, 0x44, 0x4C, 0x54, 0x5C, 0x64, 0x6C, 0x74, 0x8C, 0x7C, 0x84, 0x94, 0xAC,
        0xB4, 0xBC, 0xC4, 0xCC, 0x9C, 0xA4,
    ];
    /// Emitter float tracks after the field arrays, in declaration order.
    const EMITTER_TAIL_TRACK_OFFSETS: [usize; 16] = [
        0xE4, 0xEC, 0xF4, 0xFC, 0x104, 0x10C, 0x114, 0x11C, 0x124, 0x12C, 0x134, 0x13C, 0x144,
        0x14C, 0x154, 0x15C,
    ];

    #[test]
    fn parses_a_particle_definition_in_declaration_order() {
        let spawn_rate = [TrackNode {
            time: 0.0,
            low: 12.0,
            high: 20.0,
            curve: 1,
            distribution: 1,
        }];
        let field_x = [TrackNode {
            time: 0.5,
            low: -3.0,
            high: 3.0,
            curve: 2,
            distribution: 1,
        }];

        let mut record = vec![0u8; usize::try_from(EMITTER_DEFINITION_SIZE).unwrap()];
        let mut trailing = Vec::new();

        write_i32(&mut record, 0x4, 3); // ImageCol
        write_i32(&mut record, 0x8, 2); // ImageRow
        write_i32(&mut record, 0xC, 6); // ImageFrames
        write_i32(&mut record, 0x10, 1); // Animated
        write_i32(&mut record, 0x14, 0b1_0000_0001); // Additive | RandomLaunchSpin
        write_i32(&mut record, 0x18, 1); // EmitterType Box
        write_i32(&mut record, 0xD4 + 4, 1); // one particle field

        // Declaration order: Image, in-struct ints, Name, SystemDuration,
        // OnDuration, 21 tracks, Field, SystemField, then 16 tracks.
        write_string(&mut trailing, "IMAGE_DOOM");
        write_string(&mut trailing, "DoomStem");
        write_track(&mut record, 0x24, &[], &mut trailing);
        write_string(&mut trailing, "3000");
        for offset in EMITTER_TRACK_OFFSETS {
            let nodes: &[TrackNode] = if offset == 0x34 { &spawn_rate } else { &[] };
            write_track(&mut record, offset, nodes, &mut trailing);
        }

        // Field array: size marker, the 0x14 record, then its two tracks.
        trailing.extend_from_slice(&PARTICLE_FIELD_SIZE.to_le_bytes());
        let mut field_record = vec![0u8; usize::try_from(PARTICLE_FIELD_SIZE).unwrap()];
        write_i32(&mut field_record, 0x0, 9); // FIELD_SHAKE
        let (field_x_struct, field_x_trailing) = track_fixture(&field_x);
        field_record[0x4..0x4 + field_x_struct.len()].copy_from_slice(&field_x_struct);
        let (field_y_struct, field_y_trailing) = track_fixture(&[]);
        field_record[0xC..0xC + field_y_struct.len()].copy_from_slice(&field_y_struct);
        trailing.extend_from_slice(&field_record);
        trailing.extend_from_slice(&field_x_trailing);
        trailing.extend_from_slice(&field_y_trailing);

        trailing.extend_from_slice(&PARTICLE_FIELD_SIZE.to_le_bytes()); // empty SystemField
        for offset in EMITTER_TAIL_TRACK_OFFSETS {
            write_track(&mut record, offset, &[], &mut trailing);
        }

        let mut payload = vec![0u8; 8];
        write_i32(&mut payload, 4, 1); // one emitter
        payload.extend_from_slice(&EMITTER_DEFINITION_SIZE.to_le_bytes());
        payload.extend_from_slice(&record);
        payload.extend_from_slice(&trailing);

        let definition = CompiledDefinition::decode(&compiled_fixture(9, &payload))
            .unwrap()
            .particles()
            .unwrap();

        let emitter = &definition.emitters[0];
        assert_eq!(emitter.image.as_deref(), Some("IMAGE_DOOM"));
        assert_eq!(emitter.name, "DoomStem");
        assert_eq!(emitter.on_duration, "3000");
        assert_eq!(emitter.image_col, 3);
        assert_eq!(emitter.image_row, 2);
        assert_eq!(emitter.image_frames, 6);
        assert_eq!(emitter.animated, 1);
        assert_eq!(emitter.emitter_type, 1);
        assert_eq!(emitter.particle_flags, 0b1_0000_0001);
        assert_eq!(emitter.spawn_rate.nodes, spawn_rate);
        assert!(emitter.system_duration.is_empty());
        assert_eq!(emitter.particle_fields.len(), 1);
        assert_eq!(emitter.particle_fields[0].field_type, 9);
        assert_eq!(emitter.particle_fields[0].x.nodes, field_x);
        assert!(emitter.particle_fields[0].y.is_empty());
        assert!(emitter.system_fields.is_empty());
    }

    #[test]
    fn particle_definition_rejects_a_foreign_emitter_size() {
        let mut payload = vec![0u8; 8];
        write_i32(&mut payload, 4, 1);
        payload.extend_from_slice(&0x100i32.to_le_bytes());
        let error = ParticleDefinition::parse(&payload).unwrap_err();
        assert!(matches!(
            error,
            ReanimError::InvalidDefinitionSize {
                field: "emitter",
                expected: EMITTER_DEFINITION_SIZE,
                actual: 0x100,
            }
        ));
    }

    #[test]
    fn parses_a_trail_definition_in_declaration_order() {
        let width = [
            TrackNode {
                time: 0.0,
                low: 1.0,
                high: 1.0,
                curve: 1,
                distribution: 1,
            },
            TrackNode {
                time: 1.0,
                low: 0.0,
                high: 0.0,
                curve: 1,
                distribution: 1,
            },
        ];

        let mut record = vec![0u8; TRAIL_DEFINITION_SIZE];
        let mut trailing = Vec::new();
        write_i32(&mut record, 0x4, 20); // MaxPoints
        record[0x8..0xC].copy_from_slice(&3.0f32.to_le_bytes()); // MinPointDistance
        write_i32(&mut record, 0xC, 1); // TrailFlags: Loops

        // Declaration order puts TrailDuration last, after the four curves.
        write_string(&mut trailing, "IMAGE_ICETRAIL");
        write_track(&mut record, 0x18, &width, &mut trailing); // WidthOverLength
        write_track(&mut record, 0x20, &[], &mut trailing); // WidthOverTime
        write_track(&mut record, 0x28, &width, &mut trailing); // AlphaOverLength
        write_track(&mut record, 0x30, &[], &mut trailing); // AlphaOverTime
        write_track(&mut record, 0x10, &[], &mut trailing); // TrailDuration

        let mut payload = record;
        payload.extend_from_slice(&trailing);

        let trail = CompiledDefinition::decode(&compiled_fixture(11, &payload))
            .unwrap()
            .trail()
            .unwrap();

        assert_eq!(trail.image.as_deref(), Some("IMAGE_ICETRAIL"));
        assert_eq!(trail.max_points, 20);
        assert_eq!(trail.min_point_distance, 3.0);
        assert_eq!(trail.trail_flags, 1);
        assert_eq!(trail.width_over_length.nodes, width);
        assert_eq!(trail.alpha_over_length.nodes, width);
        assert!(trail.width_over_time.is_empty());
        assert!(trail.alpha_over_time.is_empty());
        assert!(trail.trail_duration.is_empty());
    }
}
