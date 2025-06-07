use chrono::{NaiveDate, NaiveTime}; // For date and time handling

#[derive(Debug)]
struct SbsMessage {
    message_type: String,        // Message type (e.g., "MSG")
    transmission_type: u8,       // Transmission type (e.g., 1, 2, 3, etc.)
    session_id: Option<String>,  // Session ID (optional, often empty)
    aircraft_id: Option<String>, // Aircraft ID (optional, often empty)
    hex_ident: String,           // ICAO 24-bit address in hexadecimal, unique identifier 
    flight_id: Option<String>,   // Flight ID (optional, often empty)
    generated_date: NaiveDate,   // Date the message was generated (UTC)
    generated_time: NaiveTime,   // Time the message was generated (UTC)
    logged_date: NaiveDate,      // Date the message was logged (UTC)
    logged_time: NaiveTime,      // Time the message was logged (UTC)
    call_sign: Option<String>,   // Aircraft call_sign (optional)
    altitude: Option<u32>,       // Altitude in feet (optional)
    ground_speed: Option<u16>,   // Ground speed in knots (optional)
    track: Option<f32>,          // Track angle in degrees (optional)
    latitude: Option<f32>,       // Latitude in decimal degrees (optional)
    longitude: Option<f32>,      // Longitude in decimal degrees (optional)
    vertical_rate: Option<i16>,  // Vertical rate in feet per minute (optional)
    squawk: Option<String>,      // Transponder squawk code (optional)
    alert: Option<bool>,         // Alert flag (true if squawk has changed)
    emergency: Option<bool>,     // Emergency flag (true if emergency code is set)
    spi: Option<bool>,           // Special Position Indicator flag
    is_on_ground: Option<bool>,  // Ground status flag
}

impl SbsMessage {
    fn new(
        message_type: String,
        transmission_type: u8,
        session_id: Option<String>,
        aircraft_id: Option<String>,
        hex_ident: String,
        flight_id: Option<String>,
        generated_date: NaiveDate,
        generated_time: NaiveTime,
        logged_date: NaiveDate,
        logged_time: NaiveTime,
        call_sign: Option<String>,
        altitude: Option<u32>,
        ground_speed: Option<u16>,
        track: Option<f32>,
        latitude: Option<f32>,
        longitude: Option<f32>,
        vertical_rate: Option<i16>,
        squawk: Option<String>,
        alert: Option<bool>,
        emergency: Option<bool>,
        spi: Option<bool>,
        is_on_ground: Option<bool>,
    ) -> SbsMessage {
        SbsMessage {
            message_type,
            transmission_type,
            session_id,
            aircraft_id,
            hex_ident,
            flight_id,
            generated_date,
            generated_time,
            logged_date,
            logged_time,
            call_sign,
            altitude,
            ground_speed,
            track,
            latitude,
            longitude,
            vertical_rate,
            squawk,
            alert,
            emergency,
            spi,
            is_on_ground,
        }
    }
}
