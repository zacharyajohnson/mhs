use reqwest::Response;
use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name, Element};

pub fn getData() {
    const base_url: &str = "https://monsterhunter.fandom.com";
    let main_url: String = String::from(base_url.to_owned() + "/wiki/Monster_Hunter_Wiki");

    let resp: Response = reqwest::get(&main_url).unwrap();

    assert!(resp.status().is_success(), "Url was not successfully retrieved by client, error code was {}", resp.status());


    let resp_document: Document = Document::from_read(resp).unwrap();

    for node in resp_document.find(Name("a"))) {
        println!("{:#?}", node);
    }
}


 