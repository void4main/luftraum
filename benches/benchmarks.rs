use std::f32::consts::PI;
use divan;
fn main() {
    divan::main();
}

pub fn haversine_distance(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f32 {
    const EARTH_RADIUS: f32 = 6371.00887714; // Earth radius in km

    // Convert degrees to radians
    let lat1_rad = lat1 * PI / 180.0;
    let lon1_rad = lon1 * PI / 180.0;
    let lat2_rad = lat2 * PI / 180.0;
    let lon2_rad = lon2 * PI / 180.0;

    // Differences
    let dlat = lat2_rad - lat1_rad;
    let dlon = lon2_rad - lon1_rad;

    // Haversine formula
    let a = (dlat / 2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();

    EARTH_RADIUS * c
}

pub fn haversine_distance2(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f32 {
    const EARTH_RADIUS: f32 = 6371.00887714; // Earth radius in km

    // Convert degrees to radians
    let lat1_rad = lat1 * PI / 180.0;
    let lon1_rad = lon1 * PI / 180.0;
    let lat2_rad = lat2 * PI / 180.0;
    let lon2_rad = lon2 * PI / 180.0;

    // Differences
    let dlat = lat2_rad - lat1_rad;
    let dlon = lon2_rad - lon1_rad;

    // Haversine formula
    //let a = (dlat / 2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);
    let a = (dlat / 2.0).sin() * (dlat / 2.0).sin() + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin() * (dlon / 2.0).sin();
    let c = 2.0 * a.sqrt().asin();

    EARTH_RADIUS * c
}

#[divan::bench]
pub fn bench_haversine_1() -> f32 {
    
    let dist = haversine_distance(divan::black_box(40.7128),
                       divan::black_box(-74.0060),
                       divan::black_box(51.5074),
                       divan::black_box(-0.1278),);
    dist
}

#[divan::bench]
pub fn bench_haversine_2() -> f32 {
    let dist = haversine_distance2(divan::black_box(40.7128),
                                   divan::black_box(-74.0060),
                                   divan::black_box(51.5074),
                                   divan::black_box(-0.1278),);
    dist
}