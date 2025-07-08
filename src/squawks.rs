use std::sync::OnceLock;
use std::collections::HashMap;
use bevy_egui::egui::Color32;
static TRANSPONDER_CODES: OnceLock<HashMap<i32, (&str, TransponderCodeColor)>> = OnceLock::new();

// 
#[derive(Debug)]
pub enum TransponderCodeColor {
    Default,
    Green,
    Yellow,
    Orange,
    Red,
    Blue
}

impl TransponderCodeColor {
    pub fn to_color32(&self) -> Color32 {
        match self {
            TransponderCodeColor::Red => Color32::RED,
            TransponderCodeColor::Orange => Color32::ORANGE,
            TransponderCodeColor::Yellow => Color32::YELLOW,
            TransponderCodeColor::Green => Color32::GREEN,
            TransponderCodeColor::Blue => Color32::LIGHT_BLUE,
            _  => Color32::GRAY,
        }
    }
}

// Static list of squawks with description, Int & DE, colours defined by me
// From https://de.wikipedia.org/wiki/Transpondercode

fn get_transponder_codes() -> &'static HashMap<i32, (&'static str, TransponderCodeColor)> {
    TRANSPONDER_CODES.get_or_init(|| {
        let mut map = HashMap::new();

        // Emergency codes
        map.insert(7500, ("Aircraft Hijacking", TransponderCodeColor::Red));
        map.insert(7600, ("Radio Communication Failure", TransponderCodeColor::Red));
        map.insert(7700, ("General Emergency", TransponderCodeColor::Red));

        // Special purpose codes
        map.insert(20, ("Hubschrauber-Rettungsflüge", TransponderCodeColor::Blue));
        map.insert(23, ("Einsatzflüge der Bundespolizei", TransponderCodeColor::Blue));
        map.insert(24, ("Militärische Flüge im Nachttiefflugsystem, die Geländefolgeflüge durchführen", TransponderCodeColor::Orange));
        map.insert(25, ("Absetzluftfahrzeug", TransponderCodeColor::Green));
        map.insert(27, ("Kunstflüge", TransponderCodeColor::Green));
        map.insert(30, ("Absetzluftfahrzeug", TransponderCodeColor::Green));
        map.insert(31, ("Open Skies", TransponderCodeColor::Orange));
        map.insert(32, ("VFR-Flüge von zivilen Luftfahrzeugen in der Identifizierungszone", TransponderCodeColor::Yellow));
        map.insert(33, ("VFR-Flüge von militärischen Luftfahrzeugen zwischen GND und FL 100", TransponderCodeColor::Yellow));
        map.insert(34, ("SAR", TransponderCodeColor::Blue));
        map.insert(35, ("VFR / IFR Wechselverfahren", TransponderCodeColor::Green));
        map.insert(36, ("Einsatzflüge der Polizei", TransponderCodeColor::Blue));
        map.insert(37, ("Einsatzflüge der Polizei mit Restlichtverstärker", TransponderCodeColor::Blue));
        map.insert(1000, ("IFR / Mode S Transponder Code", TransponderCodeColor::Green));
        map.insert(2000, ("Militärische Flüge im Nachttiefflugsystem", TransponderCodeColor::Orange));
        map.insert(7000, ("VFR-Flüge ziviler Luftfahrzeuge", TransponderCodeColor::Default));

        map
    })
}

pub fn get_transponder_description(squawk: i32) -> Option<&'static (&'static str, TransponderCodeColor)> {
     get_transponder_codes().get(&squawk).clone()
}
