use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub body: String
}

impl Note  {
    fn new(title: &str, body: &str) -> Self{
        Self { title: title.to_string(),
               body: body.to_string() 
            }
    }
}
