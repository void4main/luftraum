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

fn ui_system(
    mut contexts: EguiContexts,
    read: Res<ShareStruct>,
    mut ui_state: ResMut<UiState>,
) {
    // TODO: Beautify code
    let read_tmp = read.0.lock().unwrap();
    // TODO: Clone to end lock?
    let plane_list = read_tmp.get_planes_id();
    let number_of_planes = plane_list.len().to_string().parse::<i32>().unwrap();
    let heading = format!("Planes ({number_of_planes})");
    egui::Window::new("Luftraum").show(contexts.ctx_mut(), |ui| {
        // List all planes
        ui.collapsing(heading, |ui| {
            for plane_id in plane_list {
                ui.label(plane_id);
            }
        });
        
        // Settings section
        ui.collapsing("Settings", |ui| {
            ui.checkbox(&mut ui_state.pos_ground_projection, "Project position to ground");
            ui.checkbox(&mut ui_state.pos_ground_arrow, "Arrow position to ground");
        });
        
    });
}
