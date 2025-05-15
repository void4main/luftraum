use std::sync::{Arc, Mutex};
use bevy::prelude::*;
use crate::network::*;
use crossbeam::channel;

mod network;
mod decode;
mod setup;
mod plugin_plane;
mod plugin_datatransfer;
mod srtm;
mod math;
mod terrain;
mod terrain_colorspectrum;

struct DataShare {
    lat: f32,
    lon: f32,
    height: f32,
}
#[derive(Resource)]
struct SharedData(Arc<Mutex<Vec<(f32, f32, f32)>>>);

#[derive(Resource)]
struct Channel(channel::Receiver<i32>);

#[tokio::main]
async fn main() {
    let (sender, receiver) = channel::unbounded();
    let shared_data = Arc::new(Mutex::new(Vec::<(f32, f32, f32)>::new()));
    let tokio_shared_data = shared_data.clone();
    tokio::spawn(async move {
        sender.send(202).unwrap();
        // let _a = connect_dump1090_sbs(SharedData(tokio_shared_data)).await;
        let _a = connect_dump1090_sbs(SharedData(tokio_shared_data)).await;
        
    });
    
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SharedData(shared_data.clone()))
        .insert_resource(Channel(receiver)) // Add the crossbeam receiver as a resource
        .add_plugins(setup::plugin) // camera, basic landscape, support gizmos
        .add_plugins(plugin_plane::plugin) // plane related, setup, updates
        .run();

}
