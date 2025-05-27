use bevy::prelude::*;
use bevy_egui::{EguiContextPass, EguiContexts, EguiPlugin, egui};
use crate::ShareStruct;

pub fn plugin(app: &mut App) {
    app.add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: true,
    })
    .add_systems(EguiContextPass, ui_system);
}

pub fn ui_system(mut contexts: EguiContexts, read: Res<ShareStruct>) {
    // TODO: Beautify code
    let read_tmp = read.0.lock().unwrap();
    // TODO: Clone to end lock?
    let plane_list = read_tmp.get_planes_id();
    
    egui::Window::new("Luftraum").show(contexts.ctx_mut(), |ui| {
        ui.collapsing("Planes", |ui| {
            for plane_id in plane_list {
                ui.label(plane_id);
            }
        });
    });
    
}
