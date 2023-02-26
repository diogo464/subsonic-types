use reqwest::blocking::get;
use subsonic_types::{
    common::Version,
    request::{self, SubsonicRequest},
    request::{Authentication, Request},
    response::Response,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://localhost:4533/";
    let ping = Request {
        username: "admin".into(),
        authentication: Authentication::Password("admin".into()),
        version: Version::LATEST,
        client: "ping-example".into(),
        format: None,
        body: request::system::Ping,
    };
    let query = ping.to_query();
    let request_url = format!("{}rest/{}?{}", base_url, request::system::Ping::PATH, query);
    println!("Request url: {}", request_url);
    let response_body = get(&request_url).unwrap().text().unwrap();
    println!("Response body: {}", response_body);
    let response = Response::from_xml(&response_body)?;
    println!("Response: {:#?}", response);
    Ok(())
}
