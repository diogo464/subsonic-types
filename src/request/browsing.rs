use serde::{Deserialize, Serialize};
use subsonic_macro::{FromQuery, SubsonicRequest, ToQuery};

use crate::common::Milliseconds;

/// Returns all configured top-level music folders. Takes no extra parameters.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getMusicFolders>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.0.0", path = "getMusicFolders")]
pub struct GetMusicFolders;

/// Returns an indexed structure of all artists.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getIndexes>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.0.0", path = "getIndexes")]
pub struct GetIndexes {
    /// If specified, only return artists in the music folder with the given ID.
    /// See [`GetMusicFolders`].
    pub music_folder_id: Option<String>,
    /// If specified, only return a result if the artist collection has changed since the given time.
    pub if_modified_since: Option<Milliseconds>,
}

/// Returns a listing of all files in a music directory. Typically used to get list of albums for an artist, or list of songs for an album.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getMusicDirectory>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.0.0", path = "getMusicDirectory")]
pub struct GetMusicDirectory {
    /// A string which uniquely identifies the music folder. Obtained by calls to getIndexes or getMusicDirectory.
    pub id: String,
}

/// Returns all genres.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getGenres>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.9.0", path = "getGenres")]
pub struct GetGenres;

/// Represents the parameters for the `getArtists` request.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getArtists>.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.8.0", path = "getArtists")]
pub struct GetArtists {
    /// If specified, only return artists in the music folder with the given ID.
    /// See [`GetMusicFolders`].
    pub music_folder_id: Option<u32>,
}

/// Returns details for an artist, including a list of albums.
///
/// See: <http://www.subsonic.org/pages/api.jsp#getArtist>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.8.0", path = "getArtist")]
pub struct GetArtist {
    /// The artist ID.
    pub id: String,
}

/// Returns details for an album, including a list of songs.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getAlbum>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.8.0", path = "getAlbum")]
pub struct GetAlbum {
    /// The album ID.
    pub id: String,
}

/// Returns details for a song.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getSong>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.8.0", path = "getSong")]
pub struct GetSong {
    /// The ID of the song to retrieve.
    pub id: String,
}

/// Represents a request to retrieve all video files.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getVideos>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.8.0", path = "getVideos")]
pub struct GetVideos;

/// Returns details for a video, including information about available audio tracks, subtitles (captions) and conversions.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getVideoInfo>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.14.0", path = "getVideoInfo")]
pub struct GetVideoInfo {
    pub id: String,
}

/// Returns artist info with biography, image URLs and similar artists, using data from <http://last.fm>.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getArtistInfo>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.11.0", path = "getArtistInfo")]
pub struct GetArtistInfo {
    /// The artist, album or song ID.
    pub id: String,
    /// Max number of similar artists to return.
    pub count: Option<u32>,
    /// Whether to return artists that are not present in the media library.
    pub include_not_present: Option<bool>,
}

impl GetArtistInfo {
    pub const DEFAULT_COUNT: u32 = 20;
    pub const DEFAULT_INCLUDE_NOT_PRESENT: bool = false;
}

/// Similar to [`GetArtistInfo`], but organizes music according to ID3 tags.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getArtistInfo2>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.11.0", path = "getArtistInfo2")]
pub struct GetArtistInfo2 {
    /// The artist, album or song ID.
    pub id: String,
    /// Max number of similar artists to return.
    pub count: Option<u32>,
    /// Whether to return artists that are not present in the media library.
    pub include_not_present: Option<bool>,
}

impl GetArtistInfo2 {
    pub const DEFAULT_COUNT: u32 = 20;
    pub const DEFAULT_INCLUDE_NOT_PRESENT: bool = false;
}

/// Returns album notes, image URLs etc, using data from <last.fm>.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getAlbumInfo>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.14.0", path = "getAlbumInfo")]
pub struct GetAlbumInfo {
    /// The album or song ID.
    pub id: String,
}

/// Similar to [`GetAlbumInfo`], but organizes music according to ID3 tags.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getAlbumInfo2>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.14.0", path = "getAlbumInfo2")]
pub struct GetAlbumInfo2 {
    /// The album or song ID.
    pub id: String,
}

/// Returns a random collection of songs from the given artist and similar artists, using data from last.fm. Typically used for artist radio features.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getSimilarSongs>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.11.0", path = "getSimilarSongs")]
pub struct GetSimilarSongs {
    /// The artist, album or song ID.
    pub id: String,
    /// Max number of songs to return.
    pub count: Option<u32>,
}

impl GetSimilarSongs {
    pub const DEFAULT_COUNT: u32 = 50;
}

/// Similar to [`GetSimilarSongs`], but organizes music according to ID3 tags.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getSimilarSongs2>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.11.0", path = "getSimilarSongs2")]
pub struct GetSimilarSongs2 {
    /// The artist, album or song ID.
    pub id: String,
    /// Max number of songs to return.
    pub count: Option<u32>,
}

impl GetSimilarSongs2 {
    pub const DEFAULT_COUNT: u32 = 50;
}

/// Returns top songs for the given artist, using data from <last.fm>.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getTopSongs>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.13.0", path = "getTopSongs")]
pub struct GetTopSongs {
    /// The artist name.
    pub id: String,
    /// Max number of songs to return.
    pub count: Option<u32>,
}

impl GetTopSongs {
    pub const DEFAULT_COUNT: u32 = 50;
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_request_encode;
    use super::*;

    #[test]
    fn test_get_music_folders() {
        let request = GetMusicFolders;
        let query = test_request_encode(&request);
        assert_eq!(query, "");
    }

    #[test]
    fn test_get_indexes() {
        let request = GetIndexes {
            music_folder_id: None,
            if_modified_since: None,
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "");

        let request = GetIndexes {
            music_folder_id: Some("123".to_string()),
            if_modified_since: Some(Milliseconds::new(123)),
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "musicFolderId=123&ifModifiedSince=123");
    }

    #[test]
    fn test_get_music_directory() {
        let request = GetMusicDirectory {
            id: "123".to_string(),
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "id=123");
    }

    #[test]
    fn test_get_genres() {
        let request = GetGenres;
        let query = test_request_encode(&request);
        assert_eq!(query, "");
    }

    #[test]
    fn tet_get_artists() {
        let request = GetArtists {
            music_folder_id: None,
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "");

        let request = GetArtists {
            music_folder_id: Some(20),
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "musicFolderId=20");
    }

    #[test]
    fn test_get_artist() {
        let request = GetArtist {
            id: "123".to_string(),
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "id=123");
    }

    #[test]
    fn test_get_album() {
        let request = GetAlbum {
            id: "123".to_string(),
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "id=123");
    }

    #[test]
    fn test_get_song() {
        let request = GetSong {
            id: "123".to_string(),
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "id=123");
    }

    #[test]
    fn test_get_videos() {
        let request = GetVideos;
        let query = test_request_encode(&request);
        assert_eq!(query, "");
    }

    #[test]
    fn test_get_video_info() {
        let request = GetVideoInfo {
            id: "123".to_string(),
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "id=123");
    }

    #[test]
    fn test_get_artist_info() {
        let request = GetArtistInfo {
            id: "123".to_string(),
            count: None,
            include_not_present: None,
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "id=123");

        let request = GetArtistInfo {
            id: "123".to_string(),
            count: Some(123),
            include_not_present: Some(true),
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "id=123&count=123&includeNotPresent");
    }

    #[test]
    fn test_get_artist_info_2() {
        let request = GetArtistInfo2 {
            id: "123".to_string(),
            count: None,
            include_not_present: None,
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "id=123");

        let request = GetArtistInfo2 {
            id: "123".to_string(),
            count: Some(123),
            include_not_present: Some(true),
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "id=123&count=123&includeNotPresent");
    }

    #[test]
    fn test_get_album_info_2() {
        let request = GetAlbumInfo2 {
            id: "123".to_string(),
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "id=123");
    }

    #[test]
    fn test_get_similar_songs() {
        let request = GetSimilarSongs {
            id: "123".to_string(),
            count: None,
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "id=123");

        let request = GetSimilarSongs {
            id: "123".to_string(),
            count: Some(123),
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "id=123&count=123");
    }

    #[test]
    fn test_get_top_songs() {
        let request = GetTopSongs {
            id: "123".to_string(),
            count: None,
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "id=123");

        let request = GetTopSongs {
            id: "123".to_string(),
            count: Some(123),
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "id=123&count=123");
    }
}
