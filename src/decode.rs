use crate::data_share::*;
use chrono::{NaiveDate, NaiveTime};
use std::sync::{Arc, Mutex};

pub fn decode_message_sbs(data_share: &Arc<Mutex<SharedDataDb>>, message: String) {
    if message.is_ascii() && message.len() > 0 && message.len() < 255 {
        // Split message into (22) pieces by definition of SBS messages
        let vec: Vec<&str> = message.split(',').collect();

        if vec.len() == 22 {
            // Map each field to needed data type
            // TODO: More sophisticated decoding
            let tmp_transmission_type = vec[1].parse::<usize>().ok().unwrap();
            let tmp_generated_date = NaiveDate::parse_from_str(vec[6], "%Y/%m/%d").ok().unwrap();
            let tmp_generated_time = NaiveTime::parse_from_str(vec[7], "%H:%M:%S%.f")
                .ok()
                .unwrap();
            let tmp_logged_date = NaiveDate::parse_from_str(vec[8], "%Y/%m/%d").ok().unwrap();
            let tmp_logged_time = NaiveTime::parse_from_str(vec[9], "%H:%M:%S%.f")
                .ok()
                .unwrap();
            let tmp_altitude = vec[11].parse::<f32>().ok();
            let tmp_ground_speed = vec[12].parse::<f32>().ok();
            let tmp_track = vec[13].parse::<f32>().ok();
            let tmp_latitude = vec[14].parse::<f32>().ok();
            let tmp_longitude = vec[15].parse::<f32>().ok();
            let tmp_vertical_rate = vec[16].parse::<f32>().ok();
            let tmp_squawk = vec[17].parse::<i32>().ok();
            let tmp_alert = vec[18].parse::<usize>().unwrap_or(0);
            let tmp_alert_bool = matches!(tmp_alert, 1);
            let tmp_emergency = vec[19].parse::<usize>().unwrap_or(0);
            let tmp_emergency_bool = matches!(tmp_emergency, 1);
            let tmp_spi = vec[20].parse::<usize>().unwrap_or(0);
            let tmp_spi_bool = matches!(tmp_spi, 1);
            let tmp_is_on_ground = vec[21].parse::<usize>().unwrap_or(0);
            let tmp_is_on_ground_bool = matches!(tmp_is_on_ground, 1);

            // Write data to 'global' struct
            let mut data_tmp = data_share.lock().unwrap();
            data_tmp.update_data(
                Some(vec[2].to_string()),  // SessionID
                Some(vec[3].to_string()),  // AircraftID
                vec[4].to_string(),        // HexIdent
                Some(vec[5].to_string()),  // FlightID
                Some(vec[10].to_string()), // CallSign
                vec[0].to_string(),        // MessageType
                tmp_transmission_type,
                tmp_generated_date,
                tmp_generated_time,
                tmp_logged_date,
                tmp_logged_time,
                tmp_altitude,
                tmp_ground_speed,
                tmp_track,
                tmp_latitude,
                tmp_longitude,
                tmp_vertical_rate,
                tmp_squawk,
                Some(tmp_alert_bool),
                Some(tmp_emergency_bool),
                Some(tmp_spi_bool),
                Some(tmp_is_on_ground_bool),
            );
        }
    }
}
