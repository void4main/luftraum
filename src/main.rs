use bevy::prelude::*;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::{error::Error, fs};

extern crate jemallocator;
use crate::data_share::SharedDataDb;
use crate::network::*;

mod data_share;
mod decode;
mod logging;
pub mod math;
mod network;
mod plugin_airspace;
mod plugin_antenna;
mod plugin_egui;
mod plugin_groundstructures;
mod plugin_plane;
mod sbs;
mod setup;
mod squawks;
mod srtm;
mod terrain;
mod terrain_color_spectrum;
mod hex_lookup;

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[derive(Resource)]
struct ShareStruct(Arc<Mutex<SharedDataDb>>);

#[derive(Debug, Deserialize, Clone)]
struct Configuration {
    sbs_server: Option<Vec<SbsServer>>,
    mqtt_broker: Option<Vec<MqttBroker>>,
    terrain_tile_size: TerrainTileSize,
    terrain_srtm_file: Vec<TerrainSrtmFile>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TerrainTileSize {
    terrain_width: f32,
    terrain_height: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TerrainSrtmFile {
    label: String,
    srtm_file: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SbsServer {
    pub label: String,
    pub sbs_hostname: String,
    pub sbs_port: u16,
}

impl SbsServer {
    fn validate(&self) -> Result<(), &'static str> {
        if self.sbs_port == 0 || self.sbs_port > 65535 {
            return Err("Configuration: Port out of range (1-65535)");
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct MqttBroker {
    pub label: String,
    pub mqtt_broker_hostname: String,
    pub mqtt_broker_port: u16,
    pub mqtt_topic: String,
    pub mqtt_user: String,
    pub mqtt_password: String,
    pub mqtt_keepalive: u64,
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    // Load configuration from file
    let cfg = load_configuration("luftraum_config.toml");
    dbg!(&cfg);

    // Create struct to store all plane data and share it between the network and bevy tasks.
    let plane_data_db = SharedDataDb::new();
    let shared_plane_data_db = Arc::new(Mutex::new(plane_data_db));
    let bevy_plane_data_db = shared_plane_data_db.clone();

    let config = cfg.unwrap();

    // Receive ADS-B data from dump1090
    for sbs_servers in config.clone().sbs_server.into_iter() {
        for sbs_server in sbs_servers {
            let tokio_plane_data_db_sbs = shared_plane_data_db.clone();
            tokio::spawn(async move {
                let _ = connect_dump1090_sbs(&tokio_plane_data_db_sbs, sbs_server).await;
            });
        }
    }

    // Receive ADS-B data from MQTT subscriptions
    for mqtt_brokers in config.mqtt_broker.into_iter() {
        for mqtt_broker in mqtt_brokers {
            let tokio_plane_data_db_mqtt = shared_plane_data_db.clone();
            tokio::spawn(async move {
                let _ = connect_mqtt(&tokio_plane_data_db_mqtt, mqtt_broker).await;
            });
        }
    }

    // Set application name
    let app_window = Some(Window {
        title: "Luftraum".into(),
        ..default()
    });

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: app_window,
            ..default()
        }))
        .insert_resource(ShareStruct(bevy_plane_data_db))
        .add_plugins(plugin_egui::plugin) // egui
        .add_plugins(setup::plugin) // camera, basic landscape, support gizmos
        .add_plugins(plugin_plane::plugin) // plane related, setup, updates
        // .add_plugins(plugin_airspace::plugin)   // static airspace structures
        .run();
}

fn load_configuration(path: &str) -> Result<Configuration, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    let cfg: Configuration = toml::from_str(&raw)?;
    // Validate each entry

        // if let Some(sbs_server) = srv.clone() {
        //     srv.validate()?;
        // }

    Ok(cfg)
}
