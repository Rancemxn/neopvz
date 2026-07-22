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
        let current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self::discover_from(&current, explicit)
    }

    fn discover_from(current: &Path, explicit: Option<&Path>) -> Result<Self, DataError> {
        let candidates = explicit
            .map(|path| vec![path.to_path_buf()])
            .unwrap_or_else(|| {
                vec![
                    current.to_path_buf(),
                    current.join("data"),
                    current.join("resources"),
                ]
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
    use std::fs;

    #[test]
    fn detects_pak_extension_case_insensitively() {
        assert!(is_pak(Path::new("main.PAK")));
        assert!(!is_pak(Path::new("main.dat")));
    }

    #[test]
    fn discovers_loose_resource_directory() {
        let root = tempfile::tempdir().unwrap();
        let properties = root.path().join("properties");
        fs::create_dir(&properties).unwrap();
        fs::write(properties.join("resources.xml"), "<ResourceManifest />").unwrap();

        let layout = AssetLayout::discover(Some(root.path())).unwrap();
        assert_eq!(layout.source, ResourceSource::Directory(root.path().into()));
        assert_eq!(layout.manifest, Some(properties.join("resources.xml")));
    }

    #[test]
    fn discovers_main_pak_in_directory() {
        let root = tempfile::tempdir().unwrap();
        let pak = root.path().join("main.pak");
        fs::write(&pak, []).unwrap();

        let layout = AssetLayout::discover(Some(root.path())).unwrap();
        assert_eq!(layout.source, ResourceSource::Pak(pak));
        assert_eq!(layout.manifest, None);
    }

    #[test]
    fn discovers_explicit_pak_file() {
        let root = tempfile::tempdir().unwrap();
        let pak = root.path().join("game.PAK");
        fs::write(&pak, []).unwrap();

        let layout = AssetLayout::discover(Some(&pak)).unwrap();
        assert_eq!(layout.source, ResourceSource::Pak(pak));
        assert_eq!(layout.manifest, None);
    }

    #[test]
    fn auto_discovers_data_directory() {
        let root = tempfile::tempdir().unwrap();
        let data = root.path().join("data");
        let properties = data.join("properties");
        fs::create_dir_all(&properties).unwrap();
        fs::write(properties.join("resources.xml"), "<ResourceManifest />").unwrap();

        let layout = AssetLayout::discover_from(root.path(), None).unwrap();
        assert_eq!(layout.source, ResourceSource::Directory(data));
        assert_eq!(layout.manifest, Some(properties.join("resources.xml")));
    }

    #[test]
    fn auto_discovers_resources_main_pak() {
        let root = tempfile::tempdir().unwrap();
        let resources = root.path().join("resources");
        fs::create_dir(&resources).unwrap();
        let pak = resources.join("main.pak");
        fs::write(&pak, []).unwrap();

        let layout = AssetLayout::discover_from(root.path(), None).unwrap();
        assert_eq!(layout.source, ResourceSource::Pak(pak));
        assert_eq!(layout.manifest, None);
    }
}
