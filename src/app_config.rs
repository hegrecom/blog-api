use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub password_salt: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Config::builder()
            .add_source(File::new("app_config.toml", FileFormat::Toml))
            .build()
            .unwrap()
            .try_deserialize::<AppConfig>()
            .unwrap()
    }
}

