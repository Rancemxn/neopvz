use std::path::Path;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("audio backend is not initialized")]
    NotInitialized,
    #[error("audio asset is missing: {0}")]
    MissingAsset(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AudioKind {
    Effect,
    Music,
}

pub trait AudioBackend {
    fn play(&mut self, kind: AudioKind, path: &Path) -> Result<(), AudioError>;
    fn stop_music(&mut self);
    fn set_volume(&mut self, kind: AudioKind, volume: f32);
}
