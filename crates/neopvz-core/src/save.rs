use std::{
    collections::BTreeSet,
    fs,
    io::{self, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use thiserror::Error;

use crate::PlantType;

pub const SAVE_FORMAT_VERSION: u32 = 1;

const MAX_PROFILE_ID_BYTES: usize = 64;
const MAX_AWARD_ID_BYTES: usize = 64;
const MAX_UNLOCKED_PLANTS: usize = 53;
const MAX_GARDEN_PLANTS: usize = 32;
const MAX_MODE_ENTRIES: usize = 6;

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("save serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("save I/O failed: {0}")]
    Io(#[from] io::Error),
    #[error("unsupported save format {found}; expected {expected}")]
    UnsupportedFormat { expected: u32, found: u32 },
    #[error("invalid save data: {0}")]
    Invalid(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SaveSettings {
    pub music_volume_percent: u8,
    pub effects_volume_percent: u8,
    pub fullscreen: bool,
    pub logical_scale: u8,
}

impl Default for SaveSettings {
    fn default() -> Self {
        Self {
            music_volume_percent: 100,
            effects_volume_percent: 100,
            fullscreen: false,
            logical_scale: 1,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SaveInventory {
    pub coins: u32,
    pub seed_packets: Vec<PlantType>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GardenPlant {
    pub plant_type: PlantType,
    pub age_ticks: u32,
    pub watered: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GardenState {
    pub plants: Vec<GardenPlant>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd, Serialize)]
pub enum ModeKind {
    Adventure,
    Survival,
    MiniGame,
    Vasebreaker,
    IZombie,
    ZenGarden,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModeCompletion {
    pub mode: ModeKind,
    pub completed_levels: u16,
    pub endless_unlocked: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SaveProfile {
    pub format_version: u32,
    pub profile_id: String,
    pub settings: SaveSettings,
    pub unlocked_plants: Vec<PlantType>,
    pub awards: Vec<String>,
    pub inventory: SaveInventory,
    pub garden: GardenState,
    pub mode_completion: Vec<ModeCompletion>,
}

impl Default for SaveProfile {
    fn default() -> Self {
        Self {
            format_version: SAVE_FORMAT_VERSION,
            profile_id: "default".to_owned(),
            settings: SaveSettings::default(),
            unlocked_plants: Vec::new(),
            awards: Vec::new(),
            inventory: SaveInventory::default(),
            garden: GardenState::default(),
            mode_completion: Vec::new(),
        }
    }
}

impl SaveProfile {
    pub fn new(profile_id: impl Into<String>) -> Self {
        Self {
            format_version: SAVE_FORMAT_VERSION,
            profile_id: profile_id.into(),
            ..Self::default()
        }
    }

    pub fn validate(&self) -> Result<(), SaveError> {
        if self.format_version != SAVE_FORMAT_VERSION {
            return Err(SaveError::UnsupportedFormat {
                expected: SAVE_FORMAT_VERSION,
                found: self.format_version,
            });
        }
        validate_text("profile_id", &self.profile_id, MAX_PROFILE_ID_BYTES)?;
        if self.settings.music_volume_percent > 100
            || self.settings.effects_volume_percent > 100
            || self.settings.logical_scale == 0
        {
            return Err(SaveError::Invalid("settings are out of range".to_owned()));
        }

        validate_unique_plants("unlocked_plants", &self.unlocked_plants)?;
        if self.unlocked_plants.len() > MAX_UNLOCKED_PLANTS {
            return Err(SaveError::Invalid("too many unlocked plants".to_owned()));
        }
        validate_unique_plants("inventory.seed_packets", &self.inventory.seed_packets)?;
        validate_unique_text("awards", &self.awards, MAX_AWARD_ID_BYTES)?;
        if self.garden.plants.len() > MAX_GARDEN_PLANTS {
            return Err(SaveError::Invalid("too many garden plants".to_owned()));
        }
        for plant in &self.garden.plants {
            if plant.plant_type.slot() >= 53 {
                return Err(SaveError::Invalid(
                    "garden contains an unknown plant".to_owned(),
                ));
            }
        }
        if self.mode_completion.len() > MAX_MODE_ENTRIES {
            return Err(SaveError::Invalid("too many mode entries".to_owned()));
        }
        let mut modes = BTreeSet::new();
        for entry in &self.mode_completion {
            if !modes.insert(entry.mode) {
                return Err(SaveError::Invalid(
                    "mode completion contains duplicates".to_owned(),
                ));
            }
        }
        Ok(())
    }

    pub fn to_json_pretty(&self) -> Result<Vec<u8>, SaveError> {
        self.validate()?;
        Ok(serde_json::to_vec_pretty(self)?)
    }

    pub fn from_json(bytes: &[u8]) -> Result<Self, SaveError> {
        let profile: Self = serde_json::from_slice(bytes)?;
        profile.validate()?;
        Ok(profile)
    }

    pub fn write_atomic(&self, path: &Path) -> Result<(), SaveError> {
        let bytes = self.to_json_pretty()?;
        let parent = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(parent)?;
        let mut temporary = NamedTempFile::new_in(parent)?;
        temporary.as_file_mut().write_all(&bytes)?;
        temporary.as_file().sync_all()?;
        temporary
            .persist(path)
            .map_err(|error| SaveError::Io(error.error))?;
        Ok(())
    }

    pub fn read(path: &Path) -> Result<Self, SaveError> {
        Self::from_json(&fs::read(path)?)
    }
}

fn validate_text(field: &str, value: &str, max_bytes: usize) -> Result<(), SaveError> {
    if value.is_empty() || value.len() > max_bytes || value.chars().any(char::is_control) {
        return Err(SaveError::Invalid(format!(
            "{field} must be non-empty, bounded, and printable"
        )));
    }
    Ok(())
}

fn validate_unique_text(field: &str, values: &[String], max_bytes: usize) -> Result<(), SaveError> {
    let mut seen = BTreeSet::new();
    for value in values {
        validate_text(field, value, max_bytes)?;
        if !seen.insert(value) {
            return Err(SaveError::Invalid(format!("{field} contains duplicates")));
        }
    }
    Ok(())
}

fn validate_unique_plants(field: &str, values: &[PlantType]) -> Result<(), SaveError> {
    let mut seen = BTreeSet::new();
    for plant in values {
        let slot = plant.slot();
        if slot >= 53 {
            return Err(SaveError::Invalid(format!(
                "{field} contains an unknown plant"
            )));
        }
        if !seen.insert(slot) {
            return Err(SaveError::Invalid(format!("{field} contains duplicates")));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> SaveProfile {
        let mut profile = SaveProfile::new("player-1");
        profile.settings = SaveSettings {
            music_volume_percent: 70,
            effects_volume_percent: 80,
            fullscreen: true,
            logical_scale: 2,
        };
        profile.unlocked_plants = vec![PlantType::Peashooter, PlantType::Sunflower];
        profile.awards = vec!["FirstSun".to_owned()];
        profile.inventory = SaveInventory {
            coins: 125,
            seed_packets: vec![PlantType::Peashooter],
        };
        profile.garden.plants.push(GardenPlant {
            plant_type: PlantType::Sunflower,
            age_ticks: 240,
            watered: true,
        });
        profile.mode_completion.push(ModeCompletion {
            mode: ModeKind::Adventure,
            completed_levels: 3,
            endless_unlocked: false,
        });
        profile
    }

    #[test]
    fn profile_json_is_deterministic_and_round_trips() {
        let profile = fixture();
        let first = profile.to_json_pretty().unwrap();
        let second = profile.to_json_pretty().unwrap();
        assert_eq!(first, second);
        assert_eq!(SaveProfile::from_json(&first).unwrap(), profile);
    }

    #[test]
    fn rejects_future_versions_and_duplicate_entries() {
        let mut future = fixture();
        future.format_version = SAVE_FORMAT_VERSION + 1;
        assert!(matches!(
            future.validate(),
            Err(SaveError::UnsupportedFormat { .. })
        ));

        let mut duplicate = fixture();
        duplicate.unlocked_plants.push(PlantType::Peashooter);
        assert!(matches!(
            duplicate.validate(),
            Err(SaveError::Invalid(message)) if message.contains("duplicates")
        ));
    }

    #[test]
    fn writes_and_reads_an_atomic_profile_file() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("profile.json");
        let profile = fixture();

        profile.write_atomic(&path).unwrap();
        assert_eq!(SaveProfile::read(&path).unwrap(), profile);

        let replacement = SaveProfile::new("replacement");
        replacement.write_atomic(&path).unwrap();
        assert_eq!(SaveProfile::read(&path).unwrap(), replacement);
    }
}
