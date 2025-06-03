use std::f32::consts::PI;

pub fn angle_deg_between(x_1: f32, y_1: f32, x_2: f32, y_2: f32) -> f32 {
    let scalar_product = x_1 * x_2 + y_1 * y_2;
    let betrag_x = (x_1 * x_1 + y_1 * y_1).sqrt();
    let betrag_y = (x_2 * x_2 + y_2 * y_2).sqrt();
    let angle = (scalar_product / (betrag_x * betrag_y)).acos().to_degrees();
    angle
}
pub fn angle_rad_between(x_1: f32, y_1: f32, x_2: f32, y_2: f32) -> f32 {
    let scalar_product = x_1 * x_2 + y_1 * y_2;
    let betrag_x = (x_1 * x_1 + y_1 * y_1).sqrt();
    let betrag_y = (x_2 * x_2 + y_2 * y_2).sqrt();
    let angle = (scalar_product / (betrag_x * betrag_y)).acos();
    angle
}

pub fn haversine_distance(lat_1: f32, lon_1: f32, lat_2: f32, lon_2: f32) -> f32 {
    const EARTH_RADIUS: f32 = 6371.00887714; // Earth radius in km
    let d_lat = lat_2 * PI / 180.0 - lat_1 * PI / 180.0;
    let d_lon = lon_2 * PI / 180.0 - lon_1 * PI / 180.0;
    let a = (d_lat / 2.0).sin() * (d_lat / 2.0).sin()
        + (lat_1 * PI / 180.0).cos()
            * (lat_2 * PI / 180.0).cos()
            * (d_lon / 2.0).sin()
            * (d_lon / 2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    let distance = EARTH_RADIUS * c;
    distance
}
pub fn get_pixel_pos(lat: f32, lon: f32, p_x: f32, p_y: f32, cell_rows: usize, cell_cols: usize, llx: f32, lly: f32, cell_dist: f32) -> (f32, f32) {
    // Map a geolocation to the pixel ground plane
    let llx_max = llx * cell_dist * cell_cols as f32 + 0.5 * llx;
    let lly_max = lly * cell_dist * cell_rows as f32 + 0.5 * lly;
    let p_pos_x = map_range(lat, llx, llx_max, -p_x/2.0, p_x/2.0);
    let p_pos_y = map_range(lon, lly, lly_max, -p_y/2.0, p_y/2.0);
    (p_pos_x, p_pos_y)
}

/// Return value is bevy pixel equivalent from meter 
pub fn get_pix_m(meter: f32, rows: usize, rows_width_deg: f32, pixel_plane_y: f32) -> f32 {
    // TODO: Check if 'accurate' enough
    const METER_DEG: f32 = 111227.5;
    let plane_length_m = rows_width_deg * METER_DEG * rows as f32;
    let pix_per_meter = pixel_plane_y / plane_length_m;
    pix_per_meter
}

/// Linear mapping of two ranges
pub fn map_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    // Linear mapping of two ranges
    out_min + ((value - in_min) / (in_max - in_min)) * (out_max - out_min)
}

pub fn get_number_triangles(subs: usize) -> usize {
    if subs <= 0 {
        return 0;
    };
    (subs + 1) * (subs + 1) * 2
}

pub fn get_number_of_triangles_row(subs: usize) -> usize {
    if subs <= 0 {
        return 0;
    }
    (subs + 1) * 2
}

pub fn get_num_subdivisions(data_points_row: u32) -> u32 {
    ((data_points_row as f32 / 2.0).floor() as u32) - 1 
}

pub trait Convertable {
    fn to_meters(&self) -> f32;
}

impl Convertable for f32 {
    fn to_meters(&self) -> f32 {
        self * 0.3048 // Conversion factor from feet to meters
    }
}

#[cfg(test)]
mod tests {
    use crate::terrain_color_spectrum::ColorSpectrum::ImhofModified;
    use crate::terrain_color_spectrum::get_height_color;
    use super::*;

    #[test]
    fn test_map_range() {
        // Latitude to Bevy pixel plane
        let result = map_range(52.5, 50.0, 55.0, 500.0, -500.0);
        assert_eq!(result, 0.0);
        let result = map_range(50.0, 50.0, 55.0, 500.0, -500.0);
        assert_eq!(result, 500.0);
        let result = map_range(55.0, 50.0, 55.0, 500.0, -500.0);
        assert_eq!(result, -500.0);
        
        // Longitude to Bevy pixel plane
        let result = map_range(5.0, 5.0, 10.0, -500.0, 500.0);
        assert_eq!(result, -500.0);
        let result = map_range(10.0, 5.0, 10.0, -500.0, 500.0);
        assert_eq!(result, 500.0);
        let result = map_range(7.5, 5.0, 10.0, -500.0, 500.0);
        assert_eq!(result, 0.0);
    }
    
    fn test_get_pix_m() {
        let meter = 0.0;
        let result = get_pix_m(meter, 6000, 0.00083, 2000.0);
        assert_eq!(result, 0.0);

        let meter = 10.0;
        let result = get_pix_m(meter, 6000, 0.00083, 2000.0);
        assert_eq!(result, 1.0);
    }
    
    fn test_get_height_color() {
        let height = 2000.0;
        let result = get_height_color(height, ImhofModified);
    }
}