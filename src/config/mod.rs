pub mod postgresql;
pub mod server;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static CONFIG_PATH: Lazy<String> = Lazy::new(|| {
    let mut path = std::path::Path::new(".").canonicalize().unwrap();
    path.push("gd.toml");
    path.to_str().unwrap().to_string()
});
pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::load().expect("Failed to load config"));

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: server::Server,
    pub postgresql: postgresql::Postgresql,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        // check if config file exists
        if !std::path::Path::new(&*CONFIG_PATH).exists() {
            tracing::warn!("Config file not found, creating one at {}", &*CONFIG_PATH);
            let config = Self::default();
            std::fs::write(&*CONFIG_PATH, toml::to_string_pretty(&config)?)?;
            tracing::warn!("Config file created, please edit it before restarting the server");
            std::process::exit(0);
        }

        // load config file
        let config = toml::from_str(&std::fs::read_to_string(&*CONFIG_PATH)?)?;
        Ok(config)
    }
}
