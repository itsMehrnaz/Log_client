use chrono::{DateTime, Utc};
use tokio::{fs::File, io::{AsyncBufReadExt, BufReader}};


#[derive(Debug)]
struct SyslogMessage {
    facility: u8,
    severity: u8,
    timestamp: DateTime<Utc>,
    message: String,
}

fn parse_syslog_line(line: &str) -> Option<SyslogMessage> {
    if let Some(end_idx) = line.find('>') {
        let pri_str = &line[1..end_idx];
        let pri: u8 = pri_str.parse().ok()?;
        let facility = pri / 8;
        let severity = pri % 8;

        let rest = &line[end_idx + 1..];
        let mut parts = rest.splitn(2, ' ');

        let timestamp_str = parts.next()?;
        let timestamp = DateTime::parse_from_rfc3339(timestamp_str).ok()?.with_timezone(&Utc);

        let message = parts.next()?.to_string();

        Some(SyslogMessage {
            facility,
            severity,
            timestamp,
            message,
        })
    } else {
        None
    }
}



#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let file = File::open("/tmp/test.log").await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if let Some(msg) = parse_syslog_line(&line) {
            println!("{:?}", msg);
        } else {
            println!("خط نامعتبر: {}", line);
        }
    }

    Ok(())
}
