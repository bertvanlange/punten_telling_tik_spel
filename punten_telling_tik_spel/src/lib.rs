use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use serde::Serialize;

use crate::inport_info::{PUB_DOWNLOAD_URL_SHEET_CONFIG, PUB_DOWNLOAD_URL_SHEET_GETIKT, PUB_DOWNLOAD_URL_SHEET_WW};
use crate::team::{Teams, populate_teams_from_google_sheet};
use crate::tikker::{Tikkers, get_tikkers_from_google_sheet};
use crate::config::{config_from_csv, Config};


mod location_date;
mod team;
mod tikker;
mod inport_info;
mod config; 

// Export the Google Sheet URLs to JavaScript
#[wasm_bindgen]
pub fn get_tikkers_url() -> String {
    PUB_DOWNLOAD_URL_SHEET_WW.to_string()
}

#[wasm_bindgen]
pub fn get_getikt_url() -> String {
    PUB_DOWNLOAD_URL_SHEET_GETIKT.to_string()
}

#[wasm_bindgen]
pub fn get_config_url() -> String {
    PUB_DOWNLOAD_URL_SHEET_CONFIG.to_string()
}

// Shared data structure
#[derive(Serialize, Debug)]
pub struct GameData {
    pub teams: Teams,
    pub tikkers: Tikkers,
}

// Core parsing logic - works in both wasm and tests
pub fn parse_game_data_core(tikkers_csv: &str, getikt_csv: &str, config_csv: &str) -> Result<GameData, String> {
    let cfg: Config = config_from_csv(config_csv)
        .map_err(|e| format!("Error parsing config: {}", e))?;
    let mut tikkers = get_tikkers_from_google_sheet(tikkers_csv, &cfg)?;
    
    let mut teams = Teams::new();
    populate_teams_from_google_sheet(getikt_csv, &mut teams, &mut tikkers, &cfg);

    teams.add_config(&cfg);

    Ok(GameData { teams, tikkers })
}

// Wasm-specific wrapper that converts to JsValue
#[wasm_bindgen]
pub fn parse_game_data(tikkers_csv: &str, getikt_csv: &str, config_csv: &str) -> Result<JsValue, JsValue> {
    let game_data = parse_game_data_core(tikkers_csv, getikt_csv, config_csv)
        .map_err(|e| JsValue::from_str(&e))?;
    
    to_value(&game_data).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}


#[cfg(test)]
mod tests {
    use super::*;
    

    #[tokio::test]
    async fn test_parse_game_data() {
        use crate::inport_info::download_sheet;
        // Sample CSV data
        let tikkers_csv = download_sheet(PUB_DOWNLOAD_URL_SHEET_WW).await.unwrap();
        let getikt_csv = download_sheet(PUB_DOWNLOAD_URL_SHEET_GETIKT).await.unwrap();
        let config_csv = download_sheet(PUB_DOWNLOAD_URL_SHEET_CONFIG).await.unwrap();

        print!("tikkers_csv: {}\n", tikkers_csv);
        print!("getikt_csv: {}\n", getikt_csv);
        print!("config_csv: {}\n",config_csv);

        let result = parse_game_data_core(&tikkers_csv, &getikt_csv,&config_csv);

        print!("result: {:?}\n", result);
        assert!(result.is_ok());
    }
}// Force rebuild Tue Nov 11 12:50:06 CET 2025
