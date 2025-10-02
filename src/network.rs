use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};
use rumqttc::{Event, Incoming, MqttOptions, AsyncClient, QoS};   // MQTT client

use super::{MqttBroker, SbsServer};
use crate::data_share::*;
use crate::decode::decode_message_sbs;
use crate::logging::log_messages;

pub async fn connect_dump1090_sbs(
    data_share: &Arc<Mutex<SharedDataDb>>, sbs_server: SbsServer,
) -> Result<(), Box<dyn std::error::Error>> {

    // Define the address and port where dump1090 is serving data and connect to server
    let addr = format!("{}:{}", sbs_server.sbs_hostname, sbs_server.sbs_port);

    loop {
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

        // TODO: Log connection information or screen/egui indicator?
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
                    let _ = log_messages("sbs", &message);
                    // Decode message and store it in struct
                    // println!("Label: {}", sbs_server.label);
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
    // TODO: Fix this, can't be reached
    Ok(())
}

/// Connect to MQTT broker to pull ADS-B messages
pub async fn connect_mqtt(data_share: &Arc<Mutex<SharedDataDb>>, mqtt_broker: MqttBroker
) -> Result<(), Box<dyn std::error::Error>> {

    // TODO: Read from file
    let broker = mqtt_broker.mqtt_broker_hostname;
    let port = mqtt_broker.mqtt_broker_port;
    let keepalive = mqtt_broker.mqtt_keepalive; // Seconds
    let topic = mqtt_broker.mqtt_topic;
    let username = mqtt_broker.mqtt_user;
    let password = mqtt_broker.mqtt_password;
    println!("Label: {}", mqtt_broker.label);
    let mut mqtt_options = MqttOptions::new("rumqtt-async", broker, port);
    mqtt_options.set_credentials(username, password);
    mqtt_options.set_keep_alive(Duration::from_secs(keepalive));

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
    client.subscribe(topic, QoS::AtMostOnce).await.unwrap();

    loop {

        match eventloop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                // println!("Topic: {}, Payload: {:?}", p.topic, p.payload);
                let message = str::from_utf8(p.payload.as_ref());
                match message {
                Ok(message) => {
                    // Log everything to file by now, message is the raw data set
                    let _ = log_messages(&p.topic, &message);
                    // Decode message and store it in struct
                    let _ = decode_message_sbs(data_share, message.parse().unwrap());
                }
                Err(e) => {
                    eprintln!("Error reading message. Error: {:?}", e);}
                }

            }
            Ok(Event::Incoming(i)) => {
                println!("Incoming = {i:?}");
            }
            Ok(Event::Outgoing(o)) => println!("Outgoing {:?}", o),    // Subscribe is in here as well ...
            Err(e) => {
                println!("Error = {e:?}");
                return Ok(());
            }
        }
    }

    Ok(())
}