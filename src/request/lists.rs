use crate::request::browsing::{GetGenres, GetMusicFolders};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ListType {
    Random,
    Newest,
    Highest,
    Frequent,
    Recent,
    /// Since 1.8.0
    AlphabeticalByName,
    /// Since 1.8.0
    AlphabeticalByArtist,
    /// Since 1.8.0
    Starred,
    /// Since 1.10.1
    ByYear,
    /// Since 1.10.1
    ByGenre,
}

/// Returns a list of random, newest, highest rated etc. albums. Similar to the album lists on the home page of the Subsonic web interface.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getAlbumList>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAlbumList {
    /// See [`ListType`].
    #[serde(rename = "type")] // TODO: rename
    pub list_type: ListType,
    /// The number of albums to return. Max 500.
    pub size: Option<u32>,
    /// The list offset. Useful if you for example want to page through the list of newest albums.
    pub offset: Option<u32>,
    /// The first year in the range. If 'fromYear > toYear' a reverse chronological list is returned.
    /// Required if [`GetAlbumList::list_type`] is [`ListType::ByYear`].
    pub from_year: Option<u32>,
    /// The last year in the range.
    /// Required if [`GetAlbumList::list_type`] is [`ListType::ByYear`].
    pub to_year: Option<u32>,
    /// The name of the genre, e.g., "Rock".
    /// Required if [`GetAlbumList::list_type`] is [`ListType::ByGenre`].
    pub genre: Option<String>,
    /// TODO: Since 1.11.0
    /// Only return albums in the music folder with the given ID. See [`GetMusicFolders`].
    pub music_folder_id: Option<String>,
}

/// Similar to [`GetAlbumList`], but organizes music according to ID3 tags.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getAlbumList2>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAlbumList2 {
    /// See [`ListType`].
    #[serde(rename = "type")] // TODO: rename
    pub list_type: ListType,
    /// The number of albums to return. Max 500.
    pub size: Option<u32>,
    /// The list offset. Useful if you for example want to page through the list of newest albums.
    pub offset: Option<u32>,
    /// The first year in the range. If 'fromYear > toYear' a reverse chronological list is returned.
    /// Required if [`GetAlbumList2::list_type`] is [`ListType::ByYear`].
    pub from_year: Option<u32>,
    /// The last year in the range.
    /// Required if [`GetAlbumList2::list_type`] is [`ListType::ByYear`].
    pub to_year: Option<u32>,
    /// The name of the genre, e.g., "Rock".
    /// Required if [`GetAlbumList2::list_type`] is [`ListType::ByGenre`].
    pub genre: Option<String>,
    /// TODO: Since 1.11.0
    /// Only return albums in the music folder with the given ID. See [`GetMusicFolders`].
    pub music_folder_id: Option<String>,
}

/// Returns random songs matching the given criteria.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getRandomSongs>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRandomSongs {
    /// The maximum number of songs to return. Max 500.
    pub size: Option<u32>,
    /// Only returns songs belonging to this genre.
    pub genre: Option<String>,
    /// Only return songs published after or in this year.
    pub from_year: Option<u32>,
    /// Only return songs published before or in this year.
    pub to_year: Option<u32>,
    /// Only return songs in the music folder with the given ID. See [`GetMusicFolders`].
    pub music_folder_id: Option<String>,
}

/// Returns songs in a given genre.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getSongsByGenre>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSongsByGenre {
    /// The genre, as returned by [`GetGenres`].
    pub genre: String,
    /// The number of songs to return. Max 500.
    pub count: Option<u32>,
    /// The offset. Useful if you for example want to page through the list of songs.
    pub offset: Option<u32>,
    /// TODO: Since 1.12.0
    /// Only return albums in the music folder with the given ID. See [`GetMusicFolders`].
    pub music_folder_id: Option<String>,
}

/// Returns what is currently being played by all users. Takes no extra parameters.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getNowPlaying>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNowPlaying;

/// Returns starred songs, albums and artists.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getStarred>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStarred {
    /// TODO: since 1.12.0
    /// Only return albums in the music folder with the given ID. See [`GetMusicFolders`].
    pub music_folder_id: Option<String>,
}

/// Returns starred songs, albums and artists.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getStarred2>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStarred2 {
    /// TODO: since 1.12.0
    /// Only return albums in the music folder with the given ID. See [`GetMusicFolders`].
    pub music_folder_id: Option<String>,
}
