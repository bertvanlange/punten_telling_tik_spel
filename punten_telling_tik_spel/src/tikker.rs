use serde::{ Serialize};

use crate::location_date::{Date, Locatie};
use crate::inport_info::read_sheet_dynamic;

#[derive(Serialize, Debug)]
pub struct Tikkers {
    pub tikker_list: Vec<Tikker>,
}

#[allow(unused)]
impl Tikkers {
    pub fn default() -> Self {
        Tikkers { tikker_list: Vec::new() }
    }

    pub fn add_tikker(&mut self, tikker: Tikker) {
        self.tikker_list.push(tikker);
    }

    pub fn add_ticker_by_name_paswoord(&mut self, name: &str, paswoord: &str) {
        let tikker = Tikker::new(name.to_string(), paswoord.to_string());
        self.tikker_list.push(tikker);
    }

    pub fn get_tikker_by_name(&mut self, name: &str) -> Option<&mut Tikker> {
        for tikker in  &mut self.tikker_list {
            if tikker.name == name {
                return Some(tikker);
            }
        }
        None
    }

    pub fn get_tikker_by_pasword(&mut self, paswoord: &str) -> Option<&mut Tikker> {
        for tikker in  &mut self.tikker_list {
            if tikker.paswoord == paswoord {
                return Some(tikker);
            }
        }
        None
    }

    pub fn add_num_tiks_by_name(&mut self, name: &str, num_tiks: u32) -> Option<&str> {
        if let Some(tikker) = self.get_tikker_by_name(name) {
            tikker.tiks += num_tiks;
            Some(&tikker.name)
        } else {
            None
        }
    }

    pub fn add_num_tiks_by_paswoord(&mut self, paswoord: &str, num_tiks: u32) -> Option<&mut Tikker> {
        if let Some(tikker) = self.get_tikker_by_pasword(paswoord) {
            tikker.tiks += num_tiks;
            Some(tikker)
        } else {
            None
        }
    }

    pub fn add_tik_with_date_by_paswoord(&mut self, paswoord: &str, tik_date: Date, num_tiks: u32) -> Option<&str> {
        if let Some(tikker) = self.add_num_tiks_by_paswoord(paswoord, num_tiks) {
            tikker.last_tick = Some(tik_date);
            Some(&tikker.name)
        } else {
            None
        }
    }


    pub fn remove_tikker_by_name(&mut self, name: &str) -> Option<Tikker> {
        if let Some(pos) = self.tikker_list.iter().position(|x| x.name == name) {
            Some(self.tikker_list.remove(pos))
        } else {
            None
        }
    }   

    pub fn remove_tikker_by_paswoord(&mut self, paswoord: &str) -> Option<Tikker> {
        if let Some(pos) = self.tikker_list.iter().position(|x| x.paswoord == paswoord) {
            Some(self.tikker_list.remove(pos))
        } else {
            None
        }
    }

}


#[derive(Serialize, Debug)]
pub struct Tikker {
    pub name: String,
    pub paswoord: String,
    pub tiks: u32,
    pub last_tick: Option<Date>,
    pub last_loc: Option<Locatie>,
}


#[allow(unused)]
impl Tikker {
    pub fn new(name: String, paswoord: String) -> Self {
        Tikker {
            name,
            paswoord,
            tiks: 0,
            last_tick: None,
            last_loc: None,
        }
    }

    pub fn add_tik(&mut self){
        self.tiks += 1;
    }

    pub fn add_tik_date(&mut self, tik_date: Date) {
        self.add_tik();
        self.last_tick = Some(tik_date);
    }

    pub fn update_location(&mut self, locatie: Locatie) {
        self.last_loc = Some(locatie);
    }

}




pub  fn get_tikkers_from_google_sheet(sheet_csv: &str) -> Result<Tikkers,String> {
    let mut tikkers = Tikkers::default();
    
    let rows = read_sheet_dynamic(sheet_csv);
    match rows {
        Ok(structure) => {
            for row in structure {
                let team_naam = row.get("Naam").unwrap_or(&"".to_string()).to_string();
                let wachtwoord = row.get("Wachtwoord").unwrap_or(&"".to_string()).to_string();
                tikkers.add_ticker_by_name_paswoord(&team_naam, &wachtwoord);
            }
        },
        Err(e) => {
            // handle error
            println!("Error reading sheet: {}", e);
        }
    }
    Ok(tikkers)
}

