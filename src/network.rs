use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};

use crate::data_share::*;
use crate::decode::decode_message_sbs;
use crate::logging::log_messages;

pub async fn connect_dump1090_sbs(
    data_share: &Arc<Mutex<SharedDataDb>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Define the address and port where dump1090 is serving data and connect to server
    let addr = "127.0.0.1:30003";

    'network: loop {
        // Reconnect till connected
        let stream = loop {
            match TcpStream::connect(&addr).await {
                Ok(stream) => break stream,
                Err(e) => {
                    eprintln!("Connection to {}, error: {}", addr, e);
                    sleep(Duration::from_secs(5)).await;
                }
            }
        };

        // TODO: Check if connection is broken/lost
        println!("Connected to dump1090 at {}", addr);

        // Wrap the stream in a buffered reader for line-by-line processing
        let reader = BufReader::new(stream);
        let mut lines = reader.lines();

        // Process each line of data received from dump1090
        'read: loop {
            let line = lines.next_line().await;
            match line {
                Ok(Some(message)) => {
                    // Log everything to file by now, message is the raw data set
                    let _ = log_messages(&message);
                    // Decode message
                    let _ = decode_message_sbs(data_share, message);
                }
                Err(e) => {
                    eprintln!("Error reading line. Error: {:?}", e);
                }
                _ => {
                    eprintln!("Connection lost.");
                    sleep(Duration::from_secs(5)).await;
                    break 'read;
                }
            }
        }
    }
    Ok(())
}
