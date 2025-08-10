use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LogPacket {
    pub level: String,
    pub message: String,
    pub timestamp: String
}

impl LogPacket {
    pub fn new(level: &str, message: &str, timestamp: &str) -> Self{
        Self 
        { 
            level: level.to_string(),
            message: message.to_string(),
            timestamp: timestamp.to_string()
        }
    }
}