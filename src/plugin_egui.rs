use crate::ShareStruct;
use crate::data_share::*;

use crate::squawks::get_transponder_description;
use bevy::prelude::*;

use bevy_egui::egui::{Color32, RichText};

use bevy_egui::{
    EguiContexts, EguiGlobalSettings, EguiMultipassSchedule, EguiPlugin, EguiPrimaryContextPass,
    EguiUserTextures, PrimaryEguiContext, egui
};

struct PlaneImages {
    plane_up: Handle<Image>,
    plane_level: Handle<Image>,
    plane_down: Handle<Image>,
}

impl FromWorld for PlaneImages {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            plane_up: asset_server.load("planes/plane_egui_t_up.png"),
            plane_level: asset_server.load("planes/plane_egui_t.png"),
            plane_down: asset_server.load("planes/plane_egui_t_down.png"),
        }
    }
}

#[derive(Default, Resource)]
pub struct UiState {
    label: String,
    value: f32,
    inverted: bool,
    egui_texture_handle: Option<egui::TextureHandle>,
    pub pos_ground_projection: bool,
    pub pos_ground_arrow: bool,
}

pub fn plugin(app: &mut App) {
    app.init_resource::<UiState>()
        .insert_resource(UiState::default())
        .add_plugins(EguiPlugin::default())
        .add_systems(EguiPrimaryContextPass, ui_system);
}

fn ui_system(
    mut contexts: EguiContexts,
    read: Res<ShareStruct>,
    mut ui_state: ResMut<UiState>,
    mut is_initialized: Local<bool>,
    mut rendered_texture_id: Local<egui::TextureId>,
    plane_images: Local<PlaneImages>,
) {
    let read_tmp = read.0.lock().unwrap();
    let plane_list = read_tmp.get_planes_id();
    let number_of_planes = plane_list.len();

    if !*is_initialized {
        *is_initialized = true;
        *rendered_texture_id = contexts.add_image(plane_images.plane_down.clone_weak());
    }

    let heading = format!("Planes ({number_of_planes})");

    egui::Window::new("Luftraum").show(contexts.ctx_mut().expect("egui-show().error"), |ui| {
        // Settings section
        ui.collapsing("Settings", |ui| {
            ui.checkbox(
                &mut ui_state.pos_ground_projection,
                "Project position to ground",
            );
            ui.checkbox(&mut ui_state.pos_ground_arrow, "Arrow position to ground");
        });

        // Settings section
        ui.collapsing("Statistics", |ui| {
            ui.label("Hello");
        });

        // List all planes
        egui::CollapsingHeader::new(heading)
            .default_open(true)
            .show(ui, |ui| {
                egui::Grid::new("some_unique_id").show(ui, |ui| {
                    for plane_id in plane_list.clone() {

                        // Squawk
                        let squawk;
                        let mut squawk_str = "-".to_string();
                        let mut color = Color32::GRAY;
                        if let Some(squawk_tmp) = read_tmp.get_squawk(plane_id.to_string()) {
                            squawk = squawk_tmp;
                            if let Some(squawk) = get_transponder_description(squawk) {
                                color = squawk.1.to_color32();
                            }
                            squawk_str = squawk_tmp.to_string();
                        }

                        // Height level
                        let height_level = read_tmp
                            .get_latest_pos(plane_id.to_string())
                            .map(|pos| pos.2.to_string())
                            .unwrap_or("-".to_string());

                        // Speed over ground
                        let ground_speed = read_tmp
                            .get_ground_speed(plane_id.to_string())
                            .map(|speed| speed.to_string())
                            .unwrap_or("-".to_string());

                        // Track
                        let track = read_tmp
                            .get_track(plane_id.to_string())
                            .map(|t| t.to_string())
                            .unwrap_or("-".to_string());

                        // Call sign
                        let call_sign = read_tmp.get_call_sign(plane_id.to_string());

                        // Is on ground
                        let on_ground_str = read_tmp
                            .is_on_ground(plane_id.to_string())
                            .filter(|&is_on_ground| is_on_ground)
                            .map(|_| "on ground".to_string())
                            .unwrap_or("-".to_string());

                        // Vertical rate
                        let vertical_rate = read_tmp
                            .get_vertical_rate(plane_id.to_string())
                            .unwrap_or_default()
                            .to_string();

                        // Vertical rate
                        let vertical_rate_str =
                            read_tmp.get_simple_vertical_rate(plane_id.to_string());

                        // Distance to antenna
                        // Antennenposition 53.5718392,9.9834842
                        let dist_to_antenna = read_tmp.get_plane_distance_to_lat_lon(plane_id.to_string(), 53.5718392, 9.9834842).unwrap_or(0.0);

                        let rate_description = read_tmp.get_vertical_rate_description(plane_id.to_string());
                        let mut bevy_icon_handle: Handle<Image>;
                        // match rate_description {
                        //     VerticalRate::UpFast => { bevy_icon_handle = images.plane_icon_up.clone_weak() },
                        //     VerticalRate::Up => { bevy_icon_handle = images.plane_icon_up.clone_weak() },
                        //     VerticalRate::Down => { bevy_icon_handle = images.plane_icon_down.clone_weak() },
                        //     VerticalRate::DownFast => { bevy_icon_handle = images.plane_icon_down.clone_weak() },
                        //     _ => { bevy_icon_handle = images.plane_icon.clone_weak() }
                        // }

                        // Build row
                        ui.label(plane_id);
                        ui.label(RichText::new(squawk_str).color(color));
                        ui.label(height_level);
                        ui.label(ground_speed);
                        ui.label(track);
                        ui.label(call_sign);
                        ui.label(on_ground_str);
                        ui.label(vertical_rate);
                        ui.label(vertical_rate_str);
                        ui.label(format!("{:05.1}", dist_to_antenna));

                        // ui.add(egui::widgets::Image::new(
                        //     *rendered_texture_id,
                        //     [256.0, 256.0],
                        // ));

                        ui.end_row();
                    }
                });
            });
    });
}
