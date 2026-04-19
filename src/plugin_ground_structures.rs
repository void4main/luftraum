// Everything static on ground level, e.g. airports, static antenna positions, city markers.

use bevy::color::palettes::tailwind::{GREEN_400, PURPLE_600, YELLOW_200};
use bevy::math::Vec3;
use bevy::prelude::Gizmos;
use crate::math::map_range;

// TODO: Place in plugin_groundstructures or/and load from file/db
pub fn support_structures(mut gizmos: Gizmos) {
    // Antennenposition 53.5718392,9.9834842
    let lat1 = map_range(53.5718392, 50.0, 55.0, 1000.0, -1000.0);
    let lon1 = map_range(9.9834842, 5.0, 10.0, -1000.0, 1000.0);
    let scale = 0.00361;
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