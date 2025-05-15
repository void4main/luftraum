use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use crate::{SharedData};
use crate::decode::*;

pub async fn connect_dump1090_sbs(data_share: SharedData) -> Result<(), Box<dyn std::error::Error>> {
    // Define the address and port where dump1090 is serving data
    let addr = "127.0.0.1:30003"; // Replace with the actual address and port of dump1090

    // Connect to the dump1090 TCP server
    let stream = TcpStream::connect(addr).await?;
    println!("Connected to dump1090 at {}", addr);

    // Wrap the stream in a buffered reader for line-by-line processing
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    // Process each line of data received from dump1090
    while let line = lines.next_line().await {
        match line {
            Ok(Some(message)) => {
                println!("ORIG: {:?}", message);
                if let Some(message) = decode_message_sbs(message) {
                    println!("Write message: {:?}", message);
                    let mut data = data_share.0.lock().unwrap();
                    data.push((message.0, message.1, message.2));
                };
            }
            Err(e) => {
                eprintln!("Error reading line. Error: {:?}", e);
                break;
            }
            _ => {
                println!("Connection rest by peer.");
                break;
            },
        }
    }
    Ok(())
}

pub async fn connect_dump1090_bo(data_share: SharedData) -> Result<(), Box<dyn std::error::Error>> {
    // Define the address and port where dump1090 is serving data
    // dump1090 --device-type hackrf --net-bo-port 30003 --interactive
    let addr = "127.0.0.1:30005";

    // Connect to the dump1090 TCP server
    let stream = TcpStream::connect(addr).await?;
    println!("Connected to dump1090 at {}", addr);

    // Wrap the stream in a buffered reader for line-by-line processing
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    // Process each line of data received from dump1090
    while let line = lines.next_line().await {
        match line {
            Ok(Some(message)) => {
                // Print or process the received message
                println!("Bo message: {:?}", message);
                // if let Some(message) = decode_message_sbs(message) {
                //     println!("Write message: {:?}", message);
                //     let mut data = data_share.0.lock().unwrap();
                //     data.push((message.0, message.1, message.2));
                // };
            }
            Err(e) => {
                eprintln!("Error reading line. Error: {:?}", e);
                break;
            }
            _ => {
                println!("Connection rest by peer.");
                break;
            },
        }
    }
    Ok(())
}