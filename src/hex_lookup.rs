use kv::*;

// Hexdb.io data to struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Aircraft {
    #[serde(rename = "ModeS")]
    pub mode_s: String,
    #[serde(rename = "Registration")]
    pub registration: String,
    #[serde(rename = "Manufacturer")]
    pub manufacturer: String,
    #[serde(rename = "ICAOTypeCode")]
    pub icao_type_code: String,
    #[serde(rename = "Type")]
    pub aircraft_type: String,
    #[serde(rename = "RegisteredOwners")]
    pub registered_owners: String,
    #[serde(rename = "OperatorFlagCode")]
    pub operator_flag_code: String,
}

pub async fn fetch_aircraft(hex_code: &str) -> Result<Option<Aircraft>, reqwest::Error> {
    // Is aircraft data already cached?
    // Errors are ignored, show must go on :-)
    let result: Result<Option<Aircraft>, Error> = get_aircraft_data_from_cache(&hex_code)
        .await
        .map_or(Ok(None), |opt| Ok(opt));

    match result {
        // Return cached data
        Ok(Some(aircraft)) => Ok(Some(aircraft)),
        // Fetch data
        _ => {
            // Not found in cache, try to fetch it from hexdb.io
            let result = fetch_aircraft_data_from_hexdbio(&hex_code).await;
            match result {
                Ok(Some(aircraft)) => Ok(Some(aircraft)),
                _ => Ok(None),
            }
        }
    }
}

async fn fetch_aircraft_data_from_hexdbio(
    hex_code: &str,
) -> Result<Option<Aircraft>, reqwest::Error> {
    let url = format!("https://hexdb.io/api/v1/aircraft/{}", hex_code);
    let body = reqwest::get(url).await?.text().await?;

    // Check if hexdb.io returns "not found error"
    if body.contains("error") {
        return Ok(None);
    }

    let result = parse_aircraft_struct(&body);
    match result {
        Ok(aircraft_struct) => {
            // dbg!(&aircraft_struct);
            let _ = store_data(&hex_code, aircraft_struct.clone()).await;
            Ok(Some(aircraft_struct))
        }
        Err(e) => {
            eprintln!("Fetch aircraft, error: {}", e);
            Ok(None)
        }
    }
}

fn parse_aircraft_struct(json: &str) -> Result<Aircraft, serde_json::Error> {
    let val: Aircraft = serde_json::from_str(json)?;
    Ok(val)
}

async fn store_data(key: &str, value: Aircraft) -> Result<(), Error> {
    let cfg = Config::new("./aircraft.db");
    let store = Store::new(cfg)?;
    let bucket = store.bucket::<String, Json<Aircraft>>(None)?;
    let key = key.to_string();
    let value = Json(value);
    let res = bucket.set(&key, &value);
    match res {
        Err(e) => {
            eprintln!("Store aircraft data, error: {}", e);
            Ok(())
        }
        _ => Ok(()),
    }
}

async fn get_aircraft_data_from_cache(key: &str) -> Result<Option<Aircraft>, Error> {
    let cfg = Config::new("./aircraft.db");
    let store = Store::new(cfg)?;
    let bucket = store.bucket::<String, Json<Aircraft>>(None)?;

    let key = key.to_string();
    let result = bucket.get(&key);
    match result {
        Ok(result) => {
            if let Some(result) = result {
                // In cache db
                let aircraft_data = result.into_inner();
                Ok(Some(aircraft_data))
            } else {
                // Not in cache db
                Ok(None)
            }
        }
        // Other problems that might occur
        Err(e) => {
            eprintln!("KV db, error: {:?}", e);
            Ok(None)
        }
    }
}