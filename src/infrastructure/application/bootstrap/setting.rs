use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub database: String,
    pub nats: String
}

impl Settings {
    pub fn new() -> Self {
        todo!()
    }
}