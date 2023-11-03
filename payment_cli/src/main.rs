use std::env;

use colored::*;

mod data;
mod cmdfunc;

fn main() {
    let mut cmd: env::Args = env::args();
    if cmd.len() <= 1 {
        cmdfunc::print_commands()
    }
    cmd.next(); // skip first argument (file location)
    match cmd.next() {
        Some(cmd_str) => {
            let v = data::get_commands().into_iter().filter(|a| a.name.to_lowercase() == cmd_str.to_lowercase())
                .collect::<Vec<_>>();
            if v.len() >= 1 { (v[0].func)(&mut cmd, &v[0]) }
            else { println!("{}", "Err: Command not found".red().bold())}
        },
        None => println!("\nPlease use one of the parameters above")
    }
}




fn set_data(args: &mut env::Args, cmd: &data::Command) {
    match args.next() {
        Some(param) => {
            match param.to_lowercase().as_str() {
                "persondata" => {},
                "paymentgoal" => {},
                &_ => println!("Not valid param given.\nCorrect params:\t{}", cmd.params)
            }
        }
        _ => {
            println!("No parameter given.\nCorrect params:\t{}", cmd.params);
        }
    }
}
