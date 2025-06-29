use crate::ShareStruct;
use bevy::prelude::*;
use bevy_egui::{EguiContextPass, EguiContexts, EguiPlugin, egui};

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
    // TODO: Beautify code
    let read_tmp = read.0.lock().unwrap();
    let plane_list = read_tmp.get_planes_id();
    let number_of_planes = plane_list.len().to_string().parse::<i32>().unwrap();
    
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
                        let mut squawk_str= "-".to_string();
                        
                        if let Some(squawk) = read_tmp.get_squawk(plane_id.to_string()) {
                            squawk_str = squawk.to_string();
                        }
                        let height_level_option = read_tmp.get_latest_pos(plane_id.to_string());
                        let mut height_level = "-".to_string();
                        if let Some(height_level_option) = height_level_option {
                            height_level = height_level_option.2.to_string();
                        }
                        let call_sign = read_tmp.get_call_sign(plane_id.to_string());
                        
                        ui.label(plane_id);
                        ui.label(squawk_str);
                        ui.label(height_level);
                        ui.label(call_sign);
                        ui.end_row();
                    }
                });
            });
        
        
    });
}
