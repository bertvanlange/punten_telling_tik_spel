


use serde::Serialize;
use crate::config::{Config,DefaultTeam};
use crate::location_date::{Locatie, Timestamp};
use crate::tikker::{Tikkers};
use crate::inport_info::{read_sheet_dynamic};

pub fn populate_teams_from_google_sheet(getikt_csv: &str, teams: &mut Teams, tikkers: &mut Tikkers,cfg: &Config) {
    let rows = read_sheet_dynamic(&getikt_csv);
    match rows {
        Ok(structure) => {
            for row in structure {
                let tijdstempel = row.get("Tijdstempel").unwrap_or(&"0".to_string()).to_string();

                if let Some(time_stamp) = cfg.is_relevent_time_stamp(tijdstempel.as_str()){
                    let team_id = row.get("Team index").unwrap_or(&"".to_string()).to_string();
                    let pasword = row.get("Wachtwoord").unwrap_or(&"".to_string()).to_string();
                    
                    println!("Processing tick for team_id: {}, pasword: {}, date: {:?}", team_id, pasword, time_stamp);
                    if tikkers.add_tik_with_date_by_paswoord(&pasword, time_stamp, 1).is_some() {
                        teams.add_tick_and_date_to_existing_or_new_team(&team_id, time_stamp);
                    }
                }
            }
        },
        Err(e) => {
            // handle error 
            println!("Error reading sheet: {}", e);
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Teams {
    pub team_list: Vec<Team>,
}

#[allow(unused)]
impl Teams {
    pub fn new() -> Self {
        Teams { team_list: Vec::new() }
    }

    pub fn add_team(&mut self, team: Team) {
        self.team_list.push(team);
    }

    pub fn get_team_by_id(&mut self, team_id: &str) -> Option<&mut Team> {
        for team in  &mut self.team_list {
            if team.team_id == team_id {
                return Some(team);
            }
        }
        None
    }


    pub fn add_tick_to_existing_or_new_team(&mut self, team_id: &str) {
        if let Some(team) = self.get_team_by_id(team_id) {
            team.add_tick();
        } else {
            let mut new_team = Team::new(team_id.to_string());
            new_team.add_tick();
            self.add_team(new_team);
        }
    }

    pub fn add_tick_and_date_to_existing_or_new_team(&mut self, team_id: &str, tick_date: Timestamp) {
        if let Some(team) = self.get_team_by_id(team_id) {
            team.add_tick_date(tick_date);
        } else {
            let mut new_team = Team::new(team_id.to_string());
            new_team.add_tick_date(tick_date);
            self.add_team(new_team);
        }
    }

    pub fn add_config(&mut self,cfg: &Config){
        if let Some(team_cfg_vec) = &cfg.default_teams {
            for team_cfg in team_cfg_vec {
                let Some (team_id) = team_cfg.team_id.clone() else {continue};
                let Some(team) = self.get_team_by_id(team_id.as_str()) else {continue};
                team.add_cfg(team_cfg);
            }
        } 
    }
}

// Structures
#[derive(Debug, Serialize,Default)]
pub struct Team {
    pub name: Option<String>,
    pub team_id: String,
    pub image_url: Option<String>,
    pub ticks: u32,
    pub points: u32,
    pub subgroup: Option<String>,
    pub last_tick: Option<Timestamp>,
    pub last_point: Option<Timestamp>,
    pub last_loc: Option<Locatie>,
    }

#[allow(unused)]
impl Team {
    pub fn new(team_id: String) -> Self {
        let mut team = Team::default();
        team.team_id = team_id;
        team
    }

    pub fn add_tick(&mut self){
        self.ticks += 1;
    }

    pub fn add_tick_date(&mut self, tick_date: Timestamp) {
        self.add_tick();
        self.last_tick = Some(tick_date);
    }


    pub fn add_points(&mut self, points: u32){
        self.points += points;
    }

    pub fn add_points_date(&mut self, points: u32, point_date: Timestamp) {
        self.add_points(points);
        self.last_point = Some(point_date);
    }   

    pub fn update_location(&mut self, locatie: Locatie) {
        self.last_loc = Some(locatie);
    }
    
    pub fn add_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn add_cfg(&mut self, team_cfg: &DefaultTeam) {
        add_if_some(&mut self.image_url, &team_cfg.image_url);
        add_if_some(&mut self.name, &team_cfg.name);
    }

}   


fn add_if_some<T: Clone>(target: &mut Option<T>, source: &Option<T>) {
    if let Some(value) = source {
        *target = Some(value.clone());
    }
}
