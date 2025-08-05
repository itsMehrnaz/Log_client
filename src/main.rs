mod packet;
use std::io::stdin;
use std::io::Write;
use packet::LogPacket;
use serde::{Serialize, Deserialize};
use std::fs::write;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn write_on_file(log: LogPacket) -> std::io::Result<()> {
    let encoded = bincode::serialize(&log).unwrap();

    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("logs.bin")?;

    file.write_all(&encoded)?;
    file.write_all(b"/n")?;

    Ok(())

}

fn read_logs() -> std::io::Result<Vec<LogPacket>>{
    let file = File::open("logs.bin")?;
    let reader = BufReader::new(file);

    let mut logs = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let bytes = line.as_bytes();
        if let Ok(log) = bincode::deserialize::<LogPacket>(bytes){
            logs.push(log);
        }
    }

    Ok(logs)
}

fn main() -> std::io::Result<()> {
    let log = LogPacket::new("Info", "System started", "2025-07-31");

    write_on_file(log)?;

    let logs = read_logs()?;
    for l in logs {
        println!("{:?}", l);
    }

    Ok(())
}
