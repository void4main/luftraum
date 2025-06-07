use std::f32::consts::{FRAC_PI_2, PI};
use bevy::app::{App, Update};
use bevy::color::palettes::tailwind::{ORANGE_600, YELLOW_500};
use bevy::math::{Vec3};
use bevy::prelude::{Gizmos, Isometry3d, Quat};
use crate::math::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        no_flight_zones,
    );
}

fn no_flight_zones(mut gizmos: Gizmos) {
    // Flugverbotszone 53°33'10, 009°59'38 Radius ca. 3,7km bis Flugfläche FL100 (ca. 3.000 Meter)
    // = 53.5523507,9.9913814 ???
    
    let lat1 = map_range(53.5523507, 50.0, 55.0, 1000.0, -1000.0);
    let lon1 = map_range(9.9913814, 5.0, 10.0, -1000.0, 1000.0);
    let scale = 0.00361;
    
    gizmos.arrow(
        Vec3::new(lon1, 0.0, lat1),
        Vec3::new(lon1, 3000.0 * scale * 0.3048, lat1),
        YELLOW_500,
    );
    
    let radius = get_pix_m(3600.0, 6000, 0.00083, 2000.0);
    
    for i in 1..4 {
        gizmos.circle(Isometry3d::new(Vec3::new(lon1, 1.0 + i as f32, lat1), Quat::from_rotation_x(FRAC_PI_2)), 10.0, ORANGE_600).resolution(64);
        gizmos.circle(Isometry3d::new(Vec3::new(lon1, 1.0 + i as f32, lat1), Quat::from_rotation_x(FRAC_PI_2)), radius, ORANGE_600).resolution(64);
    }
    
}
