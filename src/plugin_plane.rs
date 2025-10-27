use std::time::Duration;

use bevy::color::palettes::tailwind::{BLUE_500, RED_400, YELLOW_200, YELLOW_500};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology}; // Plane track mesh
use bevy::render::render_asset::RenderAssetUsages;

use crate::ShareStruct;
use crate::math::*;
use crate::plugin_egui::*;

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
            show_tracks,
        ),
    );
}

#[derive(Resource)]
struct TimerResource(Timer);

#[derive(Component, Resource)]
pub struct Plane {
    pub hex: String,              // Plane hex-id
    pub pos: Vec<[f32; 3]>,       // Collects all [lat, lon, alt] to show flight path in Bevy coordinates
    pub track_id: Option<Entity>, // Bevy entity id of track
}

impl Plane {
    pub fn new(hex: String) -> Plane {
        Plane {
            hex,
            pos: Vec::new(),
            track_id: None,
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
                let pos = read_tmp.get_latest_known_pos(plane_id.to_string());

                if pos.is_some() {
                    let lat = pos.unwrap().0;
                    let lon = pos.unwrap().1;
                    let lat1 = map_range(lat, 50.0, 55.0, 1000.0, -1000.0);
                    let lon1 = map_range(lon, 5.0, 10.0, -1000.0, 1000.0);
                    let height = pos.unwrap().2;

                    // TODO: Distribute scale factor and clarify magic 0.3048
                    let scale = 0.00361;
                    plane.0.translation = Vec3::new(lon1, height * scale * 0.3048, lat1); // What was 0.3048 again?

                    // Save position, to show flight path
                    plane.1.pos.push([lon1, height * scale * 0.3048, lat1]);
                }

                // Rotate plane
                if let Some(track) = read_tmp.get_track(plane_id.to_string()) {
                    // Real degree to bevy degree
                    let new_track: f32 = (180.0 - track + 360.0) % 360.0;
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
    asset_server: Res<AssetServer>,
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
                second_child: Entity::PLACEHOLDER,
            };

            commands
                .spawn((
                    Plane::new(plane_id_tmp),
                    SceneRoot(
                        asset_server
                            .load(GltfAssetLabel::Scene(0).from_asset("planes/plane-b.glb")),
                    ),
                    // TODO: Scale planes according to zoom level and other
                    Transform::from_xyz(-1000.0, 0.0, 0.0).with_scale(Vec3::splat(1.5)),
                ))
                .with_children(|parent| {
                    child_entities.first_child = parent
                        .spawn((
                            Mesh3d(meshes.add(Sphere { radius: 0.5 })),
                            MeshMaterial3d(materials.add(Color::Srgba(RED_400))),
                            Transform::from_xyz(-2.0, 0.0, 0.0),
                        ))
                        .id();
                    child_entities.second_child = parent
                        .spawn((
                            Mesh3d(meshes.add(Sphere::new(0.5))),
                            MeshMaterial3d(materials.add(Color::Srgba(BLUE_500))),
                            Transform::from_xyz(2.0, 0.0, 0.0),
                        ))
                        .id();
                });
        }
    }
}

pub fn update_route(read: Res<ShareStruct>, mut gizmos: Gizmos, ui_state: Res<UiState>) {
    let read_tmp = read.0.lock().unwrap();
    let list = read_tmp.get_planes_id();

    // TODO: Distribute map ranges
    for plane in list {
        for plane_data in read_tmp.get_latest_known_pos(plane.to_string()) {
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

            // TODO: Clean up this mess
            if ui_state.plane_checkbox.contains_key(&plane.to_string()) {
                let &checked = ui_state.plane_checkbox.get(&plane.to_string()).unwrap();
                if checked {
                    let ant_lat1 = map_range(53.5718392, 50.0, 55.0, 1000.0, -1000.0);
                    let ant_lon1 = map_range(9.9834842, 5.0, 10.0, -1000.0, 1000.0);
                    let scale = 0.00361;

                    gizmos.line(
                        Vec3::new(lon1, plane_data.2 * scale * 0.3048, lat1),
                        Vec3::new(ant_lon1, 1.0 * scale * 0.3048, ant_lat1),
                        YELLOW_200,
                    );
                }
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
    mut ui_state: ResMut<UiState>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let mut read_tmp = read.0.lock().unwrap();
        for plane_id in query.iter_mut() {
            // Plane 'lifetime' if unseen
            // TODO: Setup time in egui
            if read_tmp.get_last_seen(plane_id.1.hex.clone()) >= 60 {
                // Remove Bevy entity
                commands.entity(plane_id.0).despawn();
                // Remove from Egui ui state
                ui_state.plane_checkbox.remove(&plane_id.1.hex.clone());
                // Remove shared data
                read_tmp.remove_plane(plane_id.1.hex.clone());
                // Remove track
                plane_id
                    .1
                    .track_id
                    .map(|track| commands.entity(track).despawn());
            }
        }
    }
}

pub fn show_tracks(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Plane)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ui_state: ResMut<UiState>,
) {
    for mut plane_id in query.iter_mut() {
        // Spawn track if plane is selected in egui and has not been built
        if plane_id.1.pos.len() >= 4 {
            if *ui_state.selected(plane_id.1.hex.as_str()) && plane_id.1.track_id == None {
                // Build mesh only if useful

                let all_pos = plane_id.1.pos.clone();
                let mesh = plane_track_mesh(all_pos.clone());
                let id = commands
                    .spawn((
                        Mesh3d(meshes.add(mesh)),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::srgb(0.2, 0.7, 0.9),
                            cull_mode: None,
                            double_sided: true,
                            unlit: true,
                            ..Default::default()
                        })),
                    ))
                    .id();
                plane_id.1.track_id = Some(id);
            } else if !*ui_state.selected(plane_id.1.hex.as_str()) && plane_id.1.track_id.is_some()
            {
                plane_id
                    .1
                    .track_id
                    .map(|track| commands.entity(track).despawn());
                plane_id.1.track_id = None;
            }
        }
    }
}

fn plane_track_mesh(positions: Vec<[f32; 3]>) -> Mesh {
    let ground: f32 = 0.0; // lower y-pos

    let mut all_positions = positions.clone();
    let mut indices: Vec<u32> = Vec::new();
    //let mut normals = Vec::new();
    //let mut uvs = Vec::new();

    // Add projection to ground
    for i in &positions {
        let g_pos = [i[0], ground, i[2]];
        all_positions.push(g_pos)
    }

    let half = all_positions.len() / 2;

    // Top triangles
    for i in 0..half - 1 {
        indices.push(i as u32);
        indices.push((i + 1) as u32);
        indices.push((i + half) as u32);
    }

    // Bottom triangles
    for j in half + 1..all_positions.len() {
        indices.push(j as u32);
        indices.push((j - 1) as u32);
        indices.push((j - half) as u32);
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, all_positions);
    //mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    //mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh.insert_indices(Indices::U32(indices));
    mesh.compute_normals();
    mesh
}
