[![Build](https://img.shields.io/github/actions/workflow/status/diogo464/subsonic-types/rust.yml)]()
[![Crates.io](https://img.shields.io/crates/v/subsonic-types)](https://crates.io/crates/subsonic-types)
[![Docs.rs](https://img.shields.io/docsrs/subsonic-types)](https://docs.rs/subsonic-types)
# subsonic-types

Subsonic API types. <http://www.subsonic.org/pages/api.jsp>

## Example
### Creating a ping request to a local subsonic server.
```rust
use reqwest::blocking::get;
use subsonic_types::{
    common::Version,
    request::{self, SubsonicRequest, Authentication, Request},
    response::{Response, ResponseStatus, ResponseBody},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://localhost:3000";
    let ping = Request {
        username: "admin".into(),
        authentication: Authentication::Password("admin".into()),
        version: Version::LATEST,
        client: "ping-example".into(),
        format: None,
        body: request::system::Ping,
    };
    let request_url = format!("{}{}?{}", base_url, request::system::Ping::PATH, ping.to_query());
    assert_eq!(
        request_url,
        "http://localhost:3000/rest/ping?u=admin&p=admin&v=1.16.1&c=ping-example"
    );

    // let response_contents = get(&request_url)?.text()?;
    let response_contents = r#"<subsonic-response status="ok" version="1.16.1"></subsonic-response>"#;
    let response = Response::from_xml(response_contents)?;
    assert_eq!(
        response,
        Response { status: ResponseStatus::Ok, version: Version::new(1, 16, 1), body: ResponseBody::Empty }
    );
    Ok(())
}
```
