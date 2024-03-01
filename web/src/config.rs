use std::path::PathBuf;

use anyhow::Result;
use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub deploy: DeploymentSettings,
    pub app: AppSettings,
}

#[derive(Deserialize)]
pub struct DeploymentSettings {
    pub addr: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct AppSettings {
    pub public_dir: PathBuf,
    pub blogs_dir: PathBuf,
}

impl Settings {
    pub fn parse() -> Result<Self> {
        let settings = Config::builder()
            .add_source(File::new("config/dev.yaml", FileFormat::Yaml))
            .build()?;

        Ok(settings.try_deserialize()?)
    }
}
