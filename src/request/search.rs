use serde::{Deserialize, Serialize};

#[allow(unused)]
use crate::{common::Milliseconds, request::browsing::GetMusicFolders};

/// Returns a listing of files matching the given search criteria. Supports paging through the result.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#search>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Search {
    /// Artist to search for.
    pub artist: Option<String>,
    /// Album to search for.
    pub album: Option<String>,
    /// Song title to search for.
    pub title: Option<String>,
    /// Searches all fields
    pub any: Option<String>,
    /// Maximum number of results to return.
    pub count: Option<u32>,
    /// Search result offset. Used for paging.
    pub offset: Option<u32>,
    /// Only return matches that are newer than this.
    /// See [`Milliseconds`].
    pub newer_than: Option<Milliseconds>,
}

/// Returns albums, artists and songs matching the given search criteria. Supports paging through the result.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#search2>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Search2 {
    /// Search query.
    pub query: String,
    /// Maximum number of artists to return.
    pub artist_count: Option<u32>,
    /// Search result offset for artists. Used for paging.
    pub artist_offset: Option<u32>,
    /// Maximum number of albums to return.
    pub album_count: Option<u32>,
    /// Search result offset for albums. Used for paging.
    pub album_offset: Option<u32>,
    /// Maximum number of songs to return.
    pub song_count: Option<u32>,
    /// Search result offset for songs. Used for paging.
    pub song_offset: Option<u32>,
    /// Since 1.12.0
    /// Only return results from the music folder with the given ID. See [`GetMusicFolders`].
    pub music_folder_id: Option<String>,
}

/// Similar to [`Search2`], but organizes music according to ID3 tags.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#search3>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Search3 {
    /// Search query.
    pub query: String,
    /// Maximum number of artists to return.
    pub artist_count: Option<u32>,
    /// Search result offset for artists. Used for paging.
    pub artist_offset: Option<u32>,
    /// Maximum number of albums to return.
    pub album_count: Option<u32>,
    /// Search result offset for albums. Used for paging.
    pub album_offset: Option<u32>,
    /// Maximum number of songs to return.
    pub song_count: Option<u32>,
    /// Search result offset for songs. Used for paging.
    pub song_offset: Option<u32>,
    /// Since 1.12.0
    /// Only return results from the music folder with the given ID. See [`GetMusicFolders`].
    pub music_folder_id: Option<String>,
}
