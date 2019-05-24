use http::{self, Response, StatusCode};
use reqwest::{header};
use scraper::{Html};
use serde_derive::Serialize;
use serde_json;

use super::fetcher;

#[derive(Debug, Serialize)]
struct ResponseBody {
  username: String,
  name: Option<String>,
  bio: Option<String>,
  site: Option<String>,
}

const NAME_SELECTOR: &str = ".vcard-fullname";
const BIO_SELECTOR: &str = ".user-profile-bio div";
const SITE_SELECTOR: &str = "[data-test-selector=\"profile-website-url\"] a";

fn extract_data(username: &str, html: &String) -> ResponseBody {
  let fragment = Html::parse_document(html);

  let name = fetcher::get_inner_text(&fragment, NAME_SELECTOR);
  let bio = fetcher::get_inner_text(&fragment, BIO_SELECTOR);
  let site = fetcher::get_inner_text(&fragment, SITE_SELECTOR);

  return ResponseBody {
    username: username.to_string(),
    name: name,
    bio: bio,
    site: site,
  };
}

pub fn get_profile_data(username: &str) -> http::Result<Response<String>> {
    let url = format!("https://github.com/{}", username);
    let html = fetcher::fetch(&url);

    let content = extract_data(&username, &html);
    let content_str =
        serde_json::to_string_pretty(&content).expect("Failed to serialize to JSON");
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::CACHE_CONTROL, "public, s-maxage=3600, stale-while-revalidate")
        .body(content_str)
        .expect("Failed to render response");
    Ok(response)
}
