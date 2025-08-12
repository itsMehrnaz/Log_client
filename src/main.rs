use chrono::{DateTime, Utc};

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
