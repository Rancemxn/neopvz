use std::path::PathBuf;

#[derive(Clone, Debug, Default)]
pub struct PlatformConfig {
    pub data_override: Option<PathBuf>,
}

pub fn platform_name() -> &'static str {
    std::env::consts::OS
}
