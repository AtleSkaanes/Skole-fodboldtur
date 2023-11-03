use std::env;

use colored::*;

use crate::data;



pub fn print_commands() {
    println!("Available Commands:\n");
    for cmd in data::get_commands() {
        println!("\t{}: ({}) - {}", cmd.name.yellow().bold(), cmd.params, cmd.desc);
    }
}

pub fn list_items(_args: &mut env::Args, _cmd: &data::Command) {
    if data::load_map().len() < 1 {
        println!("{}", "No people registered".yellow().bold());
        return;
    }
    println!("name,\tpaid,\tmonthly,\tremaining");
    for (key, value) in data::load_map() {
        println!("{},\t{},\t{},\t{}", key.yellow().bold(),
                value.currently_paid, value.monthly_payment, 4500.0 - value.currently_paid);
    }
}


pub fn add_content(args: &mut env::Args, cmd: &data::Command) {
    match args.next() {
        Some(param) => {
            match param.to_lowercase().as_str() {
                "payment" => add_payment(args),
                "person" => add_person(args),
                &_ => println!("Not valid param given.\nCorrect params:\t{}", cmd.params)
            }
        }
        _ => {
            println!("No parameter given.\nCorrect params:\t{}", cmd.params);
        }
    }
}

pub fn add_person(args: &mut env::Args) {
    let mut new_person = data::PersonData::default();
    let mut people_map = data::load_map();
    match args.next() {
        Some(param) => {
            if people_map.contains_key(&param) {
                println!("{} is already registered.\n\tuse 'set PersonData [name] [payed amount] [monthly payment]' to change values", param);
                println!("\t{}", "^^^ nvm... The 'set' feature isn't implemented yet, so just go into data.json to change values.".red().bold());
                return;
            }
            new_person.currently_paid = match args.next() {
                Some(arg) => arg.parse::<f64>().expect("Should use numbers"),
                _ => 0.0
            };
            new_person.monthly_payment = match args.next() {
                Some(arg) => arg.parse::<f64>().expect("Should use numbers"),
                _ => 0.0
            };
            people_map.insert(param.clone(), new_person);
            data::save_map(&people_map);
            println!("Added {}", param.yellow().bold());
        },
        _ => {
            println!("No parameter set given. use <name> [starting payment] [monthly payment]");
            println!("\t{} = name of person (required)", "<name>".yellow().bold());
            println!("\t{} = what the person already has paid (0 by default)", "[starting payment]".yellow().bold());
            println!("\t{} = what the person will pay each month automatically (0 by default)", "[monthly payment]".yellow().bold());
        }
    }
}

pub fn add_payment(args: &mut env::Args) {
    let mut people_map = data::load_map();
    match args.next() {
        Some(param) => {
            if !people_map.contains_key(&param) {
                println!("{} is not registered.
                         use 'add PersonData <name> [starting payment] [monthly payment]' to add person", param);
                return;
            }
            let payment: f64 = match args.next() {
                Some(amount) => amount.parse::<f64>().expect("Should only use numbers"),
                _ => {
                    println!("No parameter for payment given.");
                    return;
                }
            };

            people_map.get_mut(&param).unwrap().currently_paid += payment;
            data::save_map(&people_map);
            println!("{} has now paid {} more", param.yellow().bold(), payment.to_string().yellow().bold());
        }
        _ => {
            println!("No parameter set given. use <name> <payment amount>");
            println!("\t{} = name of person (required)", "<name>".yellow().bold());
            println!("\t{} = amount the person is paying (required))", "[starting payment]".yellow().bold());
        }
    }
}


