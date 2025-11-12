use std::error::Error;
use std::fs;
use serde::Deserialize;
use crate::network::{MqttBroker, SbsServer};

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub(crate) sbs_server: Option<Vec<SbsServer>>,
    pub(crate) mqtt_broker: Option<Vec<MqttBroker>>,
    //terrain_tile_size: TerrainTileSize,
    //terrain_srtm_file: Vec<TerrainSrtmFile>,
}

pub fn load_configuration(path: &str) -> bevy::prelude::Result<Configuration, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    let cfg: Configuration = toml::from_str(&raw)?;
    Ok(cfg)
}