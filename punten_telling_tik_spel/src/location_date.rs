use serde::Serialize;
use chrono::NaiveDateTime;

// Simple type alias for clarity
pub type Timestamp = NaiveDateTime;

// Helper function to parse your specific timestamp format
pub fn parse_tijdstempel(tijdstempel: &str) -> Option<Timestamp> {
    // Format: "19-10-2025 14:12:22"
    NaiveDateTime::parse_from_str(tijdstempel, "%d-%m-%Y %H:%M:%S").ok()
}

// Location structure with chrono timestamp
#[derive(Serialize, Debug, Clone)]
pub struct Locatie {
    pub latitude: f64,
    pub longitude: f64,
    pub date: Timestamp,
}

impl Locatie {
    pub fn new(latitude: f64, longitude: f64, date: Timestamp) -> Self {
        Locatie { latitude, longitude, date }
    }
}
