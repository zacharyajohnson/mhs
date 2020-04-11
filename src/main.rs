use std::env;
use mhs::parser;

mod gui;
mod cli;
mod game;

mod parser;
mod hunter;

use hunter::weapon;

fn main() {
    let cli_arguments: Vec<String> = env::args().collect();

    //if cli_arguments.len() == 0 { 
      //  gui::start_gui();
    //} else {
        for argument in &cli_arguments {
            println!("{}" , argument);
        }

        let command: String = cli_arguments.get(1).unwrap().to_string();
        let command_arguments = cli_arguments.get(2..).iter().map(|x| )
        
        cli::process_command(&command, command_arguments);
//    }

}


