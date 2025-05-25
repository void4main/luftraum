use std::f32::consts::PI;
use std::time::Duration;

use bevy::color::palettes::tailwind::RED_400;
use bevy::prelude::*;
use bevy_panorbit_camera::FocusBoundsShape::{Cuboid, Sphere};
use crate::ShareStruct;
use crate::math::*;

#[derive(Resource)]
struct TimerResource(Timer);

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_plane)
        .insert_resource(TimerResource(Timer::new(
            Duration::from_secs(10),
            TimerMode::Repeating,
        )))
        .add_systems(
            Update,
            (create_planes, update_planes, list_plane_ids, update_route),
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

    pub fn set_latitude(&mut self, latitude: Option<f32>) {
        self.latitude = latitude;
    }
}

pub fn spawn_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn((
    //     Plane::new("aacc".to_string()),
    //     Mesh3d(meshes.add(Capsule3d::new(1.2, 0.3))),
    //     MeshMaterial3d(materials.add(Color::WHITE)),
    //     Transform::from_xyz(-1.0, 2.0, 1.5).with_rotation(Quat::from_rotation_y(PI / 3.0)),
    // ));
    // commands.spawn((
    //     Plane::new("bbbb".to_string()),
    //     Mesh3d(meshes.add(Capsule3d::new(1.2, 0.3))),
    //     MeshMaterial3d(materials.add(Color::WHITE)),
    //     Transform::from_xyz(-1.0, 4.0, 1.5).with_rotation(Quat::from_rotation_y(PI / 3.0)),
    // ));
}

// Create update all planes positions
// TODO: Divide create and update etc.
pub fn update_planes(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut Transform, Entity, &mut Plane)>,
    read: Res<ShareStruct>,
) {
    // TODO: Beautify code
    let read_tmp = read.0.lock().unwrap();
    // TODO: Clone to end lock?
    let plane_list = read_tmp.get_planes_id();

    // Get all existing plane_id in database
    for plane_id in plane_list {
        let plane_id_temp = String::from(plane_id);
        //println!("plane_id: {}", plane_id);
        'inner: for mut plane in query.iter_mut() {
            if plane_id_temp == plane.2.hex {
                //println!("Update plane {:?}", plane_id);
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
                    plane.0.translation = Vec3::new(lon1, height * scale * 0.3048, lat1);
                }
                break 'inner;
            }
        }
    }
}
pub fn create_planes(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut Transform, Entity, &mut Plane)>,
    read: Res<ShareStruct>,
) {
    let read_tmp = read.0.lock().unwrap();
    let plane_list = read_tmp.get_planes_id();
    let spawned_list = query
        .iter()
        .map(|e| e.2.hex.clone())
        .collect::<Vec<String>>();

    for plane_id in plane_list {
        let plane_id_tmp = String::from(plane_id);
        if !spawned_list.contains(&plane_id_tmp) {
            //println!("Create plane {:?}", plane_id);
            commands.spawn((
                Plane::new(plane_id_tmp),
                Mesh3d(meshes.add(Capsule3d::new(1.0, 1.0))),
                MeshMaterial3d(materials.add(Color::WHITE)),
                Transform::from_xyz(-1000.0, 0.0, 0.0),
            ));
        }
    }
}

pub fn update_route(read: Res<ShareStruct>, mut gizmos: Gizmos) {
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
        }
    }
}

fn list_plane_ids(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<TimerResource>,
    read: Res<ShareStruct>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let read_tmp = read.0.lock().unwrap();
        let list = read_tmp.get_planes_id();
        for plane_id in list {
            let plane_id_string = plane_id.to_string();
            // Data dump to see what's going on
            // println!(
            //     "PlaneId: {:?} - Pos: {:?}",
            //     plane_id,
            //     read_tmp.get_latest_pos(plane_id_string)
            // );
        }
    }
}
