//! Subsonic API types. <http://www.subsonic.org/pages/api.jsp>
//! 
//! # Example
//! ## Creating a ping request to a local subsonic server.
//! ```
//! use reqwest::blocking::get;
//! use subsonic_types::{
//!     common::Version,
//!     request::{self, SubsonicRequest, Authentication, Request},
//!     response::{Response, ResponseStatus, ResponseBody},
//! };
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let base_url = "http://localhost:3000";
//!     let ping = Request {
//!         username: "admin".into(),
//!         authentication: Authentication::Password("admin".into()),
//!         version: Version::LATEST,
//!         client: "ping-example".into(),
//!         format: None,
//!         body: request::system::Ping,
//!     };
//!     let request_url = format!("{}{}?{}", base_url, request::system::Ping::PATH, ping.to_query());
//!     assert_eq!(
//!         request_url,
//!         "http://localhost:3000/rest/ping?u=admin&p=admin&v=1.16.1&c=ping-example"
//!     );
//!     
//!     // let response_contents = get(&request_url)?.text()?;
//!     let response_contents = r#"<subsonic-response status="ok" version="1.16.1"></subsonic-response>"#;
//!     let response = Response::from_xml(response_contents)?;
//!     assert_eq!(
//!         response,
//!         Response { status: ResponseStatus::Ok, version: Version::new(1, 16, 1), body: ResponseBody::Empty }
//!     );
//!     Ok(())
//! }
//! ```

pub(crate) mod deser;
#[macro_use]
pub(crate) mod query;

pub mod common;
pub mod request;
pub mod response;
