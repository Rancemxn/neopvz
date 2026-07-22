use std::path::Path;

use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle};
use kira::track::{TrackBuilder, TrackHandle};
use kira::{AudioManager, AudioManagerSettings, DefaultBackend, Tween};
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
    fn stop_music(&mut self);
    fn set_volume(&mut self, kind: AudioKind, decibels: f32);
}

pub struct KiraAudioBackend {
    _manager: AudioManager<DefaultBackend>,
    effects: TrackHandle,
    music: TrackHandle,
    music_handle: Option<StaticSoundHandle>,
}

impl KiraAudioBackend {
    pub fn new() -> Result<Self, AudioError> {
        let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
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
        })
    }
}

impl AudioBackend for KiraAudioBackend {
    fn play(&mut self, kind: AudioKind, path: &Path) -> Result<(), AudioError> {
        let data = load_sound(path)?;
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

fn load_sound(path: &Path) -> Result<StaticSoundData, AudioError> {
    if !path.is_file() {
        return Err(AudioError::MissingAsset(path.display().to_string()));
    }
    StaticSoundData::from_file(path).map_err(|error| AudioError::Decode {
        path: path.display().to_string(),
        reason: error.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_missing_audio_before_decoding() {
        let result = load_sound(Path::new("definitely-missing-neopvz-audio.ogg"));
        assert!(matches!(result, Err(AudioError::MissingAsset(_))));
    }
}
