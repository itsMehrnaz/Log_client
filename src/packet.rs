use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogPacket {
    pub level: String,
    pub message: String,
    pub timestamp: String
}