use serde::{Deserialize, Serialize};
use subsonic_types_macro::{FromQuery, SubsonicRequest, ToQuery};

#[allow(unused)]
use crate::{
    common::{Milliseconds, UserRating},
    request::lists::GetNowPlaying,
};

/// Attaches a star to a song, album or artist.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#star>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.8.0", path = "star")]
pub struct Star {
    /// The ID of the file (song) or folder (album/artist) to star.
    #[serde(default)]
    pub id: Vec<String>,
    /// The ID of an album to star.
    /// Use this rather than id if the client accesses the media collection according to ID3 tags rather than file structure.
    #[serde(default)]
    pub album_id: Vec<String>,
    /// The ID of an artist to star.
    /// Use this rather than id if the client accesses the media collection according to ID3 tags rather than file structure.
    #[serde(default)]
    pub artist_id: Vec<String>,
}

/// Removes the star from a song, album or artist.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#unstar>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.8.0", path = "unstar")]
pub struct Unstar {
    /// The ID of the file (song) or folder (album/artist) to star.
    #[serde(default)]
    pub id: Vec<String>,
    /// The ID of an album to star.
    /// Use this rather than id if the client accesses the media collection according to ID3 tags rather than file structure.
    #[serde(default)]
    pub album_id: Vec<String>,
    /// The ID of an artist to star.
    /// Use this rather than id if the client accesses the media collection according to ID3 tags rather than file structure.
    #[serde(default)]
    pub artist_id: Vec<String>,
}

/// Sets the rating for a music file.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#setRating>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.6.0", path = "setRating")]
pub struct SetRating {
    /// A string which uniquely identifies the file (song) or folder (album/artist) to rate.
    pub id: String,
    /// The rating between 1 and 5 (inclusive), or 0 to remove the rating.
    pub rating: UserRating,
}

/// Registers the local playback of one or more media files.
/// Typically used when playing media that is cached on the client.
/// This operation includes the following:
/// - "Scrobbles" the media files on last.fm if the user has configured his/her last.fm credentials on the Subsonic server (Settings > Personal).
/// - Updates the play count and last played timestamp for the media files. (Since 1.11.0)
/// - Makes the media files appear in the "Now playing" page in the web app, and appear in the list of songs returned by [`GetNowPlaying`] (Since 1.11.0)
/// Since 1.8.0 you may specify multiple id (and optionally time) parameters to scrobble multiple files.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#scrobble>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.5.0", path = "scrobble")]
pub struct Scrobble {
    /// A string which uniquely identifies the file to scrobble.
    #[serde(default)]
    pub id: Vec<String>,
    /// The time at which the song was listened to.
    #[serde(default)]
    pub time: Vec<Milliseconds>,
    /// Whether this is a "submission" or a "now playing" notification.
    pub submission: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::request::SubsonicRequest;

    use super::super::tests::test_request_encode;
    use super::*;

    #[test]
    fn test_star() {
        let request = Star {
            id: vec!["1".to_string()],
            album_id: vec![],
            artist_id: vec![],
        };
        let encoded = test_request_encode(&request);
        assert_eq!(encoded, "id=1");

        let request = Star {
            id: vec![],
            album_id: vec!["1".to_string()],
            artist_id: vec![],
        };
        let encoded = test_request_encode(&request);
        assert_eq!(encoded, "albumId=1");

        let request = Star {
            id: vec![],
            album_id: vec![],
            artist_id: vec!["1".to_string()],
        };
        let encoded = test_request_encode(&request);
        assert_eq!(encoded, "artistId=1");

        let request = Star {
            id: vec!["1".to_string()],
            album_id: vec!["2".to_string()],
            artist_id: vec!["3".to_string()],
        };
        let encoded = test_request_encode(&request);
        assert_eq!(encoded, "id=1&albumId=2&artistId=3");

        let request = Star {
            id: vec!["1".to_string(), "2".to_string()],
            album_id: vec!["3".to_string(), "4".to_string()],
            artist_id: vec!["5".to_string(), "6".to_string()],
        };
        let encoded = test_request_encode(&request);
        assert_eq!(
            encoded,
            "id=1&id=2&albumId=3&albumId=4&artistId=5&artistId=6"
        );
    }

    #[test]
    fn test_unstar() {
        let request = Unstar {
            id: vec!["1".to_string()],
            album_id: vec![],
            artist_id: vec![],
        };
        let encoded = test_request_encode(&request);
        assert_eq!(encoded, "id=1");

        let request = Unstar {
            id: vec![],
            album_id: vec!["1".to_string()],
            artist_id: vec![],
        };
        let encoded = test_request_encode(&request);
        assert_eq!(encoded, "albumId=1");

        let request = Unstar {
            id: vec![],
            album_id: vec![],
            artist_id: vec!["1".to_string()],
        };
        let encoded = test_request_encode(&request);
        assert_eq!(encoded, "artistId=1");

        let request = Unstar {
            id: vec!["1".to_string()],
            album_id: vec!["2".to_string()],
            artist_id: vec!["3".to_string()],
        };
        let encoded = test_request_encode(&request);
        assert_eq!(encoded, "id=1&albumId=2&artistId=3");

        let request = Unstar {
            id: vec!["1".to_string(), "2".to_string()],
            album_id: vec!["3".to_string(), "4".to_string()],
            artist_id: vec!["5".to_string(), "6".to_string()],
        };
        let encoded = test_request_encode(&request);
        assert_eq!(
            encoded,
            "id=1&id=2&albumId=3&albumId=4&artistId=5&artistId=6"
        );
    }

    #[test]
    fn test_set_rating() {
        let request = SetRating {
            id: "1".to_string(),
            rating: UserRating::new(5).unwrap(),
        };
        let encoded = test_request_encode(&request);
        assert_eq!(encoded, "id=1&rating=5");

        let request = SetRating {
            id: "1".to_string(),
            rating: UserRating::new(2).unwrap(),
        };
        let encoded = test_request_encode(&request);
        assert_eq!(encoded, "id=1&rating=2");

        let encoded = "id=1";
        SetRating::from_query(encoded).unwrap_err();
    }

    #[test]
    fn test_scrobble() {
        let request = Scrobble {
            id: vec!["1".to_string()],
            time: vec![],
            submission: None,
        };
        let encoded = test_request_encode(&request);
        assert_eq!(encoded, "id=1");

        let request = Scrobble {
            id: vec![],
            time: vec![Milliseconds::new(1)],
            submission: None,
        };
        let encoded = test_request_encode(&request);
        assert_eq!(encoded, "time=1");

        let request = Scrobble {
            id: vec!["1".to_string()],
            time: vec![Milliseconds::new(1)],
            submission: None,
        };
        let encoded = test_request_encode(&request);
        assert_eq!(encoded, "id=1&time=1");

        let request = Scrobble {
            id: vec!["1".to_string(), "2".to_string()],
            time: vec![Milliseconds::new(1), Milliseconds::new(2)],
            submission: None,
        };
        let encoded = test_request_encode(&request);
        assert_eq!(encoded, "id=1&id=2&time=1&time=2");

        let request = Scrobble {
            id: vec!["1".to_string(), "2".to_string()],
            time: vec![Milliseconds::new(1), Milliseconds::new(2)],
            submission: Some(true),
        };
        let encoded = test_request_encode(&request);
        assert_eq!(encoded, "id=1&id=2&time=1&time=2&submission");
    }
}
