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
    pub fn load() -> anyhow::Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::with_name(&CONFIG_PATH))
            .build()?
            .try_deserialize()
    }
}
