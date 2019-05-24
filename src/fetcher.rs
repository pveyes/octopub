use http::{StatusCode};
use reqwest::{header, Client};
use scraper::{Html, Selector};
use url::Url;

pub fn fetch(url: &str) -> String {
  let url = Url::parse(&url).expect("Failed to parse URL");
  let client = Client::new();
  let mut res = client
    .get(url)
    .header(header::ACCEPT, "text/html")
    .send()
    .expect("Failed to send HTTP request");

  assert_eq!(res.status(), StatusCode::OK);
  let html = res.text().expect("Failed to get HTML");
  html
}

pub fn get_inner_text(fragment: &Html, selector: &str) -> Option<String> {
  let selector = Selector::parse(selector).unwrap();
  let el = fragment.select(&selector).next();
  
  match el {
    Some(node) => Some(node.text().collect::<Vec<_>>().join("")),
    None => None
  }
}
