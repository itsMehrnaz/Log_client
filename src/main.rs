mod packet;
use packet::Note;
use std::io::{BufReader, BufWriter};
use std::fs::{write, File, OpenOptions};
use bincode::{serialize_into, deserialize_from};

fn main() -> std::io::Result<()>{
    let logs = vec![
        Note::new("title", "body")
    ];

    let file = File::create("log.bin")?;
    let writer = BufWriter::new(file);
}