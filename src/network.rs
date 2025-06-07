use chrono::{NaiveDate, NaiveTime};
use std::io;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};

use crate::data_share::*;

// async fn network_connect(dst_address: &str) -> io::Result<TcpStream> {
//     loop {
//         match TcpStream::connect(dst_address).await {
//             Ok(stream) => Ok(stream),
//             Err(err) => {
//                 eprintln!("Network connection error: {}", err);
//                 sleep(Duration::from_secs(5)).await;
//                 continue;
//             }
//         }
//     }
// }

pub async fn connect_dump1090_sbs(data_share: Arc<Mutex<SharedDataDb>>) -> Result<(), Box<dyn std::error::Error>>  {
    // Define the address and port where dump1090 is serving data and connect to server
   
        let addr = "127.0.0.1:30003";
        //let stream = network_connect(addr).await.unwrap();
        // TODO: Check for errors
        let stream = TcpStream::connect(addr).await.unwrap();
        //println!("Connected to dump1090 at {}", addr);

        // Wrap the stream in a buffered reader for line-by-line processing
        let reader = BufReader::new(stream);
        let mut lines = reader.lines();

        // Process each line of data received from dump1090
        while let line = lines.next_line().await {
            match line {
                Ok(Some(message)) => {
                    // println!("ORIG: {:?}", message);

                    // Split message into 22 pieces
                    // TODO: Check len
                    let vec: Vec<&str> = message.split(',').collect();

                    if vec.len() == 22 {
                        // Map each field to needed data type
                        // TODO: Move decoding to fn in decode.rs
                        // TODO: More sophisticated decoding
                        let tmp_transmission_type = vec[1].parse::<usize>().ok().unwrap();
                        let tmp_generated_date =
                            NaiveDate::parse_from_str(vec[6], "%Y/%m/%d").ok().unwrap();
                        let tmp_generated_time = NaiveTime::parse_from_str(vec[7], "%H:%M:%S%.f")
                            .ok()
                            .unwrap();
                        let tmp_logged_date =
                            NaiveDate::parse_from_str(vec[8], "%Y/%m/%d").ok().unwrap();
                        let tmp_logged_time = NaiveTime::parse_from_str(vec[9], "%H:%M:%S%.f")
                            .ok()
                            .unwrap();
                        let tmp_altitude = vec[11].parse::<f32>().ok();
                        let tmp_ground_speed = vec[12].parse::<f32>().ok();
                        let tmp_track = vec[13].parse::<f32>().ok();
                        let tmp_latitude = vec[14].parse::<f32>().ok();
                        let tmp_longitude = vec[15].parse::<f32>().ok();
                        let tmp_vertical_rate = vec[16].parse::<f32>().ok();
                        let tmp_squawk = vec[17].parse::<i32>().ok();
                        let tmp_alert = vec[18].parse::<usize>().unwrap_or(0);
                        let mut tmp_alert_bool = false;
                        if tmp_alert == 1 {
                            tmp_alert_bool = true;
                        }
                        let tmp_emergency = vec[19].parse::<usize>().unwrap_or(0);
                        let mut tmp_emergency_bool = false;
                        if tmp_emergency == 1 {
                            tmp_emergency_bool = true;
                        }
                        let tmp_spi = vec[20].parse::<usize>().unwrap_or(0);
                        let mut tmp_spi_bool = false;
                        if tmp_spi == 1 {
                            tmp_spi_bool = true;
                        }
                        let tmp_is_on_ground = vec[21].parse::<usize>().unwrap_or(0);
                        let mut tmp_is_on_ground_bool = false;
                        if tmp_is_on_ground == 1 {
                            tmp_is_on_ground_bool = true;
                        }

                        let mut data_tmp = data_share.lock().unwrap();
                        data_tmp.update_data(
                            Some(vec[2].to_string()),  // SessionID
                            Some(vec[3].to_string()),  // AircraftID
                            vec[4].to_string(),        // HexIdent
                            Some(vec[5].to_string()),  // FlightID
                            Some(vec[10].to_string()), // CallSign
                            vec[0].to_string(),        // MessageType
                            tmp_transmission_type,
                            tmp_generated_date,
                            tmp_generated_time,
                            tmp_logged_date,
                            tmp_logged_time,
                            tmp_altitude,
                            tmp_ground_speed,
                            tmp_track,
                            tmp_latitude,
                            tmp_longitude,
                            tmp_vertical_rate,
                            tmp_squawk,
                            Some(tmp_alert_bool),
                            Some(tmp_emergency_bool),
                            Some(tmp_spi_bool),
                            Some(tmp_is_on_ground_bool),
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Error reading line. Error: {:?}", e);
                    break;
                }
                _ => {
                    eprintln!("Connection lost.");
                }
            }
        }
    Ok(())
}
