use std::io::prelude::*;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::env;
use serde::{Serialize, Deserialize};
use serde_json;

use crate::*;



pub struct Command<'a> {
    pub name: &'a str,
    pub params: &'a str,
    pub desc: &'a str,
    pub func: fn(&mut env::Args, &Command)
}

#[derive(Default, Serialize, Deserialize)]
pub struct PersonData {
    pub currently_paid: f64,
    pub monthly_payment: f64
}



pub fn load_map() -> HashMap<String, PersonData>{
    let data_string: String = read_file().expect("Should create or read file");
    if data_string.is_empty() {
        return HashMap::new()
    }
    return match serde_json::from_str(data_string.as_str()) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Failed to parse data: {}", err);
            HashMap::new()
        }
    }
}

pub fn save_map(map: &HashMap<String, PersonData>) {
    let j = serde_json::to_string_pretty(&map).expect("Should output valid string");
    write_to_file(&j).expect("Should be able to write to file")
}

pub fn get_path() -> Result<String, std::io::Error> {
    match std::env::current_dir() {
        Ok(path) => Ok(path.into_os_string().into_string().unwrap()),
        Err(e) => Err(e)
    }
}

pub fn get_commands() -> Vec<Command<'static>> {
    vec![
        Command { name: "ListItems", params: "", desc: "Lists all people and their current payments", func: cmdfunc::list_items },
        Command { name: "Add", params: "Payment, Person", desc: "Add stuff", func: cmdfunc::add_content },
        // Command { name: "Set", params: "PersonData, PaymentGoal", desc: "Set data", func:set_data }
    ]
}


fn write_to_file(input: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open("data.json")?;
    file.write_all(&input.as_bytes())?;
    Ok(())
}

fn read_file() -> Result<String, std::io::Error> {
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("data.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}
