use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegisterCommand {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserRegisterCommandResult {
    pub id: i32,
}