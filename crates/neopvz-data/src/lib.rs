use std::{
    fs::{self, File},
    io::{BufRead, Read},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

mod formats;
mod pak;

pub use formats::{CompiledDefinition, CompiledError, Mo3Resource, MusicError};
pub use pak::{PakArchive, PakEntry, PakError};

pub const TARGET_RESOURCE_VERSION: &str = "1.0.0.1051";
const RESOURCE_MANIFEST_PATH: &str = "properties/resources.xml";
pub(crate) const MAX_RESOURCE_SIZE: u64 = 64 * 1024 * 1024;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("resource path does not exist: {0}")]
    MissingPath(PathBuf),
    #[error("resource path is not a supported directory or PAK archive: {0}")]
    UnsupportedPath(PathBuf),
}

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("invalid resource manifest XML: {0}")]
    Xml(#[from] quick_xml::DeError),
}

#[derive(Debug, Error)]
pub enum ResourcePathError {
    #[error("unsafe resource path: {0}")]
    Unsafe(String),
    #[error("resource path escapes the resource directory: {0}")]
    EscapesRoot(String),
}

#[derive(Debug, Error)]
pub enum ResourceError {
    #[error(transparent)]
    Path(#[from] ResourcePathError),
    #[error("failed to read loose resource: {0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Pak(#[from] PakError),
    #[error(transparent)]
    Manifest(#[from] ManifestError),
    #[error(transparent)]
    Compiled(#[from] CompiledError),
    #[error(transparent)]
    Music(#[from] MusicError),
    #[error("resource is too large: {path} ({size} bytes)")]
    TooLarge { path: String, size: u64 },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ResourceSource {
    Directory(PathBuf),
    Pak(PathBuf),
}

#[derive(Debug)]
pub enum ResourceProvider {
    Directory(PathBuf),
    Pak(PakArchive),
}

impl ResourceProvider {
    pub fn open(source: &ResourceSource) -> Result<Self, ResourceError> {
        match source {
            ResourceSource::Directory(root) => Ok(Self::Directory(root.canonicalize()?)),
            ResourceSource::Pak(path) => Ok(Self::Pak(PakArchive::load(path)?)),
        }
    }

    pub fn read(&self, path: &str) -> Result<Vec<u8>, ResourceError> {
        let normalized = normalize_resource_path(path)?;
        match self {
            Self::Directory(root) => {
                let resolved = root.join(&normalized).canonicalize()?;
                if !resolved.starts_with(root) {
                    return Err(ResourcePathError::EscapesRoot(path.to_owned()).into());
                }
                let mut data = Vec::new();
                File::open(resolved)?
                    .take(MAX_RESOURCE_SIZE + 1)
                    .read_to_end(&mut data)?;
                let size = u64::try_from(data.len()).unwrap_or(u64::MAX);
                if size > MAX_RESOURCE_SIZE {
                    return Err(ResourceError::TooLarge {
                        path: path.to_owned(),
                        size,
                    });
                }
                Ok(data)
            }
            Self::Pak(archive) => Ok(archive.read(&normalized)?),
        }
    }

    pub fn paths(&self) -> Result<Vec<String>, ResourceError> {
        match self {
            Self::Directory(root) => directory_paths(root),
            Self::Pak(archive) => Ok(archive
                .entries()
                .map(|entry| entry.path.clone())
                .collect()),
        }
    }

    pub fn compiled_animation_paths(&self) -> Result<Vec<String>, ResourceError> {
        Ok(self
            .paths()?
            .into_iter()
            .filter(|path| is_compiled_animation(path))
            .collect())
    }

    pub fn music_paths(&self) -> Result<Vec<String>, ResourceError> {
        Ok(self
            .paths()?
            .into_iter()
            .filter(|path| is_music(path))
            .collect())
    }

    pub fn read_compiled(&self, path: &str) -> Result<CompiledDefinition, ResourceError> {
        Ok(CompiledDefinition::decode(&self.read(path)?)?)
    }

    pub fn read_music(&self, path: &str) -> Result<Mo3Resource, ResourceError> {
        Ok(Mo3Resource::parse(self.read(path)?)?)
    }

    pub fn inventory(&self) -> Result<ResourceInventory, ResourceError> {
        let manifest_data = self.read(RESOURCE_MANIFEST_PATH)?;
        let manifest = ResourceManifest::parse(&manifest_data[..])?;
        let paths = self.paths()?;
        let compiled: Vec<_> = paths
            .iter()
            .filter(|path| is_compiled_animation(path))
            .collect();
        let music: Vec<_> = paths.iter().filter(|path| is_music(path)).collect();

        for path in &compiled {
            self.read_compiled(path)?;
        }
        for path in &music {
            self.read_music(path)?;
        }

        Ok(ResourceInventory {
            groups: manifest.groups.len(),
            entries: manifest.entry_count(),
            images: manifest.count(ResourceKind::Image),
            fonts: manifest.count(ResourceKind::Font),
            sounds: manifest.count(ResourceKind::Sound),
            compiled_animations: compiled.len(),
            music: music.len(),
        })
    }
}

fn directory_paths(root: &Path) -> Result<Vec<String>, ResourceError> {
    let mut pending = vec![root.to_path_buf()];
    let mut paths = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_symlink() {
                continue;
            }

            let path = entry.path();
            if file_type.is_dir() {
                pending.push(path);
            } else if file_type.is_file() {
                let relative = path
                    .strip_prefix(root)
                    .map_err(|_| ResourcePathError::Unsafe(path.display().to_string()))?;
                let relative = relative
                    .to_str()
                    .ok_or_else(|| ResourcePathError::Unsafe(path.display().to_string()))?;
                paths.push(normalize_resource_path(relative)?);
            }
        }
    }
    paths.sort_unstable();
    Ok(paths)
}

fn is_compiled_animation(path: &str) -> bool {
    let path = path.to_ascii_uppercase();
    path.starts_with("COMPILED/") && path.ends_with(".COMPILED")
}

fn is_music(path: &str) -> bool {
    let path = path.to_ascii_uppercase();
    path.starts_with("SOUNDS/") && path.ends_with(".MO3")
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AssetLayout {
    pub source: ResourceSource,
    pub manifest: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ResourceKind {
    Image,
    Font,
    Sound,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceInventory {
    pub groups: usize,
    pub entries: usize,
    pub images: usize,
    pub fonts: usize,
    pub sounds: usize,
    pub compiled_animations: usize,
    pub music: usize,
}

impl ResourceInventory {
    pub fn version(&self) -> Option<&'static str> {
        (self.groups == 29
            && self.entries == 626
            && self.images == 439
            && self.fonts == 20
            && self.sounds == 167
            && self.compiled_animations == 250
            && self.music == 2)
            .then_some(TARGET_RESOURCE_VERSION)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceEntry {
    pub kind: ResourceKind,
    pub id: String,
    pub path: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroup {
    pub id: String,
    pub entries: Vec<ResourceEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceManifest {
    pub groups: Vec<ResourceGroup>,
}

impl ResourceManifest {
    pub fn parse<R: BufRead>(source: R) -> Result<Self, ManifestError> {
        let raw: RawManifest = quick_xml::de::from_reader(source)?;
        Ok(Self {
            groups: raw.groups.into_iter().map(ResourceGroup::from).collect(),
        })
    }

    pub fn count(&self, kind: ResourceKind) -> usize {
        self.groups
            .iter()
            .flat_map(|group| &group.entries)
            .filter(|entry| entry.kind == kind)
            .count()
    }

    pub fn entry_count(&self) -> usize {
        self.groups.iter().map(|group| group.entries.len()).sum()
    }
}

pub(crate) fn normalize_resource_path(path: &str) -> Result<String, ResourcePathError> {
    let path = path.replace('\\', "/");
    if path.is_empty() || path.starts_with('/') {
        return Err(ResourcePathError::Unsafe(path));
    }

    let mut normalized = Vec::new();
    for component in path.split('/') {
        if component.is_empty()
            || component == "."
            || component == ".."
            || component.contains(':')
            || component.contains('\0')
        {
            return Err(ResourcePathError::Unsafe(path));
        }
        normalized.push(component);
    }
    Ok(normalized.join("/"))
}

#[derive(Deserialize)]
struct RawManifest {
    #[serde(rename = "Resources", default)]
    groups: Vec<RawGroup>,
}

#[derive(Deserialize)]
struct RawGroup {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "$value", default)]
    items: Vec<RawItem>,
}

#[derive(Deserialize)]
enum RawItem {
    SetDefaults(RawDefaults),
    Image(RawEntry),
    Font(RawEntry),
    Sound(RawEntry),
}

#[derive(Default, Deserialize)]
struct RawDefaults {
    #[serde(rename = "@path", default)]
    path: Option<String>,
    #[serde(rename = "@idprefix", default)]
    id_prefix: Option<String>,
}

#[derive(Deserialize)]
struct RawEntry {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@path")]
    path: String,
}

impl From<RawGroup> for ResourceGroup {
    fn from(raw: RawGroup) -> Self {
        let mut path = String::new();
        let mut id_prefix = String::new();
        let mut entries = Vec::new();

        for item in raw.items {
            match item {
                RawItem::SetDefaults(defaults) => {
                    if let Some(value) = defaults.path {
                        path = value;
                    }
                    if let Some(value) = defaults.id_prefix {
                        id_prefix = value;
                    }
                }
                RawItem::Image(entry) => entries.push(resource_entry(
                    ResourceKind::Image,
                    &id_prefix,
                    &path,
                    entry,
                )),
                RawItem::Font(entry) => {
                    entries.push(resource_entry(ResourceKind::Font, &id_prefix, &path, entry))
                }
                RawItem::Sound(entry) => entries.push(resource_entry(
                    ResourceKind::Sound,
                    &id_prefix,
                    &path,
                    entry,
                )),
            }
        }

        Self {
            id: raw.id,
            entries,
        }
    }
}

fn resource_entry(
    kind: ResourceKind,
    id_prefix: &str,
    base_path: &str,
    raw: RawEntry,
) -> ResourceEntry {
    ResourceEntry {
        kind,
        id: format!("{id_prefix}{}", raw.id),
        path: join_resource_path(base_path, &raw.path),
    }
}

fn join_resource_path(base: &str, path: &str) -> String {
    let base = base.trim_end_matches(&['/', '\\'][..]);
    let path = path.trim_start_matches(&['/', '\\'][..]);
    if base.is_empty() {
        path.replace('\\', "/")
    } else if path.is_empty() {
        base.replace('\\', "/")
    } else {
        format!("{base}/{path}").replace('\\', "/")
    }
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

    #[test]
    fn parses_manifest_defaults_and_counts_entries() {
        let xml = br#"
            <ResourceManifest>
              <Resources id="Init">
                <SetDefaults path="images" idprefix="IMAGE_" />
                <Image id="LOGO" path="logo" />
                <SetDefaults path="sounds" idprefix="SOUND_" />
                <Sound id="CLICK" path="click" />
              </Resources>
            </ResourceManifest>
        "#;

        let manifest = ResourceManifest::parse(&xml[..]).unwrap();
        assert_eq!(manifest.groups.len(), 1);
        assert_eq!(manifest.entry_count(), 2);
        assert_eq!(manifest.count(ResourceKind::Image), 1);
        assert_eq!(manifest.count(ResourceKind::Font), 0);
        assert_eq!(manifest.count(ResourceKind::Sound), 1);
        assert_eq!(manifest.groups[0].entries[0].id, "IMAGE_LOGO");
        assert_eq!(manifest.groups[0].entries[0].path, "images/logo");
        assert_eq!(manifest.groups[0].entries[1].id, "SOUND_CLICK");
        assert_eq!(manifest.groups[0].entries[1].path, "sounds/click");
    }

    #[test]
    fn rejects_resource_without_path() {
        let error = ResourceManifest::parse(
            &b"<ResourceManifest><Resources id=\"Init\"><Image id=\"x\" /></Resources></ResourceManifest>"[..],
        )
        .unwrap_err();
        assert!(matches!(error, ManifestError::Xml(_)));
    }

    #[test]
    fn directory_and_pak_providers_read_the_same_path() {
        let root = tempfile::tempdir().unwrap();
        let directory = root.path().join("loose");
        fs::create_dir_all(directory.join("properties")).unwrap();
        fs::write(
            directory.join("properties/resources.xml"),
            b"<ResourceManifest />",
        )
        .unwrap();
        fs::create_dir_all(directory.join("compiled/reanim")).unwrap();
        fs::write(directory.join("compiled/reanim/test.compiled"), b"compiled").unwrap();
        fs::create_dir(directory.join("sounds")).unwrap();
        fs::write(directory.join("sounds/mainmusic.mo3"), b"MO3\0music").unwrap();

        let pak_path = root.path().join("main.pak");
        fs::write(
            &pak_path,
            pak::fixture(&[
                ("properties\\resources.xml", b"<ResourceManifest />"),
                ("compiled\\reanim\\test.compiled", b"compiled"),
                ("sounds\\mainmusic.mo3", b"MO3\0music"),
            ]),
        )
        .unwrap();

        let loose = ResourceProvider::open(&ResourceSource::Directory(directory)).unwrap();
        let pak = ResourceProvider::open(&ResourceSource::Pak(pak_path)).unwrap();
        assert_eq!(
            loose.read("properties\\resources.xml").unwrap(),
            pak.read("properties\\resources.xml").unwrap()
        );
        assert_eq!(loose.compiled_animation_paths().unwrap().len(), 1);
        assert_eq!(pak.compiled_animation_paths().unwrap().len(), 1);
        assert_eq!(loose.music_paths().unwrap().len(), 1);
        assert_eq!(pak.music_paths().unwrap().len(), 1);
    }

    #[test]
    fn rejects_unsafe_resource_paths() {
        for path in [
            "",
            "/absolute",
            "\\absolute",
            "C:\\absolute",
            "../outside",
            "inside/../outside",
            "./inside",
            "inside//file",
        ] {
            assert!(normalize_resource_path(path).is_err(), "accepted {path:?}");
        }
    }

    #[cfg(unix)]
    #[test]
    fn directory_provider_rejects_symlink_escape() {
        use std::os::unix::fs::symlink;

        let root = tempfile::tempdir().unwrap();
        let directory = root.path().join("loose");
        fs::create_dir(&directory).unwrap();
        let outside = root.path().join("outside");
        fs::write(&outside, b"private").unwrap();
        symlink(&outside, directory.join("escape")).unwrap();

        let provider = ResourceProvider::open(&ResourceSource::Directory(directory)).unwrap();
        assert!(matches!(
            provider.read("escape"),
            Err(ResourceError::Path(ResourcePathError::EscapesRoot(_)))
        ));
    }

    #[test]
    fn identifies_the_target_inventory() {
        let inventory = ResourceInventory {
            groups: 29,
            entries: 626,
            images: 439,
            fonts: 20,
            sounds: 167,
            compiled_animations: 250,
            music: 2,
        };
        assert_eq!(inventory.version(), Some(TARGET_RESOURCE_VERSION));
    }
}
