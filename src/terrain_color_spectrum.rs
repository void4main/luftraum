use bevy::prelude::*;

pub enum ColorSpectrum {
    /// Type of pre-defined color spectrum to color different height-level.
    // TODO: Check accuracy
    //Imhof,
    ImhofModified,
}

// pub fn get_height_color(height_m: f32, colors: ColorSpectrum) -> [f32; 4] {
//     match colors {
//         ColorSpectrum::ImhofModified => {
//             // Imhof colors with modifications
//             // Unit of measure is meter
//             // Height 0 is always blue for convenience
//             match height_m {
//                 _height if height_m < 0.0 => {
//                     Color::srgb(0.05, 0.125, 0.075).to_linear().to_f32_array()
//                 }
//                 _height if height_m == 0.0 => {
//                     Color::srgb(0.025, 0.075, 0.275).to_linear().to_f32_array()
//                 }
//                 _height if height_m > 0.0 && height_m < 100.0 => {
//                     Color::srgb(0.654, 0.772, 0.541).to_linear().to_f32_array()
//                 }
//                 _height if height_m >= 100.0 && height_m < 200.0 => {
//                     Color::srgb(0.753, 0.863, 0.634).to_linear().to_f32_array()
//                 }
//                 _height if height_m >= 200.0 && height_m < 500.0 => {
//                     Color::srgb(0.882, 0.879, 0.624).to_linear().to_f32_array()
//                 }
//                 _height if height_m >= 500.0 && height_m < 1000.0 => {
//                     Color::srgb(0.855, 0.783, 0.592).to_linear().to_f32_array()
//                 }
//                 _height if height_m >= 1000.0 && height_m < 2000.0 => {
//                     Color::srgb(0.829, 0.743, 0.576).to_linear().to_f32_array()
//                 }
//                 _height if height_m >= 2000.0 && height_m < 4000.0 => {
//                     Color::srgb(0.754, 0.643, 0.523).to_linear().to_f32_array()
//                 }
//                 _height if height_m >= 4000.0 && height_m < 9000.0 => {
//                     Color::srgb(0.677, 0.546, 0.473).to_linear().to_f32_array()
//                 }
//                 _ => Color::srgb(1.0, 1.0, 1.0).to_linear().to_f32_array(),
//             }
//         }
//     }
// }

const IMHOF_COLORS: &[[f32; 4]] = &[
    // Pre-calculated colors
    [0.002, 0.012, 0.004, 1.0],  // < 0.0: srgb(0.05, 0.125, 0.075).to_linear()
    [0.001, 0.004, 0.058, 1.0],  // == 0.0: srgb(0.025, 0.075, 0.275).to_linear()
    [0.379, 0.565, 0.260, 1.0],  // 0-100: srgb(0.654, 0.772, 0.541).to_linear()
    [0.535, 0.715, 0.356, 1.0],  // 100-200: srgb(0.753, 0.863, 0.634).to_linear()
    [0.759, 0.754, 0.346, 1.0],  // 200-500: srgb(0.882, 0.879, 0.624).to_linear()
    [0.704, 0.584, 0.310, 1.0],  // 500-1000: srgb(0.855, 0.783, 0.592).to_linear()
    [0.660, 0.523, 0.293, 1.0],  // 1000-2000: srgb(0.829, 0.743, 0.576).to_linear()
    [0.537, 0.368, 0.243, 1.0],  // 2000-4000: srgb(0.754, 0.643, 0.523).to_linear()
    [0.412, 0.263, 0.197, 1.0],  // 4000-9000: srgb(0.677, 0.546, 0.473).to_linear()
    [1.0, 1.0, 1.0, 1.0],        // >= 9000: srgb(1.0, 1.0, 1.0).to_linear()
];

/// Returns the pre-assigned and modified Imhof color assigned to a height
pub fn get_height_color(height_m: f32, colors: ColorSpectrum) -> [f32; 4] {
    match colors {
        ColorSpectrum::ImhofModified => {
            let index = if height_m < 0.0 { 0 }
            else if height_m == 0.0 { 1 }
            else if height_m < 100.0 { 2 }
            else if height_m < 200.0 { 3 }
            else if height_m < 500.0 { 4 }
            else if height_m < 1000.0 { 5 }
            else if height_m < 2000.0 { 6 }
            else if height_m < 4000.0 { 7 }
            else if height_m < 9000.0 { 8 }
            else { 9 };

            IMHOF_COLORS[index]
        }
    }
}