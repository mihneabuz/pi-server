use std::path::PathBuf;

use anyhow::Result;
use config::{Config, Environment, File, FileFormat};
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
pub enum StaticFileCompression {
    None,
    Gzip,
}

#[derive(Deserialize)]
pub struct AppSettings {
    pub public_dir: PathBuf,
    pub compression: StaticFileCompression,
    pub blogs_dir: PathBuf,
    pub rate_limit: RateLimitSettings,
    pub conn_limit: u32
}

#[derive(Deserialize)]
pub struct RateLimitSettings {
    pub max_allowed: u32,
    pub per_seconds: u32,
}

impl Settings {
    pub fn parse(file: &str) -> Result<Self> {
        let settings = Config::builder()
            .add_source(File::new(file, FileFormat::Yaml))
            .add_source(Environment::default().separator("_"))
            .build()?;

        Ok(settings.try_deserialize()?)
    }
}
