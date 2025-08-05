use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogPacket {
    pub level: String,
    pub message: String,
    pub timestamp: String
}

impl LogPacket{
    pub fn new(level:&str, message: &str, timestamp:&str) -> Self{
        Self { 
               level: "Info".to_string(),
               message: message.to_string(), 
               timestamp: timestamp.to_string() 
            }
    }
}