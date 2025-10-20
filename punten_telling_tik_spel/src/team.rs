


use serde::Serialize;
use crate::location_date::{Date, Locatie};
use crate::tikker::{Tikkers};
use crate::inport_info::{read_sheet_dynamic};

pub fn populate_teams_from_google_sheet(getikt_csv: &str, teams: &mut Teams, tikkers: &mut Tikkers) {
    let rows = read_sheet_dynamic(&getikt_csv);
    match rows {
        Ok(structure) => {
            for row in structure {
                let team_id = row.get("Team index").unwrap_or(&"".to_string()).to_string();
                let pasword = row.get("Wachtwoord").unwrap_or(&"".to_string()).to_string();
                let date: Date = Date::from_tijdstempel(row.get("Tijdstempel").unwrap_or(&"0".to_string()));
                println!("Processing tick for team_id: {}, pasword: {}, date: {:?}", team_id, pasword, date);
                if date.year == 0 {
                    continue;
                }
                if tikkers.add_tik_with_date_by_paswoord(&pasword, date.clone(), 1).is_some(){
                    teams.add_tick_and_date_to_existing_or_new_team(&team_id, date);
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

    pub fn add_tick_and_date_to_existing_or_new_team(&mut self, team_id: &str, tick_date: Date) {
        if let Some(team) = self.get_team_by_id(team_id) {
            team.add_tick_date(tick_date);
        } else {
            let mut new_team = Team::new(team_id.to_string());
            new_team.add_tick_date(tick_date);
            self.add_team(new_team);
        }
    }
}

// Structures
#[derive(Debug, Serialize)]
pub struct Team {
    pub name: Option<String>,
    pub team_id: String,
    pub ticks: u32,
    pub points: u32,
    pub subgroup: Option<String>,
    pub last_tick: Option<Date>,
    pub last_point: Option<Date>,
    pub last_loc: Option<Locatie>,
    }

#[allow(unused)]
impl Team {
    pub fn new(team_id: String) -> Self {
        Team {
            name: None,
            team_id,
            ticks: 0,
            points: 0,
            subgroup: None,
            last_tick: None,
            last_point: None,
            last_loc: None,
        }
    }

    pub fn add_tick(&mut self){
        self.ticks += 1;
    }

    pub fn add_tick_date(&mut self, tick_date: Date) {
        self.add_tick();
        self.last_tick = Some(tick_date);
    }


    pub fn add_points(&mut self, points: u32){
        self.points += points;
    }

    pub fn add_points_date(&mut self, points: u32, point_date: Date) {
        self.add_points(points);
        self.last_point = Some(point_date);
    }   

    pub fn update_location(&mut self, locatie: Locatie) {
        self.last_loc = Some(locatie);
    }
    
    pub fn add_name(&mut self, name: String) {
        self.name = Some(name);
    }

}   


