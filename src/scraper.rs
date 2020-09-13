// use regex::{Regex, RegexBuilder};
use reqwest::Response;
use select::document::Document;
use select::predicate::Name;
use select::predicate::Class;
use std::collections::{BTreeSet, HashSet};

mod monster_hunter_fandom;
use monster_hunter_fandom::URLS;

mod kiranico;
use super::game::*;

// Somehow cache these values in memory and in a file so we don't have to reach out via an HTTP request all the time.
// TODO These implementation details are starting to become specific to monster_hunter_fandom. I should create a wrapper around this.
pub fn get_games() -> HashSet<Game> {
    let mut games: HashSet<Game> = HashSet::new();

    for supported_game in SUPPORTED_GAMES {
        match get_game(supported_game) {
            Some(game) => {
                games.insert(game);
            }
            None => panic!("Cannot find info for {}", supported_game),
        }
    }

    games
}

pub fn get_game(game_title: &str) -> Option<Game> {
    if !SUPPORTED_GAMES.contains(&game_title) {
        return Option::None;
    }

    let mut game_url: String = String::from(URLS.base_url);
    let game_title_url: String = game_title.replace(" ", "_");
    game_url.push_str(&game_title_url);

    let resp: Response = reqwest::get(&game_url).unwrap();

    assert!(
        resp.status().is_success(),
        "Url {} was not successfully retrieved by client, error code was {}",
        game_url,
        resp.status()
    );

    let resp_document: Document = Document::from_read(resp).unwrap();

    for node in resp_document.find(Class("pi-data")) {
        println!("{:?}", node);
    }

    let mut titles: BTreeSet<String> = BTreeSet::new();

    if let Some(alternative_title) = get_alternative_titles(&game_title) {
        titles.insert(String::from(alternative_title));
    }

    titles.insert(String::from(game_title));

    let mut absolute_urls: BTreeSet<String> = BTreeSet::new();
    absolute_urls.insert(game_url);

    Option::Some(Game {
        titles,
        absolute_urls,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::Iterator;

    #[test]
    fn get_games_list_test() {
        let games: HashSet<Game> = get_games();

        // We should at least return something
        assert_ne!(0, games.len());

        // Checks to see if we have all games we support by title.
        for supported_game in SUPPORTED_GAMES {
            assert_ne!(
                games.iter().find(|x| x.titles.contains(*supported_game)),
                None
            );
        }

        // Make sure that the list we are returning is the same length as the amount of games we support
        assert_eq!(SUPPORTED_GAMES.len(), games.len());
    }

    #[test]
    fn get_game_test() {
        let game_title: &str = "Monster Hunter";
        let game: Game = get_game(game_title).unwrap();
        
        assert!(
            game.titles.contains(game_title),
            "Test Failed. Searched for game {} and got {}",
            game_title,
            game
        );
    }
}
