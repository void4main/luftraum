use std::sync::{Arc, Mutex};
use bevy::prelude::*;

use crate::network::*;
use crate::data_share::SharedDataDb;

mod network;
mod decode;
mod setup;
mod plugin_egui;
mod plugin_plane;
mod plugin_datatransfer;
mod srtm;
mod math;
mod terrain;
mod terrain_color_spectrum;
mod sbs;
mod data_share;
mod plugin_airspace;
mod plugin_groundstructures;

#[derive(Resource)]
struct ShareStruct(Arc<Mutex<SharedDataDb>>);

#[tokio::main]
async fn main() {
    
    // Create struct to store all plane data and share it
    let plane_data_db = SharedDataDb::new();
    let shared_plane_data_db = Arc::new(Mutex::new(plane_data_db));
    let tokio_plane_data_db = shared_plane_data_db.clone();
    let bevy_plane_data_db = shared_plane_data_db.clone();
    
    // Receive ADS-B data from dump1090
    tokio::spawn(async move {
        let _a = connect_dump1090_sbs(tokio_plane_data_db).await;
    });
    
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ShareStruct(bevy_plane_data_db))
        .add_plugins(plugin_egui::plugin)       // egui
        .add_plugins(setup::plugin)             // camera, basic landscape, support gizmos
        .add_plugins(plugin_plane::plugin)      // plane related, setup, updates
        .add_plugins(plugin_airspace::plugin)   // static airspace structures
        .run();

}

