use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use crate::math::*;
use crate::plugin_plane::*;
use crate::srtm::*;
use crate::terrain_color_spectrum::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(PanOrbitCameraPlugin)
        .add_plugins(WireframePlugin { debug_flags: Default::default() })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_planes, support_structures, toggle_wireframe),
        );
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Camera
    commands.spawn((
        (
            Camera3d::default(),
            Transform::from_xyz(0., 34.5, 12.).looking_at(Vec3::ZERO, Vec3::Y),
        ),
        PanOrbitCamera::default(),
    ));

    let size_dataset_row: u32 = 6000; // TODO: Determine from somewhere

    // Terrain
    let srtm_data = import_srtm(size_dataset_row as usize, 0);
    let sub_divisions = get_num_subdevisions(srtm_data.num_cols as u32) * 2; // TODO: Why * 2.0?
    let terrain_width = 2000.0;
    let terrain_height = 2000.0;

    // Build mesh
    let mut terrain = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(terrain_width, terrain_height)
            .subdivisions(sub_divisions),
    );

    // Transform heights of mesh
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        // TODO: Get data from file
        let pix_meter = get_pix_m(1.0, size_dataset_row as usize, 0.0008333, terrain_width);
        let scale = pix_meter;
        for pos in positions.iter_mut().enumerate() {
            pos.1[1] = srtm_data.terrain_data[pos.0] * scale;
        }
    
        // Add colour scheme
        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[_, g, _]| get_height_color(*g / scale, ColorSpectrum::ImhofModified))
            .collect();
        terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        terrain.compute_normals();
    }

    // Spawn terrain
    commands.spawn((
        Mesh3d(meshes.add(terrain)),
        MeshMaterial3d(materials.add(StandardMaterial {
            ..Default::default()
        })),
        Terrain,
    ));
    //
    //
    //
    // Terrain 2, TODO: Terrain sizes and stitching etc. etc.
    // 
    //
    //
    //
    let srtm_data = import_srtm(size_dataset_row as usize, 1);
    let sub_divisions = get_num_subdevisions(srtm_data.num_cols as u32) * 2; // TODO: Why * 2.0?
    let terrain_width = 2000.0;
    let terrain_height = 2000.0;

    // Build mesh
    let mut terrain = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(terrain_width, terrain_height)
            .subdivisions(sub_divisions),
    );

    // Transform heights of mesh
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        // TODO: Get data from file
        let pix_meter = get_pix_m(1.0, size_dataset_row as usize, 0.0008333, terrain_width);
        let scale = pix_meter;
        for pos in positions.iter_mut().enumerate() {
            pos.1[1] = srtm_data.terrain_data[pos.0] * scale;
        }

        // Add colour scheme
        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[_, g, _]| get_height_color(*g / scale, ColorSpectrum::ImhofModified))
            .collect();
        terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        terrain.compute_normals();
    }

    // Spawn terrain
    commands.spawn((
        Mesh3d(meshes.add(terrain)),
        MeshMaterial3d(materials.add(StandardMaterial {
            ..Default::default()
        })),
        Transform::from_xyz(2000.0, 0.0, 0.0),
        Terrain,
    ));
    
    
    
}

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
    
    // Fuji Yama
    // 35.361865, 138.732045
    // let lat1 = map_range(35.361865, 35.0, 40.0, 1000.0, -1000.0);
    // let lon1 = map_range(138.732045, 135.0, 140.0, -1000.0, 1000.0);
    // let scale = 0.00361;
    // gizmos.cross(Vec3::new(lon1, 3776.24 * scale, lat1), 15.5, WHITE);
}

#[derive(Component)]
pub struct Terrain;

pub fn toggle_wireframe(
    mut commands: Commands,
    landscapes_wireframes: Query<Entity, (With<Terrain>, With<Wireframe>)>,
    landscapes: Query<Entity, (With<Terrain>, Without<Wireframe>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    
    if input.just_pressed(KeyCode::Space) {
        for terrain in &landscapes {
            commands.entity(terrain).insert(Wireframe);
        }
        for terrain in &landscapes_wireframes {
            commands.entity(terrain).remove::<Wireframe>();
        }
    }
}