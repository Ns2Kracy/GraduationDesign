use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Postgresql {
    pub address: String,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Postgresql {
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.username, self.password, self.address, self.database
        )
    }
}
