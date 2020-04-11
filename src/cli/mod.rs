use std::collections::HashSet;
use std::collections::HashMap;
use lazy_static::lazy_static;

use super::parser;

// lazy_static!{
//     static ref FUNCTION_TABLE: HashMap<&'static str, fn(args: &[&str]) -> HashSet<String> > = {
//         let mut hashmap: HashMap<&'static str, fn(args: &[&str]) -> HashSet<String> > = HashMap::new();
//         hashmap.insert("game", game);
//         hashmap
//     };
// }
pub fn process_command(command: &str, command_arguments: &[&str]) -> HashSet<String> {
    // Use pattern matching instead of panic

    // match FUNCTION_TABLE.get(command) {
    //     Some(x) => x(&[]),
    //     None => format!("Command {} not recognized", command
    // };
    // let function_pointer: fn(args: &[&str]) -> HashSet<String>  = *FUNCTION_TABLE.get(command).expect(&format!("Command {} not recognized", command));
    //function_pointer(&[])
    //TODO Create an enum for the commands?
    match command {
        "game" => game(&[]),
        _ => HashSet::new()
    }
}

fn game(args: &[&str]) -> HashSet<String> {
    let mut result: HashSet<String> = HashSet::new();
    if args.len() == 0 {
        for game in parser::get_games() {
            result.insert(game.title);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn process_command_game_no_args() {
        let process_command_game_result: HashSet<String> = process_command("game");
        let game_function_result: HashSet<String> = game(&[]);
        assert_eq!(process_command_game_result , game_function_result, "Command processor function process_command return value does not match the game function return value");
        
        // Dont have to check size of result for both sets since we know they are both equal at this point.
        // I know this seems like a weird random number
        // 63 is the number of valid game title results I got when I first ran it so from now on that is the base I am expecting
        assert!(game_function_result.len() >= 63, "Expected 63 title listings at least, got {}", game_function_result.len());
        
    }
}