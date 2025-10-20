use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use crate::inport_info::{PUB_DOWNLOAD_URL_SHEET_WW, PUB_DOWNLOAD_URL_SHEET_GETIKT};
use crate::team::{Teams, populate_teams_from_google_sheet};
use crate::tikker::{Tikkers, get_tikkers_from_google_sheet};

mod location_date;
mod team;
mod tikker;
mod inport_info;

// Export the Google Sheet URLs to JavaScript
#[wasm_bindgen]
pub fn get_tikkers_url() -> String {
    PUB_DOWNLOAD_URL_SHEET_WW.to_string()
}

#[wasm_bindgen]
pub fn get_getikt_url() -> String {
    PUB_DOWNLOAD_URL_SHEET_GETIKT.to_string()
}

// Main function: JavaScript fetches CSVs and passes them here for parsing
#[wasm_bindgen]
pub fn parse_game_data(tikkers_csv: &str, getikt_csv: &str) -> Result<JsValue, JsValue> {
    // Parse tikkers from the CSV
    let mut tikkers = get_tikkers_from_google_sheet(tikkers_csv)
        .map_err(|e| JsValue::from_str(&format!("Error parsing tikkers: {}", e)))?;
    
    // Parse teams and populate from getikt CSV
    let mut teams = Teams::new();
    populate_teams_from_google_sheet(getikt_csv, &mut teams, &mut tikkers);

    // Create a simple struct to hold both
    use serde::Serialize;
    
    #[derive(Serialize)]
    struct GameData {
        teams: Teams,
        tikkers: Tikkers,
    }
    
    let game_data = GameData { teams, tikkers };

    // Convert to JsValue for JavaScript
    to_value(&game_data).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}






// // Example logic: calculate points and aggregate ticks
// #[wasm_bindgen]
// pub fn compute_game_data(json_rows: &str) -> JsValue {
//     // Parse JSON rows from JS (array of {team_id, team_name, ticker})
//     let rows: Vec<serde_json::Value> = serde_json::from_str(json_rows).unwrap();

//     use std::collections::HashMap;
//     let mut teams_map: HashMap<String, Team> = HashMap::new();
//     let mut tickers_map: HashMap<String, Tikker> = HashMap::new();

//     for row in rows {
//         let team_name = row["team_name"].as_str().unwrap_or("Unknown").to_string();
//         let team_id = row["team_id"].as_u64().unwrap_or(0);
//         let ticker_name = row["ticker"].as_str().unwrap_or("Unknown").to_string();

//         // Team aggregation
//         let team = teams_map.entry(team_name.clone()).or_insert(Team { name: team_name.clone(), ticks: 0, points: 0 });
//         team.ticks += 1;
//         team.points += 69 + 42 * (team_id as u32);

//         // Ticker aggregation
//         let ticker = tickers_map.entry(ticker_name.clone()).or_insert(Tikker { naam: ticker_name.clone(), getikt: 0 });
//         ticker.getikt += 1;
//     }

//     // Return JSON to JS
//     let result = serde_json::json!({
//         "teams": teams_map.values().collect::<Vec<&Team>>(),
//         "tickers": tickers_map.values().collect::<Vec<&Tikker>>()
//     });
    
//     to_value(&result).unwrap()
// }

#[cfg(test)]
mod tests {
    use super::*;
    

    #[tokio::test]
    async fn test_parse_game_data() {
        use crate::inport_info::download_sheet;
        // Sample CSV data
        let tikkers_csv = download_sheet(PUB_DOWNLOAD_URL_SHEET_WW).await.unwrap();
        let getikt_csv = download_sheet(PUB_DOWNLOAD_URL_SHEET_GETIKT).await.unwrap();

        print!("tikkers_csv: {}\n", tikkers_csv);
        print!("getikt_csv: {}\n", getikt_csv);

        let result = parse_game_data(&tikkers_csv, &getikt_csv);

        print!("result: {:?}\n", result);
        assert!(result.is_ok());
    }
}