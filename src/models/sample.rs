use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sample {
    pub id: i32,
    pub name: String,
}
