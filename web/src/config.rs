use anyhow::Result;
use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub port: u16,
}

impl Settings {
    pub fn parse() -> Result<Self> {
        let settings = Config::builder()
            .add_source(File::new("config/dev.yaml", FileFormat::Yaml))
            .build()?;

        Ok(settings.try_deserialize()?)
    }
}
