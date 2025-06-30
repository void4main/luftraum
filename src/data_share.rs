use chrono::{NaiveDate, NaiveTime};
use std::collections::HashMap;

pub struct SharedDataDb {
    plane_db: HashMap<String, PlaneDataSet>, // PlaneID and related data
}

struct PlaneDataSet {
    plane_id: String,           // Redundant to hex_ident
    last_seen: usize,           // Set to 0 on new or updated entries
    data_const: PlaneDataConst, // Store all fixed plane data
    data_var: PlaneDataVar,     // Store variable plane data
}

struct PlaneDataConst {
    session_id: Option<String>,  // Session ID (optional, often empty)
    aircraft_id: Option<String>, // Aircraft ID (optional, often empty)
    hex_ident: String,           // ICAO 24-bit address in hexadecimal
    flight_id: Option<String>,   // Flight ID (optional, often empty)
    call_sign: Option<String>,   // Aircraft call_sign (optional)
}

struct PlaneDataVar {
    message_type: Vec<String>,       // Message type (e.g., "MSG")
    transmission_type: Vec<usize>,   // Transmission type (e.g., 1, 2, 3, etc.)
    generated_date: Vec<NaiveDate>,  // Date the message was generated (UTC)
    generated_time: Vec<NaiveTime>,  // Time the message was generated (UTC)
    logged_date: Vec<NaiveDate>,     // Date the message was logged (UTC)
    logged_time: Vec<NaiveTime>,     // Time the message was logged (UTC)
    altitude: Vec<Option<f32>>,      // Altitude in feet (optional)
    ground_speed: Vec<Option<f32>>,  // Ground speed in knots (optional)
    track: Vec<Option<f32>>,         // Track angle in degrees (optional)
    latitude: Vec<Option<f32>>,      // Latitude in decimal degrees (optional)
    longitude: Vec<Option<f32>>,     // Longitude in decimal degrees (optional)
    vertical_rate: Vec<Option<f32>>, // Vertical rate in feet per minute (optional)
    squawk: Vec<Option<i32>>,        // Transponder squawk code (optional)
    alert: Vec<Option<bool>>,        // Alert flag (true if squawk has changed)
    emergency: Vec<Option<bool>>,    // Emergency flag (true if emergency code is set)
    spi: Vec<Option<bool>>,          // Special Position Indicator flag
    is_on_ground: Vec<Option<bool>>, // Ground status flag
}

impl SharedDataDb {
    pub fn new() -> SharedDataDb {
        SharedDataDb {
            plane_db: HashMap::<String, PlaneDataSet>::new(),
        }
    }

    pub fn get_planes_id(&self) -> Vec<&str> {
        let list_of_planes: Vec<&str> = self.plane_db.keys().map(|s| s.as_str()).collect();
        list_of_planes
    }

    pub fn increase_last_seen(&mut self) {
        // Increase last_seen of all plane entries
        for (_key, value) in self.plane_db.iter_mut() {
            value.last_seen += 10;
        }
    }

    pub fn get_last_seen(&self, plane_id: String) -> usize {
        self.plane_db.get(&plane_id).unwrap().last_seen
    }

    pub fn get_squawk(&self, plane_id: String) -> Option<i32> {
        self.plane_db
            .get(&plane_id)
            .unwrap()
            .data_var
            .squawk
            .last()
            .unwrap()
            .clone()
    }
    
    pub fn get_ground_speed(&self, plane_id: String) -> Option<f32> {
        self.plane_db.get(&plane_id).unwrap().data_var.ground_speed.last().unwrap().clone()
    }
    
    pub fn get_track(&self, plane_id: String) -> Option<f32> {
        self.plane_db.get(&plane_id).unwrap().data_var.track.last().unwrap().clone()
    }

    pub fn remove_plane(&mut self, plane_id: String) {
        self.plane_db.remove(&plane_id);
    }

    pub fn get_latest_pos(&self, plane_id: String) -> Option<(f32, f32, f32)> {
        if self.plane_db.contains_key(&plane_id) {
            let p_dataset = self.plane_db.get(&plane_id).unwrap();
            let lat = p_dataset.data_var.latitude.last().unwrap();
            let long = p_dataset.data_var.longitude.last().unwrap();
            let alt = p_dataset.data_var.altitude.last().unwrap();
            return lat
                .and_then(|lat| long.and_then(|long| alt.map(|altitude| (lat, long, altitude))));
        }
        None
    }

    pub fn get_call_sign(&self, plane_id: String) -> String {
        // TODO: Weired code, cleanup
        if self.plane_db.contains_key(&plane_id) {
            let p_dataset = self.plane_db.get(&plane_id).unwrap();
            let call_sign = p_dataset.data_const.call_sign.clone().unwrap();
            return call_sign;
        }
        "-".to_string()
    }

    pub fn update_data(
        &mut self,
        session_id: Option<String>,
        aircraft_id: Option<String>,
        hex_ident: String,
        flight_id: Option<String>,
        call_sign: Option<String>,
        message_type: String,
        transmission_type: usize,
        generated_date: NaiveDate,
        generated_time: NaiveTime,
        logged_date: NaiveDate,
        logged_time: NaiveTime,
        altitude: Option<f32>,
        ground_speed: Option<f32>,
        track: Option<f32>,
        latitude: Option<f32>,
        longitude: Option<f32>,
        vertical_rate: Option<f32>,
        squawk: Option<i32>,
        alert: Option<bool>,
        emergency: Option<bool>,
        spi: Option<bool>,
        is_on_ground: Option<bool>,
    ) {
        let temp = &mut self.plane_db;
        // Update if plane already created
        if temp.contains_key(&hex_ident) {
            // TODO: implement update
            let data_temp = temp.get_mut(&hex_ident).unwrap();
            // TODO: Define message types, 3 = ES Airborne Position Message

            // New data, so last_seen is 0
            data_temp.last_seen = 0;

            if transmission_type == 3 {
                data_temp.data_var.latitude.push(latitude);
                data_temp.data_var.longitude.push(longitude);
                data_temp.data_var.altitude.push(altitude);
            }
            if transmission_type == 4 {
                data_temp.data_var.ground_speed.push(ground_speed);
                data_temp.data_var.track.push(track);
            }
            if transmission_type == 5 {
                if call_sign.is_some() {
                    if call_sign.clone().unwrap().len() > 0 {
                        data_temp.data_const.call_sign = call_sign;
                    }
                }
            }
            if transmission_type == 6 {
                if squawk.is_some() {
                    data_temp.data_var.squawk.push(squawk);
                }
            }
        } else {
            temp.insert(
                hex_ident.clone(),
                PlaneDataSet {
                    plane_id: hex_ident.clone(),
                    last_seen: 0, // New insert, so last_seen is now
                    data_const: PlaneDataConst {
                        session_id,
                        aircraft_id,
                        hex_ident,
                        flight_id,
                        call_sign,
                    },
                    data_var: PlaneDataVar {
                        message_type: vec![message_type],
                        transmission_type: vec![transmission_type],
                        generated_date: vec![generated_date],
                        generated_time: vec![generated_time],
                        logged_date: vec![logged_date],
                        logged_time: vec![logged_time],
                        altitude: vec![altitude],
                        ground_speed: vec![ground_speed],
                        track: vec![track],
                        latitude: vec![latitude],
                        longitude: vec![longitude],
                        vertical_rate: vec![vertical_rate],
                        squawk: vec![squawk],
                        alert: vec![alert],
                        emergency: vec![emergency],
                        spi: vec![spi],
                        is_on_ground: vec![is_on_ground],
                    },
                },
            );
        }
    }
}
