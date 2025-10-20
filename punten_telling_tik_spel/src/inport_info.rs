// use serde::{ Deserialize};
use std::collections::HashMap;
use csv::ReaderBuilder;

pub const PUB_DOWNLOAD_URL_SHEET_GETIKT : &str = "https://docs.google.com/spreadsheets/d/e/2PACX-1vQG7HcjC0122o9DzbvQk-yLKb_V6omnuf78kIKeqaOsq2-GG_PYO3GzHhm7sN6_oHBBn2sdjPqROo2B/pub?output=csv";
pub const PUB_DOWNLOAD_URL_SHEET_WW     : &str = "https://docs.google.com/spreadsheets/d/e/2PACX-1vTA19ZmwxnFxahu07xl1lf--04UZRY_opXg4kApwn90WrMOU8BauZ-Drs0cDyobG-bQ5wC7F_lykOYJ/pub?output=csv";

// #[derive(Debug, Deserialize)]
// struct SheetrowGetikt{
//     #[serde(rename = "Tijdstempel")]
//     pub time_stamp: String,
//     #[serde(rename = "Team")]
//     pub team_naam: String,
//     #[serde(rename = "Team index")]
//     pub team_id: String,
//     #[serde(rename = "Wachtwoord")]
//     pub wachtwoord: String,
// }

// #[derive(Debug, Deserialize)]
// struct SheetrowWw{
//     #[serde(rename = "Team")]
//     pub team_naam: String,
//     #[serde(rename = "Wachtwoord")]
//     pub wachtwoord: String,
// }

pub fn read_sheet_dynamic(text_file: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(text_file.as_bytes());

    let headers = rdr.headers()?.clone();
    let mut rows = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let mut row = HashMap::new();
        for (key, value) in headers.iter().zip(record.iter()) {
            row.insert(key.to_string(), value.to_string());
        }
        rows.push(row);
    }

    Ok(rows)
}

// download the sheet from the given URL and return the content as a String
// Only compile for tests (uses reqwest which isn't available in WASM)
#[cfg(test)]
pub async fn download_sheet(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response: String = reqwest::get(url).await?.text().await?;
    Ok(response)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_sheet() {
        let result = download_sheet(PUB_DOWNLOAD_URL_SHEET_GETIKT).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_read_sheet_dynamic() {
        let result = download_sheet(PUB_DOWNLOAD_URL_SHEET_GETIKT).await;
        assert!(result.is_ok());
        let sheet_content = result.unwrap();
        let structure = read_sheet_dynamic(&sheet_content);
        assert!(structure.is_ok());
        let rows = structure.unwrap();
        print!("{:?}\n", rows);

        for row in rows {
            print!("{:?}\n", row);
        }
    }

}