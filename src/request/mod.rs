use crate::common::Version;

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

// internal module
mod query;
pub use query::QueryError;

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
        crate::query::to_query(self)
    }

    fn from_query(query: &str) -> Result<Self, crate::query::QueryParseError> {
        crate::query::from_query(query)
    }
}

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
    fn test_subsonic_request() {
        let request = chat::AddChatMessage {
            message: "Hello".to_string(),
        };
        assert_eq!("message=Hello", query::to_query(&request));

        let request = annotation::Unstar {
            id: vec!["1".to_string(), "2".to_string()],
            album_id: vec!["3".to_string(), "4".to_string()],
            artist_id: vec!["5".to_string(), "6".to_string()],
        };
        assert_eq!(
            "id=1&id=2&albumId=3&albumId=4&artistId=5&artistId=6",
            query::to_query(&request)
        );
    }

    #[test]
    fn test_deserializer() {
        let query = "message=Hello";
        let request: chat::AddChatMessage = query::from_query(query).unwrap();
        assert_eq!(request.message, "Hello");

        let query = "id=1&id=2&albumId=3&albumId=4&artistId=5&artistId=6";
        let request: annotation::Unstar = query::from_query(query).unwrap();
        assert_eq!(
            annotation::Unstar {
                id: vec!["1".to_string(), "2".to_string()],
                album_id: vec!["3".to_string(), "4".to_string()],
                artist_id: vec!["5".to_string(), "6".to_string()],
            },
            request
        );
    }
}
