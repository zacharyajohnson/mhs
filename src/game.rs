use std::collections::BTreeSet;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct Game {
    // Some games have multiple titles depending on the region they were released in
    pub titles: BTreeSet<String>,
    pub absolute_urls: BTreeSet<String>, // pub console: String
                                         // pub generation : &'a str
}

// TODO Clean up display trait for games
impl Display for Game {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        write!(formatter, "{:#?}{:#?}", self.titles, self.absolute_urls)
    }
}

/*
   We will support any games that have had an offical release in English. The games we will support is as follows:
      First Generation:
         Monster Hunter
         Monster Hunter Freedom / Monster Hunter Portable(Japanese Release Title) This game is the closest you can get to Monster Hunter G (Expansion to Monster Hunter),
            but has several notable differences from that game so they are not functionally equivalent.

      Second Generation:
         Monster Hunter Freedom 2 / Monster Hunter Portable 2nd(Japanese Release Title)
         Monster Hunter Freedom Unite / Monster Hunter Portable 2nd G(Japanese Release Title)

      Third Generation:
         Monster Hunter 3 / Monster Hunter Tri(Alternative name this title can be called)
         Monster Hunter 3 Ultimate / Monster Hunter 3G(Japanese Release Title) / Monster Hunter 3G HD Version(Japanese Release Title on the Wii U)

      Fourth Generation:
         Monster Hunter 4 Ultimate / Monster Hunter 4G(Japanese Release Title)
         Monster Hunter Generations / Monster Hunter X(Japanese Release Title)
         Monster Hunter Generations Ultimate / Monster Hunter XX(Japanese Release Title)

      Fifth Generation:
         Monster Hunter: World
         Monster Hunter World: Iceborne

      Spin-Offs:
         Monster Hunter Stories
         Monster Hunter Dynamic Hunting

   Games we will not support are games that did not have an offical release outside Japan. They include the Following
      First Generation:
         Monster Hunter G(Expansion to Monster Hunter)

      Second Generation:
         Monster Hunter 2

      Third Generation:
         Monster Hunter Portable 3rd

      Fourth Generation:
         Monster Hunter 4(Base game of Monster Hunter 4 Ultimate. Since this is the base game of MH4U, all content of this game should be included in MH4U, which we support)

      Frontier:
         Any Monster Hunter Frontier release

      Diary Series:
         Any Diary series game

      Spin Offs:
         Any spin off that is not Monster Hunter Stories

      Other:
         Monster Hunter Online(Not released yet and won't be outside of China)
*/
pub const SUPPORTED_GAMES: &'static [&str] = &[
    "Monster Hunter",
    "Monster Hunter Freedom",
    "Monster Hunter Freedom 2",
    "Monster Hunter Freedom Unite",
    "Monster Hunter 3",
    "Monster Hunter 3 Ultimate",
    "Monster Hunter 4 Ultimate",
    "Monster Hunter Generations",
    "Monster Hunter Generations Ultimate",
    "Monster Hunter: World",
    "Monster Hunter World: Iceborne",
];

pub fn get_alternative_titles(title: &str) -> Option<&str> {
    match title {
        "Monster Hunter Freedom" => Some("Monster Hunter Portable"),

        "Monster Hunter Freedom 2" => Some("Monster Hunter Portable 2nd"),

        "Monster Hunter Freedom Unite" => Some("Monster Hunter Portable 2nd G"),

        "Monster Hunter 3" => Some("Monster Hunter Tri"),

        "Monster Hunter 3 Ultimate" => Some("Monster Hunter 3G"),

        "Monster Hunter 4 Ultimate" => Some("Monster Hunter 4G"),

        "Monster Hunter Generations" => Some("Monster Hunter X"),

        "Monster Hunter Generations Ultimate" => Some("Monster Hunter XX"),

        _ => None,
    }
}
