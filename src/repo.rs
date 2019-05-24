use http::{self, Response, StatusCode};
use reqwest::{header};
use scraper::{Html};
use serde_derive::Serialize;
use serde_json;

use super::fetcher;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResponseBody {
  description: String,
  link: Option<String>,
  watch_count: u32,
  star_count: u32,
  fork_count: u32,
}

const DESC_SELECTOR: &str = "[itemprop=\"about\"]";
const LINK_SELECTOR: &str = "[itemprop=\"url\"] a";
const WATCH_SELECTOR: &str = ".pagehead-actions li:nth-child(1) .social-count";
const STAR_SELECTOR: &str = ".pagehead-actions li:nth-child(2) .social-count";
const FORK_SELECTOR: &str = ".pagehead-actions li:nth-child(3) .social-count";

fn extract_data(html: &String) -> ResponseBody {
  let fragment = Html::parse_document(html);

  let description = fetcher::get_inner_text(&fragment, DESC_SELECTOR).unwrap();
  let link = fetcher::get_inner_text(&fragment, LINK_SELECTOR);
  let watch_count = fetcher::get_inner_text(&fragment, WATCH_SELECTOR).unwrap();
  let star_count = fetcher::get_inner_text(&fragment, STAR_SELECTOR).unwrap();
  let fork_count = fetcher::get_inner_text(&fragment, FORK_SELECTOR).unwrap();

  return ResponseBody {
    description: description.trim().to_string(),
    link: link,
    watch_count: watch_count.trim().parse::<u32>().unwrap(),
    star_count: star_count.trim().parse::<u32>().unwrap(),
    fork_count: fork_count.trim().parse::<u32>().unwrap(),
  };
}

pub fn get_repository_data(owner: &str, repository: &str) -> http::Result<Response<String>> {
    let url = format!("https://github.com/{}/{}", owner, repository);
    let html = fetcher::fetch(&url);

    let content = extract_data(&html);
    let content_str =
        serde_json::to_string_pretty(&content).expect("Failed to serialize to JSON");
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::CACHE_CONTROL, "public, maxage=3600, s-maxage=3600, stale-while-revalidate")
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(content_str)
        .expect("Failed to render response");
    Ok(response)
}
