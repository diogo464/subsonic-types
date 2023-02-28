//! Module for Subsonic API requests.
//!
//! # Example
//! Building the request to get a song by ID:
//!
//! ```rust
//! # fn main() {
//!     use subsonic_types::{
//!         common::{Version, Format},
//!         request::{Request, SubsonicRequest, browsing::GetSong, Authentication}
//!     };
//!
//!     let request = Request {
//!         username: "admin".into(),
//!         authentication: Authentication::Password("admin".into()),
//!         version: Version::LATEST,
//!         client: "Rust Example".into(),
//!         format: Some(Format::Json.to_string()),
//!         body: GetSong {
//!             id: "123".into(),
//!        },
//!     };
//!
//!     let query = request.to_query();
//!     assert_eq!("u=admin&p=admin&v=1.16.1&c=Rust%20Example&f=json&id=123", query);
//! # }
//! ```
//! 
//! Parsing a request:
//! ```rust
//! # fn main() {
//!     use subsonic_types::{
//!         common::{Version, Format},
//!         request::{Request, SubsonicRequest, browsing::GetSong, Authentication}
//!     };
//!
//!     let query = "u=admin&p=admin&v=1.16.1&c=Rust%20Example&f=json&id=123";
//!     let request: Request<GetSong> = Request::from_query(query).unwrap();
//!     let expected = Request {
//!         username: "admin".into(),
//!         authentication: Authentication::Password("admin".into()),
//!         version: Version::LATEST,
//!         client: "Rust Example".into(),
//!         format: Some(Format::Json.to_string()),
//!         body: GetSong {
//!             id: "123".into(),
//!        },
//!     };
//!     assert_eq!(expected, request);
//! # }
//! ```

use subsonic_macro::{FromQuery, ToQuery};

use crate::{
    common::Version,
    query::{self, FromQuery, QueryAccumulator, QueryPair, QueryValueParseError, ToQuery},
};

/// System methods
pub mod system;

/// Browing methods
pub mod browsing;

/// Albumm/song lists methods
pub mod lists;

/// Searching methods
pub mod search;

/// Playlists methods
pub mod playlists;

/// Media retrieval methods
pub mod retrieval;

/// Media annotation methods
pub mod annotation;

/// Sharing methods
pub mod sharing;

/// Podcast methods
pub mod podcast;

/// Jukebox methods
pub mod jukebox;

/// Internet radio methods
pub mod radio;

/// Chat methods
pub mod chat;

/// User management methods
pub mod user;

/// Bookmarks methods
pub mod bookmark;

/// Media library scanning methods
pub mod scan;

/// Trait for Subsonic API requests
/// ```rust
/// # fn main() {
///     use subsonic_types::request::SubsonicRequest;
///
///     assert_eq!("/rest/getArtistInfo", subsonic_types::request::browsing::GetArtistInfo::PATH);
///     assert_eq!(subsonic_types::common::Version::V1_11_0, subsonic_types::request::browsing::GetArtistInfo::SINCE);
///
///     let request = subsonic_types::request::browsing::GetArtistInfo {
///         id: "123".into(),
///         count: Some(10),
///         include_not_present: Some(true),
///     };
///     let query = request.to_query();
///     assert_eq!("id=123&count=10&includeNotPresent", query);
///     
///     let parsed = subsonic_types::request::browsing::GetArtistInfo::from_query(&query).unwrap();
///     assert_eq!(request, parsed);
/// # }
/// ```
pub trait SubsonicRequest:
    crate::query::ToQuery
    + crate::query::FromQuery
    + std::fmt::Debug
    + std::cmp::PartialEq
    + std::clone::Clone
{
    const PATH: &'static str;
    const SINCE: Version;

    fn to_query(&self) -> String {
        query::to_query(self)
    }

    fn from_query(query: &str) -> query::Result<Self> {
        query::from_query(query)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Authentication {
    Password(String),
    Token { token: String, salt: String },
}

#[derive(Debug, Clone, PartialEq, ToQuery, FromQuery)]
pub struct Request<R: SubsonicRequest> {
    #[query(rename = "u")]
    pub username: String,
    #[query(flatten)]
    pub authentication: Authentication,
    #[query(rename = "v")]
    pub version: Version,
    #[query(rename = "c")]
    pub client: String,
    #[query(rename = "f")]
    pub format: Option<String>,
    #[query(flatten)]
    pub body: R,
}

impl<R> SubsonicRequest for Request<R>
where
    R: SubsonicRequest,
{
    const PATH: &'static str = R::PATH;

    const SINCE: Version = R::SINCE;
}

impl ToQuery for Authentication {
    fn to_query_builder<B>(&self, builder: &mut B)
    where
        B: crate::query::QueryBuilder,
    {
        match self {
            Authentication::Password(p) => builder.emit_key_value("p", p),
            Authentication::Token { token, salt } => {
                builder.emit_key_value("t", token);
                builder.emit_key_value("s", salt);
            }
        }
    }
}

const _: () = {
    #[derive(Default)]
    pub struct AuthenticationAccum {
        password: Option<String>,
        token: Option<String>,
        salt: Option<String>,
    }

    impl QueryAccumulator for AuthenticationAccum {
        type Output = Authentication;

        fn consume<'a>(
            &mut self,
            pair: crate::query::QueryPair<'a>,
        ) -> crate::query::Result<crate::query::ConsumeStatus<'a>> {
            let (key, value) = (pair.key, pair.value);
            match (key.as_ref(), value) {
                ("p", Some(p)) => {
                    self.password = Some(p.to_string());
                    Ok(crate::query::ConsumeStatus::Consumed)
                }
                ("t", Some(t)) => {
                    self.token = Some(t.to_string());
                    Ok(crate::query::ConsumeStatus::Consumed)
                }
                ("s", Some(s)) => {
                    self.salt = Some(s.to_string());
                    Ok(crate::query::ConsumeStatus::Consumed)
                }
                (_, v) => Ok(crate::query::ConsumeStatus::Ignored(QueryPair {
                    key,
                    value: v,
                })),
            }
        }

        fn finish(self) -> crate::query::Result<Self::Output> {
            if let (Some(token), Some(salt)) = (self.token, self.salt) {
                Ok(Authentication::Token { token, salt })
            } else if let Some(password) = self.password {
                Ok(Authentication::Password(password))
            } else {
                Err(crate::query::QueryParseError::invalid_value(
                    "p/t/s",
                    QueryValueParseError::message("one of p, t, s must be present"),
                ))
            }
        }
    }

    impl FromQuery for Authentication {
        type QueryAccumulator = AuthenticationAccum;
    }
};

#[cfg(test)]
mod tests {
    use super::*;

    pub(super) fn test_request_encode<T>(req: &T) -> String
    where
        T: SubsonicRequest,
    {
        let query = req.to_query();
        let req2: Result<T, _> = crate::query::from_query(&query);
        assert!(
            req2.is_ok(),
            "failed to parse from query: '{}' error: '{}'",
            query,
            req2.unwrap_err()
        );
        assert_eq!(req, &req2.unwrap(), "query: {}", query);
        query
    }

    #[test]
    fn test_ping_request() {
        let req = Request {
            username: "user".to_string(),
            authentication: Authentication::Password("password".to_string()),
            version: Version::new(1, 16, 1),
            client: "test".to_string(),
            format: None,
            body: system::Ping,
        };
        let query = test_request_encode(&req);
        assert_eq!(query, "u=user&p=password&v=1.16.1&c=test");
    }
}
