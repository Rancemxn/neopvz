use std::{
    collections::{HashMap, hash_map::Entry},
    io::Cursor,
    path::Path,
};

use kira::backend::cpal::{
    CpalBackendSettings,
    cpal::{BufferSize, StreamConfig},
};
use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle};
use kira::track::{TrackBuilder, TrackHandle};
use kira::{AudioManager, AudioManagerSettings, DefaultBackend, Frame, Tween};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("audio backend is not initialized")]
    NotInitialized,
    #[error("audio asset is missing: {0}")]
    MissingAsset(String),
    #[error("audio backend failed: {0}")]
    Backend(String),
    #[error("audio decoding failed for {path}: {reason}")]
    Decode { path: String, reason: String },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AudioKind {
    Effect,
    Music,
}

pub trait AudioBackend {
    fn play(&mut self, kind: AudioKind, path: &Path) -> Result<(), AudioError>;
    fn play_bytes(&mut self, kind: AudioKind, path: &str, bytes: Vec<u8>)
    -> Result<(), AudioError>;
    fn stop_music(&mut self);
    fn set_volume(&mut self, kind: AudioKind, decibels: f32);
}

pub struct KiraAudioBackend {
    _manager: AudioManager<DefaultBackend>,
    effects: TrackHandle,
    music: TrackHandle,
    music_handle: Option<StaticSoundHandle>,
    sounds: HashMap<String, StaticSoundData>,
}

impl KiraAudioBackend {
    pub fn new() -> Result<Self, AudioError> {
        let settings = if std::env::var_os("NEOPVZ_FORCE_AUDIO_DEVICE_ERROR").is_some() {
            AudioManagerSettings {
                backend_settings: CpalBackendSettings {
                    device: None,
                    config: Some(StreamConfig {
                        channels: 0,
                        sample_rate: 1,
                        buffer_size: BufferSize::Default,
                    }),
                },
                ..AudioManagerSettings::default()
            }
        } else {
            AudioManagerSettings::default()
        };
        let mut manager = AudioManager::<DefaultBackend>::new(settings)
            .map_err(|error| AudioError::Backend(error.to_string()))?;
        let effects = manager
            .add_sub_track(TrackBuilder::new())
            .map_err(|error| AudioError::Backend(error.to_string()))?;
        let music = manager
            .add_sub_track(TrackBuilder::new())
            .map_err(|error| AudioError::Backend(error.to_string()))?;
        Ok(Self {
            _manager: manager,
            effects,
            music,
            music_handle: None,
            sounds: HashMap::new(),
        })
    }

    pub fn preload_bytes(&mut self, path: &str, bytes: Vec<u8>) -> Result<(), AudioError> {
        cached_sound(&mut self.sounds, path, || load_sound_bytes(path, bytes)).map(|_| ())
    }
}

impl AudioBackend for KiraAudioBackend {
    fn play(&mut self, kind: AudioKind, path: &Path) -> Result<(), AudioError> {
        let data = cached_sound(&mut self.sounds, &path.to_string_lossy(), || {
            load_sound(path)
        })?;
        self.play_data(kind, data)
    }

    fn play_bytes(
        &mut self,
        kind: AudioKind,
        path: &str,
        bytes: Vec<u8>,
    ) -> Result<(), AudioError> {
        let data = cached_sound(&mut self.sounds, path, || load_sound_bytes(path, bytes))?;
        self.play_data(kind, data)
    }

    fn stop_music(&mut self) {
        if let Some(mut handle) = self.music_handle.take() {
            handle.stop(Tween::default());
        }
    }

    fn set_volume(&mut self, kind: AudioKind, decibels: f32) {
        let track = match kind {
            AudioKind::Effect => &mut self.effects,
            AudioKind::Music => &mut self.music,
        };
        track.set_volume(decibels, Tween::default());
    }
}

impl KiraAudioBackend {
    fn play_data(&mut self, kind: AudioKind, data: StaticSoundData) -> Result<(), AudioError> {
        match kind {
            AudioKind::Effect => {
                self.effects
                    .play(data)
                    .map_err(|error| AudioError::Backend(error.to_string()))?;
            }
            AudioKind::Music => {
                self.stop_music();
                self.music_handle = Some(
                    self.music
                        .play(data)
                        .map_err(|error| AudioError::Backend(error.to_string()))?,
                );
            }
        }
        Ok(())
    }
}

fn cached_sound(
    sounds: &mut HashMap<String, StaticSoundData>,
    path: &str,
    load: impl FnOnce() -> Result<StaticSoundData, AudioError>,
) -> Result<StaticSoundData, AudioError> {
    let data = match sounds.entry(path.to_owned()) {
        Entry::Occupied(entry) => entry.into_mut(),
        Entry::Vacant(entry) => entry.insert(load()?),
    };
    // Kira clones share the decoded PCM; each playback has its own handle.
    Ok(data.clone())
}

fn load_sound(path: &Path) -> Result<StaticSoundData, AudioError> {
    if !path.is_file() {
        return Err(AudioError::MissingAsset(path.display().to_string()));
    }
    if path
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("au"))
    {
        let bytes = std::fs::read(path).map_err(|error| AudioError::Decode {
            path: path.display().to_string(),
            reason: error.to_string(),
        })?;
        return load_sound_bytes(&path.display().to_string(), bytes);
    }
    StaticSoundData::from_file(path).map_err(|error| AudioError::Decode {
        path: path.display().to_string(),
        reason: error.to_string(),
    })
}

fn load_sound_bytes(path: &str, bytes: Vec<u8>) -> Result<StaticSoundData, AudioError> {
    if path.to_ascii_lowercase().ends_with(".au") {
        return load_au_sound(path, &bytes);
    }
    StaticSoundData::from_cursor(Cursor::new(bytes)).map_err(|error| AudioError::Decode {
        path: path.to_owned(),
        reason: error.to_string(),
    })
}

fn load_au_sound(path: &str, bytes: &[u8]) -> Result<StaticSoundData, AudioError> {
    let invalid = |reason: &str| AudioError::Decode {
        path: path.to_owned(),
        reason: reason.to_owned(),
    };
    if bytes.len() < 24 || &bytes[..4] != b".snd" {
        return Err(invalid("invalid Sun AU header"));
    }
    let data_offset = u32::from_be_bytes(bytes[4..8].try_into().unwrap()) as usize;
    let data_size = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
    let encoding = u32::from_be_bytes(bytes[12..16].try_into().unwrap());
    let sample_rate = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
    let channels = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
    if data_offset < 24 || data_offset > bytes.len() || sample_rate == 0 || channels != 1 {
        return Err(invalid("unsupported Sun AU stream header"));
    }
    if encoding != 1 {
        return Err(invalid("unsupported Sun AU encoding"));
    }
    let available = bytes.len() - data_offset;
    let data_len = if data_size == u32::MAX {
        available
    } else {
        usize::try_from(data_size)
            .map_err(|_| invalid("Sun AU data size does not fit this platform"))?
    };
    if data_len > available {
        return Err(invalid("Sun AU data exceeds the resource"));
    }
    let frames = bytes[data_offset..data_offset + data_len]
        .iter()
        .copied()
        .map(|sample| Frame::from_mono(decode_mulaw(sample)))
        .collect::<Vec<_>>();
    Ok(StaticSoundData {
        sample_rate,
        frames: frames.into(),
        settings: Default::default(),
        slice: None,
    })
}

fn decode_mulaw(sample: u8) -> f32 {
    let sample = !sample;
    let sign = sample & 0x80 != 0;
    let exponent = u32::from((sample >> 4) & 0x07);
    let mantissa = i32::from(sample & 0x0f);
    let magnitude = ((mantissa << 3) + 132) << exponent;
    let magnitude = magnitude - 132;
    let signed = if sign { -magnitude } else { magnitude };
    signed as f32 / 32_768.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cached_sounds_reuse_pcm_and_failed_decodes_can_be_retried() {
        let mut sounds = HashMap::new();
        assert!(
            cached_sound(&mut sounds, "sample", || load_sound_bytes("sample", vec![])).is_err()
        );
        assert!(sounds.is_empty());
        let first = cached_sound(&mut sounds, "sample", || {
            Ok(StaticSoundData {
                sample_rate: 8_012,
                frames: vec![Frame::from_mono(0.5)].into(),
                settings: Default::default(),
                slice: None,
            })
        })
        .unwrap();
        let second =
            cached_sound(&mut sounds, "sample", || panic!("decoded a cached sound")).unwrap();
        assert!(std::sync::Arc::ptr_eq(&first.frames, &second.frames));
        assert_eq!(second.frames[0], Frame::from_mono(0.5));
    }

    #[test]
    fn rejects_missing_audio_before_decoding() {
        let result = load_sound(Path::new("definitely-missing-neopvz-audio.ogg"));
        assert!(matches!(result, Err(AudioError::MissingAsset(_))));
    }

    #[test]
    fn rejects_invalid_resource_audio_with_its_path() {
        let result = load_sound_bytes("sounds/click.ogg", b"not audio".to_vec());
        assert!(matches!(
            result,
            Err(AudioError::Decode { path, .. }) if path == "sounds/click.ogg"
        ));
    }

    #[test]
    fn decodes_sun_au_mulaw_bytes_into_mono_frames() {
        let mut bytes = Vec::from(&b".snd"[..]);
        bytes.extend_from_slice(&24_u32.to_be_bytes());
        bytes.extend_from_slice(&2_u32.to_be_bytes());
        bytes.extend_from_slice(&1_u32.to_be_bytes());
        bytes.extend_from_slice(&8_012_u32.to_be_bytes());
        bytes.extend_from_slice(&1_u32.to_be_bytes());
        bytes.extend_from_slice(&[0xff, 0x00]);

        let sound = load_sound_bytes("sounds/diamond.au", bytes).expect("valid AU");
        assert_eq!(sound.sample_rate, 8_012);
        assert_eq!(sound.num_frames(), 2);
        assert_eq!(sound.frames[0], Frame::from_mono(0.0));
        assert!(sound.frames[1].left < 0.0);
        assert_eq!(sound.frames[1].left, sound.frames[1].right);
    }
}
