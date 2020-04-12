use std::fmt;

#[derive(Debug,PartialEq, Eq, Hash)]
pub struct Game {
    pub title : String,
    pub absolute_url : String
    // pub console: String
   // pub generation : &'a str
}

impl fmt::Display for Game {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(formatter, "{}{}", self.title, self.absolute_url)
    }
}


