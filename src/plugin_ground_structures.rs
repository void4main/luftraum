// Everything static on ground level, e.g. airports, static antenna positions, city markers.

use crate::math::map_range;
use bevy::app::{App, Update};
use bevy::color::palettes::tailwind::{GREEN_400, PURPLE_600, YELLOW_200};
use bevy::math::Vec3;
use bevy::prelude::Gizmos;
use crate::TERRAIN_SETUP;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (support_structures,));
}

// TODO: Place in plugin_groundstructures or/and load from file/db
pub fn support_structures(mut gizmos: Gizmos) {
    let scale = TERRAIN_SETUP.get().unwrap().lock().unwrap().scale;
    let min_lat = TERRAIN_SETUP.get().unwrap().lock().unwrap().min_lat;
    let max_lat = TERRAIN_SETUP.get().unwrap().lock().unwrap().max_lat;
    let min_lon = TERRAIN_SETUP.get().unwrap().lock().unwrap().min_lon;
    let max_lon = TERRAIN_SETUP.get().unwrap().lock().unwrap().max_lon;

    // Antennenposition 53.5718392,9.9834842
    let lat1 = map_range(53.5718392, min_lat, max_lat, 1000.0, -1000.0);
    let lon1 = map_range(9.9834842, min_lon, max_lon, -1000.0, 1000.0);
    
    gizmos.cross(Vec3::new(lon1, 10.0 * scale, lat1), 15.5, PURPLE_600);

    // HH Flughafen
    // 53.6308882,9.9888915
    let lat1 = map_range(53.6308882, 50.0, 55.0, 1000.0, -1000.0);
    let lon1 = map_range(9.9888915, 5.0, 10.0, -1000.0, 1000.0);
    let scale = 0.00361;
    gizmos.cross(Vec3::new(lon1, 10.0 * scale, lat1), 15.5, GREEN_400);

    // HH Finkenwerder
    // 53.5351691,9.8381561
    let lat1 = map_range(53.5351691, 50.0, 55.0, 1000.0, -1000.0);
    let lon1 = map_range(9.8381561, 5.0, 10.0, -1000.0, 1000.0);
    let scale = 0.00361;
    gizmos.cross(Vec3::new(lon1, 10.0 * scale, lat1), 15.5, GREEN_400);

    // Hannover
    // 52.22,9.44
    let lat1 = map_range(52.22, 50.0, 55.0, 1000.0, -1000.0);
    let lon1 = map_range(9.44, 5.0, 10.0, -1000.0, 1000.0);
    let scale = 0.00361;
    gizmos.cross(Vec3::new(lon1, 10.0 * scale, lat1), 15.5, YELLOW_200);

    // Itzehoe
    // 53.9324022,9.4830417,
    let lat1 = map_range(53.9324022, 50.0, 55.0, 1000.0, -1000.0);
    let lon1 = map_range(9.4830417, 5.0, 10.0, -1000.0, 1000.0);
    let scale = 0.00361;
    gizmos.cross(Vec3::new(lon1, 10.0 * scale, lat1), 15.5, YELLOW_200);

    // Bremen
    // 53.1195743,8.4059783
    let lat1 = map_range(53.1195743, 50.0, 55.0, 1000.0, -1000.0);
    let lon1 = map_range(8.4059783, 5.0, 10.0, -1000.0, 1000.0);
    let scale = 0.00361;
    gizmos.cross(Vec3::new(lon1, 10.0 * scale, lat1), 15.5, YELLOW_200);

    // Fuji Yama
    // 35.361865, 138.732045
    // let lat1 = map_range(35.361865, 35.0, 40.0, 1000.0, -1000.0);
    // let lon1 = map_range(138.732045, 135.0, 140.0, -1000.0, 1000.0);
    // let scale = 0.00361;
    // gizmos.cross(Vec3::new(lon1, 3776.24 * scale, lat1), 15.5, WHITE);
}
