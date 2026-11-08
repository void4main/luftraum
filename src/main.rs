use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use std::process;
use bevy::prelude::*;

use crate::configuration::*;
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
mod plugin_aircraft;
//mod sbs;
mod setup;
mod squawks;
mod srtm;
mod terrain;
mod terrain_color_spectrum;
mod plugin_sound;
mod configuration;
mod plugin_cursor;
mod plugin_messages;

struct TerrainVariables {
    scale: f32,     // meter in pix
    min_lat: f32,
    max_lat: f32,
    min_lon: f32,
    max_lon: f32,
    windows_x_min: f32,
    windows_x_max: f32,
    windows_y_min: f32,
    windows_y_max: f32,
}
static TERRAIN_SETUP: OnceLock<Mutex<TerrainVariables>> = OnceLock::new();


#[derive(Resource)]
struct ShareStruct(Arc<Mutex<SharedDataDb>>);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration from file
    let cfg = load_configuration("luftraum_config.toml");
    let config = cfg.unwrap_or_else(|err| {
        error!("Error loading 'luftraum_config.toml': {}", err);
        eprintln!("Error loading 'luftraum_config.toml', {}", err);
        process::exit(1);
    });

    // Temp
    // dump_aircraft_data_from_cache()?;
    
    // Create struct to store all aircraft data and share it between the network and bevy tasks.
    let plane_data_db = SharedDataDb::new();
    let shared_plane_data_db = Arc::new(Mutex::new(plane_data_db));
    let bevy_plane_data_db = shared_plane_data_db.clone();

    // Create struct to store additional aircraft data, e.g. aircraft type
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
            let tokio_aircraft_data_db_mqtt = shared_plane_data_db.clone();
            tokio::spawn(async move {
                let _ = connect_mqtt(&tokio_aircraft_data_db_mqtt, mqtt_broker).await;
            });
        }
    }

    // Set application name and run bevy and plugins
    let app_window = Some(Window {
        ..default()
    });

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: app_window,
            ..default()
        }))
        .insert_resource(ShareStruct(bevy_plane_data_db))   
        .add_plugins(setup::plugin)             // camera, basic landscape, support gizmos
        .add_plugins(plugin_cursor::plugin)     // Mouse cursor projection to ground
        .add_plugins(plugin_egui::plugin)       // egui
        .add_plugins(plugin_aircraft::plugin)   // aircraft related, setup, updates
        .add_plugins(plugin_sound::plugin)      // notifications
        //.add_plugins(plugin_messages::plugin)
        // .add_plugins(plugin_airspace::plugin)          // static airspace structures, e.g. no flight zones
        //.add_plugins(plugin_ground_structures::plugin)
        .run();

    Ok(())
}