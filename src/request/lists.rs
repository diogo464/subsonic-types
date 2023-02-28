use serde::{Deserialize, Serialize};
use subsonic_types_macro::{FromQuery, SubsonicRequest, ToQuery};

#[allow(unused)]
use crate::request::browsing::{GetGenres, GetMusicFolders};
use crate::{impl_from_query_value_for_parse, impl_to_query_value_for_display};

#[derive(Debug)]
pub struct InvalidListType;

impl std::fmt::Display for InvalidListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid list type")
    }
}

impl std::error::Error for InvalidListType {}

#[derive(Debug, Clone, PartialEq)]
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
impl_to_query_value_for_display!(ListType);
impl_from_query_value_for_parse!(ListType);

impl ListType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ListType::Random => "random",
            ListType::Newest => "newest",
            ListType::Highest => "highest",
            ListType::Frequent => "frequent",
            ListType::Recent => "recent",
            ListType::AlphabeticalByName => "alphabeticalByName",
            ListType::AlphabeticalByArtist => "alphabeticalByArtist",
            ListType::Starred => "starred",
            ListType::ByYear => "byYear",
            ListType::ByGenre => "byGenre",
        }
    }
}

impl std::fmt::Display for ListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ListType {
    type Err = InvalidListType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "random" => Ok(ListType::Random),
            "newest" => Ok(ListType::Newest),
            "highest" => Ok(ListType::Highest),
            "frequent" => Ok(ListType::Frequent),
            "recent" => Ok(ListType::Recent),
            "alphabeticalByName" => Ok(ListType::AlphabeticalByName),
            "alphabeticalByArtist" => Ok(ListType::AlphabeticalByArtist),
            "starred" => Ok(ListType::Starred),
            "byYear" => Ok(ListType::ByYear),
            "byGenre" => Ok(ListType::ByGenre),
            _ => Err(InvalidListType),
        }
    }
}

impl serde::Serialize for ListType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ListType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

/// Returns a list of random, newest, highest rated etc. albums. Similar to the album lists on the home page of the Subsonic web interface.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getAlbumList>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.2.0", path = "getAlbumList")]
pub struct GetAlbumList {
    /// See [`ListType`].
    #[serde(rename = "type")]
    #[query(rename = "type")]
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
    /// Since 1.11.0
    /// Only return albums in the music folder with the given ID. See [`GetMusicFolders`].
    pub music_folder_id: Option<String>,
}

/// Similar to [`GetAlbumList`], but organizes music according to ID3 tags.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getAlbumList2>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.8.0", path = "getAlbumList2")]
pub struct GetAlbumList2 {
    /// See [`ListType`].
    #[serde(rename = "type")]
    #[query(rename = "type")]
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
    /// Since 1.11.0
    /// Only return albums in the music folder with the given ID. See [`GetMusicFolders`].
    pub music_folder_id: Option<String>,
}

/// Returns random songs matching the given criteria.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getRandomSongs>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.2.0", path = "getRandomSongs")]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.9.0", path = "getSongsByGenre")]
pub struct GetSongsByGenre {
    /// The genre, as returned by [`GetGenres`].
    pub genre: String,
    /// The number of songs to return. Max 500.
    pub count: Option<u32>,
    /// The offset. Useful if you for example want to page through the list of songs.
    pub offset: Option<u32>,
    /// Since 1.12.0
    /// Only return albums in the music folder with the given ID. See [`GetMusicFolders`].
    pub music_folder_id: Option<String>,
}

/// Returns what is currently being played by all users. Takes no extra parameters.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getNowPlaying>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.0.0", path = "getNowPlaying")]
pub struct GetNowPlaying;

/// Returns starred songs, albums and artists.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getStarred>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.8.0", path = "getStarred")]
pub struct GetStarred {
    /// Since 1.12.0
    /// Only return albums in the music folder with the given ID. See [`GetMusicFolders`].
    pub music_folder_id: Option<String>,
}

/// Returns starred songs, albums and artists.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getStarred2>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.12.0", path = "getStarred2")]
pub struct GetStarred2 {
    /// Since 1.12.0
    /// Only return albums in the music folder with the given ID. See [`GetMusicFolders`].
    pub music_folder_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_request_encode;
    use super::*;

    #[test]
    fn test_get_album_list() {
        let request = GetAlbumList {
            list_type: ListType::ByGenre,
            size: Some(10),
            offset: Some(0),
            genre: Some("Rock".to_string()),
            from_year: None,
            to_year: None,
            music_folder_id: None,
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "type=byGenre&size=10&offset=0&genre=Rock");
    }
}
