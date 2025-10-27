use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{error::Error, fs, process};

use crate::data_share::SharedDataDb;
use crate::hex_lookup::*;
use crate::network::*;

use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod data_share;
mod decode;
mod hex_lookup;
mod logging;
pub mod math;
mod network;
mod plugin_airspace;
mod plugin_antenna;
mod plugin_egui;
mod plugin_ground_structures;
mod plugin_plane;
//mod sbs;
mod setup;
mod squawks;
mod srtm;
mod terrain;
mod terrain_color_spectrum;
mod plugin_sound;

#[derive(Resource)]
struct ShareStruct(Arc<Mutex<SharedDataDb>>);

#[derive(Debug, Deserialize, Clone)]
struct Configuration {
    sbs_server: Option<Vec<SbsServer>>,
    mqtt_broker: Option<Vec<MqttBroker>>,
    //terrain_tile_size: TerrainTileSize,
    //terrain_srtm_file: Vec<TerrainSrtmFile>,
}

#[tokio::main]
async fn main() {
    // Load configuration from file
    let cfg = load_configuration("luftraum_config.toml");
    let config = cfg.unwrap_or_else(|err| {
        eprintln!("Error loading 'luftraum_config.toml': {}", err);
        process::exit(1);
    });

    // Create struct to store all aircraft data and share it between the network and bevy tasks.
    let plane_data_db = SharedDataDb::new();
    let shared_plane_data_db = Arc::new(Mutex::new(plane_data_db));
    let bevy_plane_data_db = shared_plane_data_db.clone();

    // Create struct to store additional aircraft data
    let aircraft_additional_data: HashMap<&str, Aircraft> = HashMap::new();
    let _shared_aircraft_additional_data = Arc::new(Mutex::new(aircraft_additional_data));

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
        .add_plugins(setup::plugin)             // camera, basic landscape, support gizmos
        .add_plugins(plugin_egui::plugin)       // egui
        .add_plugins(plugin_plane::plugin)      // plane related, setup, updates
        .add_plugins(plugin_sound::plugin)      //
        // .add_plugins(plugin_airspace::plugin)          // static airspace structures, e.g. no flight zones
        // .add_plugins(plugin_ground_structures::plugin)
        .run();
}

fn load_configuration(path: &str) -> Result<Configuration, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    let cfg: Configuration = toml::from_str(&raw)?;
    Ok(cfg)
}