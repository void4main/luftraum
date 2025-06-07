use std::time::Duration;

use bevy::color::palettes::tailwind::{RED_400, YELLOW_500};
use bevy::prelude::*;

use crate::ShareStruct;
use crate::math::*;
use crate::plugin_egui::*;

#[derive(Resource)]
struct TimerResource(Timer);

pub fn plugin(app: &mut App) {
    //app.add_systems(Startup, spawn_plane)
    app.insert_resource(TimerResource(Timer::new(
        Duration::from_secs(10),
        TimerMode::Repeating,
    )))
    .add_systems(
        Update,
        (
            create_planes,
            update_planes,
            update_route,
            increase_plane_last_seen,
            despawn_planes,
        ),
    );
}

#[derive(Component, Debug)]
pub struct Plane {
    pub hex: String,
    pub mode: Option<String>,
    pub squawk: Option<i32>,
    pub flight: Option<String>,
    pub altitude: Option<f32>,
    pub speed: Option<f32>,
    pub heading: Option<f32>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub rssi: Option<f32>,
}
impl Plane {
    pub fn new(hex: String) -> Plane {
        Plane {
            hex,
            mode: None,
            squawk: None,
            flight: None,
            altitude: None,
            speed: None,
            heading: None,
            latitude: None,
            longitude: None,
            rssi: None,
        }
    }
}

// Create update all planes positions
pub fn update_planes(mut query: Query<(&mut Transform, &mut Plane)>, read: ResMut<ShareStruct>) {
    // TODO: Beautify code
    let read_tmp = read.0.lock().unwrap();
    let plane_list: Vec<String> = read_tmp
        .get_planes_id()
        .iter()
        .map(|&s| s.to_string())
        .collect();

    // Get all existing plane_id in database
    for plane_id in plane_list {
        'inner: for mut plane in query.iter_mut() {
            if plane_id == plane.1.hex {
                // Update position if all Some has data
                let pos = read_tmp.get_latest_pos(plane_id.to_string());

                if pos.is_some() {
                    let lat = pos.unwrap().0;
                    let lon = pos.unwrap().1;
                    let lat1 = map_range(lat, 50.0, 55.0, 1000.0, -1000.0);
                    let lon1 = map_range(lon, 5.0, 10.0, -1000.0, 1000.0);
                    let height = pos.unwrap().2;

                    // TODO: Distribute scale factor
                    let scale = 0.00361;
                    plane.0.translation = Vec3::new(lon1, height * scale * 0.3048, lat1); // What was 0.3048 again?
                }
                break 'inner;
            }
        }
    }
}

pub fn create_planes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&mut Transform, &Plane)>,
    read: Res<ShareStruct>,
) {
    let read_tmp = read.0.lock().unwrap();
    let plane_list = read_tmp.get_planes_id();
    let spawned_list = query
        .iter()
        .map(|e| e.1.hex.clone())
        .collect::<Vec<String>>();

    for plane_id in plane_list {
        let plane_id_tmp = String::from(plane_id);
        if !spawned_list.contains(&plane_id_tmp) {
            commands.spawn((
                Plane::new(plane_id_tmp),
                Mesh3d(meshes.add(Capsule3d::new(1.0, 1.0))),
                MeshMaterial3d(materials.add(Color::WHITE)),
                Transform::from_xyz(-1000.0, 0.0, 0.0),
            ));
        }
    }
}

pub fn update_route(read: Res<ShareStruct>, mut gizmos: Gizmos, ui_state: Res<UiState>) {
    let read_tmp = read.0.lock().unwrap();
    let list = read_tmp.get_planes_id();

    // TODO: Distribute map ranges
    for plane in list {
        for plane_data in read_tmp.get_latest_pos(plane.to_string()) {
            let lat1 = map_range(plane_data.0, 50.0, 55.0, 1000.0, -1000.0);
            let lon1 = map_range(plane_data.1, 5.0, 10.0, -1000.0, 1000.0);
            // TODO: Distribute scale factor
            let scale = 0.00361;
            gizmos.cross(
                Vec3::new(lon1, plane_data.2 * scale * 0.3048, lat1),
                5.0,
                RED_400,
            );
            // Indicate ground location
            if ui_state.pos_ground_projection {
                gizmos.cross(Vec3::new(lon1, 0.0, lat1), 5.0, RED_400);
            }
            
            if ui_state.pos_ground_arrow {
                gizmos.arrow(
                    Vec3::new(lon1, 0.0, lat1),
                    Vec3::new(lon1, plane_data.2 * scale * 0.3048, lat1),
                    YELLOW_500,
                );
            }
        }
    }
}

fn increase_plane_last_seen(
    time: Res<Time>,
    mut timer: ResMut<TimerResource>,
    read: Res<ShareStruct>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let mut read_tmp = read.0.lock().unwrap();
        read_tmp.increase_last_seen(); // Increase by 10
    }
}

fn despawn_planes(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<TimerResource>,
    mut query: Query<(Entity, &Plane)>,
    read: Res<ShareStruct>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let mut read_tmp = read.0.lock().unwrap();
        for plane_id in query.iter_mut() {
            // Plane 'lifetime' if unseen
            // TODO: Setup in egui
            if read_tmp.get_last_seen(plane_id.1.hex.clone()) > 60 {
                read_tmp.remove_plane(plane_id.1.hex.clone());
                commands.entity(plane_id.0).despawn();
            }
        }
    }
}
