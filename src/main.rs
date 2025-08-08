mod packet;
use packet::Note;
use serde::Deserialize;
use std::io::{BufReader, BufWriter};
use std::fs::{write, File, OpenOptions};
use bincode::{serialize_into, deserialize_from};

fn main() -> std::io::Result<()>{
    let logs = vec![
        Note::new("title", "body"),
        Note::new("title", "body"),
        Note::new("title", "body"),
        Note::new("title", "body"),
    ];

    let file = File::create("log.bin")?;
    let mut writer = BufWriter::new(file);
    serialize_into(&mut writer, &logs).expect("error to serilize");

    let file = File::open("log.bin")?;
    let mut reader = BufReader::new(file);
    let deserialize_code :Vec<Note> = deserialize_from(&mut reader).expect("error to deserilize");

    println!("error");
    for log in deserialize_code.iter().filter(|l| l.title == "hi") {
        println!("{:?}", log);
    }

    Ok(())
}