use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("resource path does not exist: {0}")]
    MissingPath(PathBuf),
    #[error("resource path is not a supported directory or PAK archive: {0}")]
    UnsupportedPath(PathBuf),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ResourceSource {
    Directory(PathBuf),
    Pak(PathBuf),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AssetLayout {
    pub source: ResourceSource,
    pub manifest: Option<PathBuf>,
}

impl AssetLayout {
    pub fn discover(explicit: Option<&Path>) -> Result<Self, DataError> {
        let candidates = explicit
            .map(|path| vec![path.to_path_buf()])
            .unwrap_or_else(|| {
                let current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
                vec![current.clone(), current.join("data"), current.join("resources")]
            });

        for candidate in candidates {
            if !candidate.exists() {
                continue;
            }
            if candidate.is_file() && is_pak(&candidate) {
                return Ok(Self {
                    source: ResourceSource::Pak(candidate),
                    manifest: None,
                });
            }
            if candidate.is_dir() {
                let manifest = candidate.join("properties").join("resources.xml");
                if manifest.is_file() {
                    return Ok(Self {
                        source: ResourceSource::Directory(candidate),
                        manifest: Some(manifest),
                    });
                }
                let pak = candidate.join("main.pak");
                if pak.is_file() {
                    return Ok(Self {
                        source: ResourceSource::Pak(pak),
                        manifest: None,
                    });
                }
            }
        }

        match explicit {
            Some(path) if !path.exists() => Err(DataError::MissingPath(path.to_path_buf())),
            Some(path) => Err(DataError::UnsupportedPath(path.to_path_buf())),
            None => Err(DataError::UnsupportedPath(PathBuf::from("auto-detect"))),
        }
    }
}

fn is_pak(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.eq_ignore_ascii_case("pak"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_pak_extension_case_insensitively() {
        assert!(is_pak(Path::new("main.PAK")));
        assert!(!is_pak(Path::new("main.dat")));
    }
}
