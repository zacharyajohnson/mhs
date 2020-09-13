use std::env;

mod cli;
mod game;
mod gui;

mod hunter;
mod scraper;

fn main() {
    let cli_arguments: Vec<String> = env::args().collect();

    for argument in &cli_arguments {
        println!("{}", argument);
    }

    scraper::get_game("Monster Hunter");
  
}
