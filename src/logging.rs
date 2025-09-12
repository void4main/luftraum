use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::time::SystemTime;

/// Logs all received ADB-S data to a static file.
pub fn log_messages(message: &str) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("raw_messages.log")?;

    let mut writer = BufWriter::new(file);
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    writeln!(writer, "{}, {}", timestamp, message)?;
    writer.flush()?;

    Ok(())
}