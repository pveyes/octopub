use http::{self, Request, Response, StatusCode};
use reqwest::{header, Client};
use scraper::{Html, Selector};
use serde_derive::Serialize;
use serde_json;
use url::Url;

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

fn get_inner_text(fragment: &Html, selector: &str) -> Option<String> {
  let selector = Selector::parse(selector).unwrap();
  let el = fragment.select(&selector).next();
  
  match el {
    Some(node) => Some(node.text().collect::<Vec<_>>().join("")),
    None => None
  }
}

fn extract_data(username: &str, html: &String) -> ResponseBody {
  let fragment = Html::parse_document(html);

  let name = get_inner_text(&fragment, NAME_SELECTOR);
  let bio = get_inner_text(&fragment, BIO_SELECTOR);
  let site = get_inner_text(&fragment, SITE_SELECTOR);

  return ResponseBody {
    username: username.to_string(),
    name: name,
    bio: bio,
    site: site,
  };
}

fn handler(request: Request<()>) -> http::Result<Response<String>> {
    let uri_str = request.uri().to_string();
    let url = Url::parse(&uri_str).unwrap();

    if url.path_segments().is_none() {
      return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Unknown request. Usage: https://gh-profile.now.sh/username".to_string())
    }

    let username = url.path().replace("/", "");
    if username == "" {
      return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Missing username. Usage: https://gh-profile.now.sh/username".to_string())
    }

    let url = format!("https://github.com/{}", username);
    let url = Url::parse(&url).expect("Failed to parse URL");
    let client = Client::new();
    let mut res = client
      .get(url)
      .header(header::ACCEPT, "text/html")
      .send()
      .expect("Failed to send HTTP request");

    assert_eq!(res.status(), StatusCode::OK);
    let html = res.text().expect("Failed to get HTML");
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

// // test
// fn main() {
//   let mut request = Request::builder();
//   request.uri("https://gh-profile.now.sh/pveyes");

//   let res = handler(request.body(()).unwrap()).unwrap();
//   let body = res.body();
//   println!("{}", body);
// }
