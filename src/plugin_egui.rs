use crate::ShareStruct;

use bevy::prelude::*;
use bevy_egui::{EguiContextPass, EguiContexts, EguiPlugin, egui};
use bevy_egui::egui::{Color32, RichText};
use crate::squawks::{is_emergency_squawk, get_transponder_description};

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
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_systems(EguiContextPass, ui_system);
}

fn ui_system(mut contexts: EguiContexts, read: Res<ShareStruct>, mut ui_state: ResMut<UiState>) {
    let read_tmp = read.0.lock().unwrap();
    let plane_list = read_tmp.get_planes_id();
    let number_of_planes = plane_list.len();

    let heading = format!("Planes ({number_of_planes})");

    egui::Window::new("Luftraum").show(contexts.ctx_mut(), |ui| {
        // Settings section
        ui.collapsing("Settings", |ui| {
            ui.checkbox(
                &mut ui_state.pos_ground_projection,
                "Project position to ground",
            );
            ui.checkbox(&mut ui_state.pos_ground_arrow, "Arrow position to ground");
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
                        let height_level_option = read_tmp.get_latest_pos(plane_id.to_string());
                        let mut height_level = "-".to_string();
                        if let Some(height_level_option) = height_level_option {
                            height_level = height_level_option.2.to_string();
                        }

                        // Speed over ground
                        let ground_speed_option = read_tmp.get_ground_speed(plane_id.to_string());
                        let mut ground_speed = "-".to_string();
                        if let Some(ground_speed_option) = ground_speed_option {
                            ground_speed = ground_speed_option.to_string();
                        }

                        // Track
                        let track_option = read_tmp.get_track(plane_id.to_string());
                        let mut track = "-".to_string();
                        if let Some(track_option) = track_option {
                            track = track_option.to_string();
                        }
                        
                        // Call sign
                        let call_sign = read_tmp.get_call_sign(plane_id.to_string());

                        // Is on ground
                        let mut on_ground_str = "-".to_string();
                        if let Some(is_on_ground) = read_tmp.is_on_ground(plane_id.to_string()) {
                            if is_on_ground {
                                on_ground_str = "on ground".to_string();
                            }
                        }
                        
                        // Build row
                        ui.label(plane_id);
                        ui.label(RichText::new(squawk_str).color(color));
                        ui.label(height_level);
                        ui.label(ground_speed);
                        ui.label(track);
                        ui.label(call_sign);
                        ui.label(on_ground_str);
                        ui.end_row();
                    }
                });
            });
    });
}
