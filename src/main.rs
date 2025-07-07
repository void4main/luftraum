use std::sync::{Arc, Mutex};
use bevy::prelude::*;

use crate::network::*;
use crate::data_share::SharedDataDb;

mod network;
mod decode;
mod setup;
mod plugin_egui;
mod plugin_plane;
mod srtm;
pub mod math;
mod terrain;
mod terrain_color_spectrum;
mod sbs;
mod data_share;
mod plugin_airspace;
mod squawks;
mod logging;
//mod plugin_groundstructure;

#[derive(Resource)]
struct ShareStruct(Arc<Mutex<SharedDataDb>>);

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tokio::main]
async fn main() {

    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    
    // Create struct to store all plane data and share it between the network and bevy tasks.
    let plane_data_db = SharedDataDb::new();
    let shared_plane_data_db = Arc::new(Mutex::new(plane_data_db));
    let tokio_plane_data_db = shared_plane_data_db.clone();
    let bevy_plane_data_db = shared_plane_data_db.clone();
    
    // Receive ADS-B data from dump1090
    tokio::spawn(async move {
        let _a = connect_dump1090_sbs(&tokio_plane_data_db).await;
    });
    
    // Set application name
    let app_window = Some(Window {title: "Luftraum".into(), ..default()});
    
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: app_window,
            ..default()
        }))
        .insert_resource(ShareStruct(bevy_plane_data_db))
        .add_plugins(plugin_egui::plugin)       // egui
        .add_plugins(setup::plugin)             // camera, basic landscape, support gizmos
        .add_plugins(plugin_plane::plugin)      // plane related, setup, updates
        // .add_plugins(plugin_airspace::plugin)   // static airspace structures
        .run();

}

