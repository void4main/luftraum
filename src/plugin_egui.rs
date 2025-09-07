use crate::ShareStruct;
use crate::plugin_plane::Plane;
use crate::squawks::get_transponder_description;
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use bevy_egui::egui::{Color32, RichText};
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use std::collections::{HashMap, HashSet};

#[derive(Default, Resource)]
pub struct UiState {
    pub plane_ids: HashSet<String>,
    pub pos_ground_projection: bool,
    pub pos_ground_arrow: bool,
    // TODO: Move statistics calculations
    pub max_distance_to_antenna: f32,
    pub min_vertical_rate: f32,
    pub max_vertical_rate: f32,
    pub min_height_level: Option<f32>,
    pub max_height_level: f32,
    pub min_speed: Option<f32>,
    pub max_speed: f32,
    // Checkboxes
    pub plane_checkbox: HashMap<String, bool>,
}

impl UiState {
    // Check if a plane is selected
    pub fn selected(&mut self, key: &str) -> &mut bool {
        self.plane_checkbox.entry(key.to_string()).or_insert(false)
    }

    pub fn add_plane(&mut self, key: &str) {
        self.plane_checkbox.insert(key.to_string(), false);
    }

    pub fn rm_plane(&mut self, key: &str) {
        self.plane_checkbox.remove(key);
    }

}

pub fn plugin(app: &mut App) {
    app.init_resource::<UiState>()
        .insert_resource(UiState::default())
        .add_plugins(EguiPlugin::default())
        .add_systems(EguiPrimaryContextPass, ui_system);
}

fn ui_system(mut contexts: EguiContexts, read: Res<ShareStruct>, mut ui_state: ResMut<UiState>) {
    let read_tmp = read.0.lock().unwrap();
    let plane_list = read_tmp.get_planes_id();
    let number_of_planes = plane_list.len();

    let heading = format!("Planes ({number_of_planes})");

    egui::Window::new("Luftraum").show(contexts.ctx_mut().expect("egui-show().error"), |ui| {
        // Settings section
        ui.collapsing("Settings", |ui| {
            ui.checkbox(
                &mut ui_state.pos_ground_projection,
                "Project position to ground",
            );
            ui.checkbox(
                &mut ui_state.pos_ground_arrow,
                "Arrow position to ground");
        });

        // Statistics section
        ui.collapsing("Statistics", |ui| {
            let max_dist = ui_state.max_distance_to_antenna;
            let max_dist_label = format!("Max. distance to antenna: {:.1} km", max_dist);
            let min_vertical_rate = ui_state.min_vertical_rate;
            let min_vertical_rate_label =
                format!("Min. vertical rate: {:.1} fpm", min_vertical_rate);
            let max_vertical_rate = ui_state.max_vertical_rate;
            let max_vertical_rate_label =
                format!("Max. vertical rate: {:.1} fpm", max_vertical_rate);
            let planes_seen = ui_state.plane_ids.len();
            let planes_seen_label = format!("Planes seen: {}", planes_seen);
            let min_speed_for_label = ui_state
                .min_speed
                .map_or("-".to_string(), |speed| speed.to_string());
            let min_speed_label = format!("Min. speed: {:.1} kt", min_speed_for_label);
            let max_speed_label = format!("Max. speed: {:.1} kt", ui_state.max_speed);
            let min_height_level_for_label = ui_state
                .min_height_level
                .map_or("-".to_string(), |v| v.to_string());
            let min_height_level_label =
                format!("Min. height level: {} ft", min_height_level_for_label);
            let max_height_level_label =
                format!("Max. height level: {} ft", ui_state.max_height_level);

            ui.label(planes_seen_label);
            ui.label(max_dist_label);
            ui.label(min_speed_label);
            ui.label(max_speed_label);
            ui.label(min_vertical_rate_label);
            ui.label(max_vertical_rate_label);
            ui.label(min_height_level_label);
            ui.label(max_height_level_label);
        });

        // TODO: Push statistics calc to different place
        // List all planes
        egui::CollapsingHeader::new(heading)
            .default_open(true)
            .show(ui, |ui| {
                egui::Grid::new("some_unique_id").show(ui, |ui| {
                    // Headline
                    ui.centered_and_justified(|ui| {
                        ui.label(RichText::new("HEX").strong());
                    });
                    let labels = [
                        "Squawk", "Height", "Vertical", "Speed", "Track", "Call", "DTA",
                    ];
                    for label in labels {
                        //ui.label(RichText::new(label).strong());
                        ui.label(label);
                    }
                    ui.end_row();

                    // List of planes
                    for plane_id in plane_list.clone() {
                        // Statistics
                        ui_state.plane_ids.insert(plane_id.to_string());

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
                            .get_latest_known_pos(plane_id.to_string())
                            .map(|pos| pos.2.to_string())
                            .unwrap_or("-".to_string());

                        // Update statistics
                        if let Some(height_level) = read_tmp
                            .get_latest_known_pos(plane_id.to_string())
                            .map(|pos| pos.2)
                        {
                            if height_level > ui_state.max_height_level {
                                ui_state.max_height_level = height_level;
                            }
                            if height_level
                                < ui_state.min_height_level.map_or(40000.0, |value| value)
                            {
                                ui_state.min_height_level = Some(height_level);
                            }
                        }

                        // Speed over ground
                        let ground_speed = read_tmp
                            .get_ground_speed(plane_id.to_string())
                            .map(|speed| speed.to_string())
                            .unwrap_or("-".to_string());

                        if let Some(ground_speed) = read_tmp.get_ground_speed(plane_id.to_string())
                        {
                            if ground_speed > ui_state.max_speed {
                                ui_state.max_speed = ground_speed;
                            }
                            if ground_speed < ui_state.min_speed.map_or(1000.0, |value| value) {
                                ui_state.min_speed = Some(ground_speed);
                            }
                        }

                        // Track
                        let track = read_tmp
                            .get_track(plane_id.to_string())
                            .map(|t| t.to_string())
                            .unwrap_or("-".to_string());

                        // Call sign
                        let call_sign = read_tmp.get_call_sign(plane_id.to_string());

                        // Is on ground
                        // let on_ground_str = read_tmp
                        //     .is_on_ground(plane_id.to_string())
                        //     .filter(|&is_on_ground| is_on_ground)
                        //     .map(|_| "on ground".to_string())
                        //     .unwrap_or("-".to_string());

                        // Vertical rate
                        let vertical_rate = read_tmp
                            .get_vertical_rate(plane_id.to_string())
                            .unwrap_or_default();

                        // Update statistics
                        if vertical_rate < ui_state.min_vertical_rate {
                            ui_state.min_vertical_rate = vertical_rate;
                        }
                        if vertical_rate > ui_state.max_vertical_rate {
                            ui_state.max_vertical_rate = vertical_rate;
                        }
                        let vertical_rate_str = vertical_rate.to_string();

                        // Distance to antenna
                        // Antennenposition 53.5718392,9.9834842
                        // TODO: Fix static setup
                        let dist_to_antenna = read_tmp
                            .get_plane_distance_to_lat_lon(
                                plane_id.to_string(),
                                53.5718392,
                                9.9834842,
                            )
                            .unwrap_or(0.0);

                        // Update statistics
                        if dist_to_antenna > ui_state.max_distance_to_antenna {
                            ui_state.max_distance_to_antenna = dist_to_antenna.clone();
                        }

                        let checkbox_value = ui_state.selected(plane_id);

                        // Build row
                        ui.checkbox(
                            checkbox_value,
                            RichText::new(plane_id.to_string()).color(Color32::LIGHT_RED),
                        );
                        //ui.label(plane_id);
                        ui.label(RichText::new(squawk_str).color(color));
                        ui.label(height_level);
                        ui.label(vertical_rate_str);
                        //ui.label(vertical_rate_simple_str);
                        ui.label(ground_speed);
                        ui.label(track);
                        ui.label(call_sign);
                        // ui.label(on_ground_str);
                        ui.label(format!("{:05.1}", dist_to_antenna));
                        ui.end_row();
                    }
                });
            });
    });
}
