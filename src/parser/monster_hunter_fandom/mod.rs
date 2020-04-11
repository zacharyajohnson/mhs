pub struct InfoURLs<'a> {
    base_url : &'a str ,
    list_of_games_url : &'a str
}

// TODO Come back to this and see if we want full paths to get rid of all of these getters that essential concat the base_url with our other urls.
impl<'a> InfoURLs<'a> {
    pub fn get_game_list_url(&self) -> String {
        String::from(self.base_url.to_owned() + self.list_of_games_url)
    }

    pub fn get_base_url(&self) -> String {
        String::from(self.base_url)
    }
}

pub const URLS : InfoURLs = InfoURLs {
    base_url : "https://monsterhunter.fandom.com",
    list_of_games_url : "/wiki/Game_List"
};

