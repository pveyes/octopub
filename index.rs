extern crate octopub;

use http::{self, Request, Response, StatusCode};
use url::Url;

use octopub::profile;
use octopub::repo;

fn handler(request: Request<()>) -> http::Result<Response<String>> {
  let uri_str = request.uri().to_string();
  let url = Url::parse(&uri_str).unwrap();

  if url.path().contains("/profile") {
    let mut path_segments = url.path_segments().unwrap();
    path_segments.next();

    match path_segments.next() {
      Some(username) => return profile::get_profile_data(&username),
      None => return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Missing username. Usage: https://octopub.now.sh/profile/:username".to_string()),
    }
  }

  if url.path().contains("/repo") {
    let mut path_segments = url.path_segments().unwrap();
    path_segments.next();

    let owner = path_segments.next();
    let repository = path_segments.next();

    match (owner, repository) {
      (Some(owner), Some(repository)) => return repo::get_repository_data(&owner, repository),
      _ => return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Missing owner/repository. Usage: https://octopub.now.sh/repo/:owner/:repository".to_string()),
    }
  }

  return Response::builder()
    .status(StatusCode::BAD_REQUEST)
    .body("Unknown request. See usage at https://github.com/pveyes/octopub".to_string());
}

// // test
// fn main() {
//   let mut request = Request::builder();
//   request.uri("https://octopub.now.sh/profile/pveyes");
//   request.uri("https://octopub.now.sh/repo/pveyes/octopub");

//   let res = handler(request.body(()).unwrap()).unwrap();
//   let body = res.body();
//   println!("{}", body);
// }
