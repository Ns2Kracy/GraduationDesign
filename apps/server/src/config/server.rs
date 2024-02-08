use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Server {
    pub fn url(&self) -> String {
        format!("{}:{}", &self.host, &self.port)
    }
}
