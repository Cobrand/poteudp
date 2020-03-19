use serde::{Serialize, Deserialize};

pub fn bincode_config() -> bincode::Config {
    let mut c = bincode::config();
    c.limit(1000);
    c
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    pub series_id: i32,
    pub message_id: i32,
    pub sender_id: String,
}