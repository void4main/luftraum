use bevy::prelude::*;

pub enum ColorSpectrum {
    /// Type of pre-defined color spectrum to color different height-level.
    // TODO: Check accuracy
    Imhof,
    ImhofModified,
}

pub fn get_height_color(height_m: f32, colors: ColorSpectrum) -> [f32; 4] {
    match colors {
        ColorSpectrum::Imhof => {
            // Imhof colors with modifications
            // Unit of measure is meter
            // Height 0 is always blue for convenience
            match height_m {
                height if height < 0.0 => {
                    Color::srgb(0.05, 0.125, 0.075).to_linear().to_f32_array()
                }
                height_m if height_m >= 0.0 && height_m < 100.0 => {
                    Color::srgb(0.654, 0.772, 0.541).to_linear().to_f32_array()
                }
                height_m if height_m >= 100.0 && height_m < 200.0 => {
                    Color::srgb(0.753, 0.863, 0.634).to_linear().to_f32_array()
                }
                height_m if height_m >= 200.0 && height_m < 500.0 => {
                    Color::srgb(0.882, 0.879, 0.624).to_linear().to_f32_array()
                }
                height_m if height_m >= 500.0 && height_m < 1000.0 => {
                    Color::srgb(0.855, 0.783, 0.592).to_linear().to_f32_array()
                }
                height_m if height_m >= 1000.0 && height_m < 2000.0 => {
                    Color::srgb(0.829, 0.743, 0.576).to_linear().to_f32_array()
                }
                height_m if height_m >= 2000.0 && height_m < 4000.0 => {
                    Color::srgb(0.754, 0.643, 0.523).to_linear().to_f32_array()
                }
                height_m if height_m >= 4000.0 && height_m < 9000.0 => {
                    Color::srgb(0.677, 0.546, 0.473).to_linear().to_f32_array()
                }
                _ => Color::srgb(1.0, 1.0, 1.0).to_linear().to_f32_array(),
            }
        }
        ColorSpectrum::ImhofModified => {
            // Imhof colors with modifications
            // Unit of measure is meter
            // Height 0 is always blue for convenience
            match height_m {
                height if height < 0.0 => {
                    Color::srgb(0.05, 0.125, 0.075).to_linear().to_f32_array()
                }
                height if height == 0.0 => {
                    Color::srgb(0.025, 0.075, 0.275).to_linear().to_f32_array()
                }
                height if height > 0.0 && height < 100.0 => {
                    Color::srgb(0.654, 0.772, 0.541).to_linear().to_f32_array()
                }
                height if height_m >= 100.0 && height_m < 200.0 => {
                    Color::srgb(0.753, 0.863, 0.634).to_linear().to_f32_array()
                }
                height_m if height_m >= 200.0 && height_m < 500.0 => {
                    Color::srgb(0.882, 0.879, 0.624).to_linear().to_f32_array()
                }
                height_m if height_m >= 500.0 && height_m < 1000.0 => {
                    Color::srgb(0.855, 0.783, 0.592).to_linear().to_f32_array()
                }
                height_m if height_m >= 1000.0 && height_m < 2000.0 => {
                    Color::srgb(0.829, 0.743, 0.576).to_linear().to_f32_array()
                }
                height_m if height_m >= 2000.0 && height_m < 4000.0 => {
                    Color::srgb(0.754, 0.643, 0.523).to_linear().to_f32_array()
                }
                height_m if height_m >= 4000.0 && height_m < 9000.0 => {
                    Color::srgb(0.677, 0.546, 0.473).to_linear().to_f32_array()
                }
                _ => Color::srgb(1.0, 1.0, 1.0).to_linear().to_f32_array(),
            }
        }
    }
}