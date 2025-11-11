// src/config.rs
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use crate::location_date::{Timestamp, parse_tijdstempel};
use csv::ReaderBuilder;
use std::error::Error;



// Main config structure (camelCase JSON courtesy of serde attrs)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub relevance: Option<Relevance>,
    pub default_teams: Option<Vec<DefaultTeam>>,
    pub sheet_overrides: Option<SheetOverrides>,
    pub ui: Option<UiConfig>,
    pub refresh_interval_seconds: Option<u32>,
    
    // Precomputed cutoff timestamp (calculated from relevance rules)
    #[serde(skip)]  // Don't serialize this field
    pub cutoff_timestamp: Option<Timestamp>,
    
    // Optional precomputed cutoff (ISO string) from JS - can keep for compatibility
    pub cutoff_iso: Option<String>,
    
    #[serde(flatten)]
    pub other: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize,Default)]
#[serde(rename_all = "camelCase")]
pub struct Relevance {
    pub days: Option<u32>,
    pub hours: Option<u32>,
    pub from: Option<Timestamp>,
    pub include_inactive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize,Default)]
#[serde(rename_all = "camelCase")]
pub struct DefaultTeam {
    pub team_id: Option<String>,
    pub name: Option<String>,
    pub image_url: Option<String>,
    pub color: Option<String>,
    pub active: Option<bool>,
}

impl DefaultTeam {
    pub fn partely_filled(&self) -> bool{
        if self.team_id.is_some()   {return true}
        if self.name.is_some()      {return true}
        if self.image_url.is_some() {return true}
        if self.color.is_some()     {return true}
        if self.active.is_some()    {return true}
        false
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetOverrides {
    pub enabled: Option<bool>,
    pub config_sheet_csv_url: Option<String>,
    pub assets_base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiConfig {
    pub compact_mode: Option<bool>,
    pub max_teams_per_page: Option<u32>,
    pub sort_by: Option<String>,
    pub show_last_activity: Option<bool>,
    pub timezone: Option<String>,
}

impl Config {
    pub fn is_relevent_time_stamp(&self,tijdstempel: &str) -> Option<Timestamp>{
        let time_stamp: chrono::NaiveDateTime = parse_tijdstempel(tijdstempel)?;
        match self.cutoff_timestamp {
            Some(v) if v < time_stamp => Some(time_stamp),
            None => Some(time_stamp),
            _ => None,
        }
    }

    pub fn add_relevance(&mut self,rest: Vec<String>){
        let mut rel = Relevance::default();
        let mut i = 0;

        while i < rest.len() {
            let token: String = rest[i].to_lowercase().to_string();
            let next = rest.get(i+1).map(|s: &String| s.trim()).unwrap_or("");
            let next_parsed = next.parse::<u32>();
            match token {
                t if t.contains("hour")     => {if let Ok(n) = next_parsed {rel.hours = Some(n)}},
                t if t.contains("day")      => {if let Ok(n) = next_parsed {rel.hours = Some(n)}},
                t if t.contains("from")     => {
                    if let Some(time_stamp) = parse_tijdstempel(next) {rel.from = Some(time_stamp)}
                },
                _ => {}
            }
            i += 2;
        }
        self.relevance = Some(rel);
        self.add_cutuf_time_stamp_from();
    }

    pub fn add_cutuf_time_stamp_from(&mut self){
        if let Some(relevance) = self.relevance.clone() { // check if ther is relevence struture
            if let Some(time_stamp) =  relevance.from { // if ther is a from, this hase prio
                self.cutoff_timestamp = Some(time_stamp)
            }else{
                // time form is not found, so compute the cut of time
                let mut time_stamp = Utc::now().naive_utc();
                if let Some(days ) = relevance.days { time_stamp -= Duration::days(days as i64)}
                if let Some(hours ) = relevance.hours { time_stamp -= Duration::hours(hours as i64)}
                self.cutoff_timestamp = Some(time_stamp)
            }
        }
    }

    pub fn add_team_config(&mut self,mut rest:  Vec<String>){
        let mut team = DefaultTeam::default();
        while rest.len() > 1 {
            let name =  rest.remove(0).to_lowercase();
            let value = rest.remove(0);
            match name.as_str() {
                "name"|"name:" => {team.name = Some(value)},
                "id" | "teamid" => {team.team_id = Some(value)},
                "image" | "image_paht" => {team.image_url = Some(value)},
                "color" => {team.color = Some(value)},
                "active" => {
                    if value.to_lowercase().contains("true") {
                        team.active = Some(true);
                    }else{
                        team.active = Some(false);
                    }
                },
                _ => {},
            }
        }
       
        if team.partely_filled() {
            match &mut self.default_teams {
                None => self.default_teams = Some(vec![team]),
                Some(default_team) => default_team.push(team),
            }
        }
    }
}


pub fn config_from_csv(config_csv: &str) -> Result<Config, Box<dyn Error>> {
    let mut cfg = Config::default();

    if config_csv.trim().is_empty() { return Ok(cfg); }

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(config_csv.as_bytes());


    for rec in rdr.records() {
        // pars line for line
        let record = rec?;
        let mut tokens: Vec<String> = record.iter().map(|s| s.trim().to_string()).collect();
        while tokens.last().map(|s| s.is_empty()).unwrap_or(false) { tokens.pop(); }
        if tokens.is_empty() { continue; }

        let key = tokens[0].to_lowercase();
        let rest: Vec<String> = if tokens.len() > 1 { tokens[1..].to_vec() } else { vec![] };
        

        match key.as_str() {
            "relevance_time" | "relevance" =>{
                cfg.add_relevance(rest);
            },
            "team_config" | "teamconfig" => {
                cfg.add_team_config(rest);
            },
            _ => {}
        }
    }
    Ok(cfg)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_parse_config_files() {
        use crate::inport_info::download_sheet;
        use crate::inport_info::{PUB_DOWNLOAD_URL_SHEET_CONFIG};

        // Sample CSV data
        let config_csv = download_sheet(PUB_DOWNLOAD_URL_SHEET_CONFIG).await.unwrap();

        print!("config_csv: {}\n",config_csv);

        let result = config_from_csv(&config_csv);

        print!("result: {:?}\n", result);
        assert!(result.is_ok());
    }
}