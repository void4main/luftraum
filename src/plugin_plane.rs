use std::time::Duration;

use bevy::color::palettes::tailwind::{RED_400, YELLOW_500, BLUE_500};
use bevy::color::*;
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

// TODO: Rename example code
#[derive(Resource)]
struct ChildEntities {
    first_child: Entity,
    second_child: Entity,
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

                    // TODO: Distribute scale factor and clarify magic 0.3048
                    let scale = 0.00361;
                    plane.0.translation = Vec3::new(lon1, height * scale * 0.3048, lat1); // What was 0.3048 again?
                }

                if let Some(track) = read_tmp.get_track(plane_id.to_string()) {
                    // Real degree to bevy degree
                    let new_track: f32 = (180.0 - track + 360.0) % 360.0 ;
                    plane.0.rotation = Quat::from_rotation_y(new_track.to_radians());
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
    asset_server: Res<AssetServer>
) {
    let read_tmp = read.0.lock().unwrap();
    let plane_list = read_tmp.get_planes_id();
    let spawned_list = query
        .iter()
        .map(|e| e.1.hex.clone())
        .collect::<Vec<String>>();

    for plane_id in plane_list {
        let plane_id_tmp = String::from(plane_id);
        // Check is plane already exists
        if !spawned_list.contains(&plane_id_tmp) {
            // Beim Spawnen
            let mut child_entities = ChildEntities {
                first_child: Entity::PLACEHOLDER,
                second_child: Entity::PLACEHOLDER
            };

            commands.spawn((
                Plane::new(plane_id_tmp),
                SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("planes/plane-b.glb"))),
                Transform::from_xyz(-1000.0, 0.0, 0.0).with_scale(Vec3::splat(0.2)),
            )).with_children(|parent| {
                child_entities.first_child = parent.spawn((
                    Mesh3d(meshes.add(Sphere { radius: 0.5 })),
                    MeshMaterial3d(materials.add(Color::Srgba(RED_400))),
                    Transform::from_xyz(-2.0, 0.0, 0.0),
                )).id();

                child_entities.second_child = parent.spawn((
                    Mesh3d(meshes.add(Sphere::new(0.5))),
                    MeshMaterial3d(materials.add(Color::Srgba(BLUE_500))),
                    Transform::from_xyz(2.0, 0.0, 0.0),
                )).id();
            });
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
        read_tmp.increase_last_seen(10); // Increase by 10 sec
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
            if read_tmp.get_last_seen(plane_id.1.hex.clone()) >= 60 {
                read_tmp.remove_plane(plane_id.1.hex.clone());
                commands.entity(plane_id.0).despawn();
            }
        }
    }
}
