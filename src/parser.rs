use reqwest::{Response};
use select::document::Document;
use select::predicate::{Name};
use std::collections::HashSet;
use std::fs::File;
use regex::{RegexBuilder, Regex};

mod monster_hunter_fandom;
use monster_hunter_fandom::URLS;

mod kiranico;

use super::game::Game;

// Somehow cache these values in memory and in a file so we don't have to reach out via an HTTP request all the time.
pub fn get_games() -> HashSet<Game> {
      let resp: Response = reqwest::get(&URLS.get_game_list_url()).unwrap();

        assert!(resp.status().is_success(), "Url {} was not successfully retrieved by client, error code was {}", URLS.get_game_list_url(),resp.status());


        let resp_document: Document = Document::from_read(resp).unwrap();
        
        let _file = File::create("C:\\Users\\zjohnson\\Documents\\urls.txt").expect("Failed to create file");

        let game_wiki_regex: Regex = RegexBuilder::new(r"Monster.*Hunter")
            .case_insensitive(false)
            .build()
            .unwrap();

        let mut games: HashSet<Game> = HashSet::new();
        for node in resp_document.find(Name("a")) {
            let attributes: Vec<(&str, &str)> = node.attrs().filter(|x| game_wiki_regex.is_match(x.1)).collect();

            if attributes.len() == 2 {
                let mut absolute_url: String = URLS.get_base_url();
                absolute_url.push_str(attributes[0].1);
                let title: String =  String::from(attributes[1].1);
                let game: Game = Game{title, absolute_url};

                games.insert(game);

                //println!("{:?}", attributes);
            } else {
                continue;
            }
        }
        games
}

pub fn get_game(game_title: &str) -> Game {
    let games: HashSet<Game> = get_games();

    let game: &Game = games.iter().find(|x| x.title == game_title).expect("Could not find game");
    Game{title: String::from(&game.title), absolute_url: String::from(&game.absolute_url)}

}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn get_games_list_test() {
        let games: HashSet<Game> = get_games();

        assert_ne!(0, games.len());
        // Checks to see if the list returns at least a game in each generation of the series
        // If we can't at least find these we are in big trouble lol
        assert_ne!(games.iter().find(|x| x.title == "Monster Hunter"), None);
        assert_ne!(games.iter().find(|x| x.title == "Monster Hunter 2"), None);
        assert_ne!(games.iter().find(|x| x.title == "Monster Hunter 3"), None);
        assert_ne!(games.iter().find(|x| x.title == "Monster Hunter 4"), None);
        assert_ne!(games.iter().find(|x| x.title == "Monster Hunter: World"), None);
    }

    #[test]
    fn get_game_test() {
        let game_title: &str = "Monster Hunter";
        let game: Game = get_game(game_title);
        assert_eq!(game_title, game.title, "Test Failed. Searched for game {} and got {}", game_title, game.title);
    }
}

 