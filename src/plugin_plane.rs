use bevy::prelude::*;
use std::time::Duration;
use std::f32::consts::PI;
use crate::math::*;
use crate::{ShareStruct};

#[derive(Resource)]
struct TimerResource(Timer);

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_plane)
        .insert_resource(TimerResource(Timer::new(Duration::from_secs(10), TimerMode::Repeating)))
        .add_systems(Update, (update_planes_data, update_planes, list_plane_ids));
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
    commands.spawn((
        Plane::new("aacc".to_string()),
        Mesh3d(meshes.add(Capsule3d::new(1.2, 0.3))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(-1.0, 2.0, 1.5).with_rotation(Quat::from_rotation_y(PI / 3.0)),
    ));
}

// Update all planes positions
pub fn update_planes(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Plane>>,
    mut gizmos: Gizmos,
) {
    for mut transform in query.iter_mut() {
        // Circle around center
        transform.translation.z = ops::sin(time.elapsed_secs()) * 2.0;
        transform.translation.x = ops::cos(time.elapsed_secs()) * 2.0;
        //let angle = player_transform.rotation.to_euler(EulerRot::ZYX).0;
        let angle = transform.rotation.x;
        let m_test =
            angle_rad_between(transform.translation.x, transform.translation.z, 0.0, -10.0);
        transform.rotate_y(angle - m_test);
    }
}

pub fn update_planes_data(mut query: Query<&mut Plane, With<Plane>>) {
    if query.is_empty() {
        return;
    };
    for mut plane_query in query.iter_mut() {
        plane_query.heading = Some(140.0);
    }
}

pub fn update_route(
    read: Res<ShareStruct>,
    mut gizmos: Gizmos
) {
    // let mut data = read.0.lock().unwrap();
    // for i in data.iter() {
    //     let lat1 = map_range(i.0, 50.0, 55.0, 1000.0, -1000.0);
    //     let lon1 = map_range(i.1, 5.0, 10.0, -1000.0, 1000.0);
    //     let scale = 0.00361;
    //     gizmos.cross(Vec3::new(lon1, i.2 * scale * 0.3048, lat1), 1.0, RED_400);
    // }
}

fn list_plane_ids(mut commands: Commands, 
                  time: Res<Time>, 
                  mut timer: ResMut<TimerResource>,
                  read: Res<ShareStruct>) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let read_tmp = read.0.lock().unwrap();
        read_tmp.get_planes_id();
    }
}