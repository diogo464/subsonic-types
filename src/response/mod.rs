//! Module for Subsonic API responses.
//!
//! # Example
//! Building a response:
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     use subsonic_types::{common::Version, response::{Response, ResponseBody, License}};
//!     let response = Response::ok(
//!         Version::V1_16_1,
//!         ResponseBody::License(License {
//!             valid: true,
//!             ..Default::default()
//!         }),
//!     );
//!     assert_eq!(
//!         r#"{"subsonic-response":{"status":"ok","version":"1.16.1","license":{"valid":true}}}"#,
//!         Response::to_json(&response)?
//!     );
//! # Ok(())
//! # }
//! ```
//!
//! Parsing a response:
//! Deserialize a response from json
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     use subsonic_types::{common::Version, response::{Response, ResponseBody, License}};
//!     let response = Response::ok(
//!         Version::V1_16_1,
//!         ResponseBody::License(License {
//!             valid: true,
//!             ..Default::default()
//!         }),
//!     );
//!     let serialized = r#"{"subsonic-response":{"status":"ok","version":"1.16.1","license":{"valid":true}}}"#;
//!     let deserialized = Response::from_json(serialized)?;
//!     assert_eq!(
//!         response,
//!         deserialized
//!     );
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use subsonic_macro::SubsonicType;

use crate::{
    common::{
        AverageRating, DateTime, Format, MediaType, Milliseconds, Seconds, UserRating, Version,
    },
    deser::{SubsonicDeserialize, SubsonicSerialize, SubsonicSerializeWrapper},
};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, SubsonicType)]
#[serde(rename_all = "lowercase")]
#[subsonic(serde)]
pub enum ResponseStatus {
    Ok,
    Failed,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Response {
    #[subsonic(attribute)]
    pub status: ResponseStatus,
    #[subsonic(attribute)]
    pub version: Version,
    #[subsonic(flatten)]
    pub body: ResponseBody,
}

impl Response {
    pub fn ok(version: Version, body: ResponseBody) -> Self {
        Self {
            status: ResponseStatus::Ok,
            version,
            body: body,
        }
    }

    pub fn ok_empty(version: Version) -> Self {
        Self {
            status: ResponseStatus::Ok,
            version,
            body: ResponseBody::Empty,
        }
    }

    pub fn failed(version: Version, error: Error) -> Self {
        Self {
            status: ResponseStatus::Failed,
            version,
            body: ResponseBody::Error(error),
        }
    }

    /// Serialize a response to json
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     use subsonic_types::{common::Version, response::{Response, ResponseBody, License}};
    ///     let response = Response::ok(
    ///         Version::V1_16_1,
    ///         ResponseBody::License(License {
    ///             valid: true,
    ///             ..Default::default()
    ///         }),
    ///     );
    ///     assert_eq!(
    ///         r#"{"subsonic-response":{"status":"ok","version":"1.16.1","license":{"valid":true}}}"#,
    ///         Response::to_json(&response)?
    ///     );
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_json(&self) -> Result<String, Error> {
        self.to_json_versioned(Version::LATEST)
    }

    /// Same as [`Response::to_json`] but allows specifying the api version.
    pub fn to_json_versioned(&self, version: Version) -> Result<String, Error> {
        pub struct SubsonicResponse<'a> {
            subsonic_response: &'a Response,
        }

        impl<'a> SubsonicSerialize for SubsonicResponse<'a> {
            fn serialize<S>(
                &self,
                serializer: S,
                format: Format,
                version: Version,
            ) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(
                    "subsonic-response",
                    &SubsonicSerializeWrapper(self.subsonic_response, format, version),
                )?;
                map.end()
            }
        }

        let response = SubsonicResponse {
            subsonic_response: self,
        };
        let mut buffer = Vec::new();
        let mut serializer = serde_json::Serializer::new(&mut buffer);
        <SubsonicResponse as SubsonicSerialize>::serialize(
            &response,
            &mut serializer,
            Format::Json,
            version,
        )
        .map_err(Error::custom)?;
        Ok(String::from_utf8(buffer).unwrap())
    }

    /// Deserialize a response from json
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     use subsonic_types::{common::Version, response::{Response, ResponseBody, License}};
    ///     let response = Response::ok(
    ///         Version::V1_16_1,
    ///         ResponseBody::License(License {
    ///             valid: true,
    ///             ..Default::default()
    ///         }),
    ///     );
    ///     let serialized = r#"{"subsonic-response":{"status":"ok","version":"1.16.1","license":{"valid":true}}}"#;
    ///     let deserialized = Response::from_json(serialized)?;
    ///     assert_eq!(
    ///         response,
    ///         deserialized
    ///     );
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_json(content: &str) -> Result<Self, Error> {
        Self::from_json_versioned(content, Version::LATEST)
    }

    /// Same as [`Response::from_json`] but allows specifying the api version.
    pub fn from_json_versioned(content: &str, version: Version) -> Result<Self, Error> {
        #[derive(SubsonicType)]
        pub struct SubsonicResponse {
            #[subsonic(rename = "subsonic-response")]
            subsonic_response: Response,
        }

        let seed = <SubsonicResponse as SubsonicDeserialize>::Seed::from((Format::Json, version));
        let mut deserializer = serde_json::Deserializer::from_str(content);
        let response = serde::de::DeserializeSeed::deserialize(seed, &mut deserializer)
            .map_err(Error::custom)?;
        Ok(response.subsonic_response)
    }

    /// Serialize a response to xml
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     use subsonic_types::{common::Version, response::{Response, ResponseBody, License}};
    ///     let response = Response::ok(
    ///         Version::V1_16_1,
    ///         ResponseBody::License(License {
    ///             valid: true,
    ///             ..Default::default()
    ///         }),
    ///     );
    ///     assert_eq!(
    ///         r#"<subsonic-response status="ok" version="1.16.1"><license valid="true"/></subsonic-response>"#,
    ///         Response::to_xml(&response)?
    ///     );
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_xml(&self) -> Result<String, Error> {
        self.to_xml_versioned(Version::LATEST)
    }

    /// Same as [`Response::to_xml`] but allows specifying the api version.
    pub fn to_xml_versioned(&self, version: Version) -> Result<String, Error> {
        let mut response = String::new();
        let serializer =
            quick_xml::se::Serializer::with_root(&mut response, Some("subsonic-response"))
                .map_err(Error::custom)?;
        <Self as SubsonicSerialize>::serialize(self, serializer, Format::Xml, version)
            .map_err(Error::custom)?;
        Ok(response)
    }

    /// Deserialize a response from xml
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     use subsonic_types::{common::Version, response::{Response, ResponseBody, License}};
    ///     let response = Response::ok(
    ///         Version::V1_16_1,
    ///         ResponseBody::License(License {
    ///             valid: true,
    ///             ..Default::default()
    ///         }),
    ///     );
    ///     let serialized = r#"<subsonic-response status="ok" version="1.16.1"><license valid="true"/></subsonic-response>"#;
    ///     let deserialized = Response::from_xml(serialized)?;
    ///     assert_eq!(
    ///         response,
    ///         deserialized
    ///     );
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_xml(content: &str) -> Result<Self, Error> {
        Self::from_xml_versioned(content, Version::LATEST)
    }

    /// Same as [`Response::from_xml`] but allows specifying the api version.
    pub fn from_xml_versioned(content: &str, version: Version) -> Result<Self, Error> {
        let seed = <Self as SubsonicDeserialize>::Seed::from((Format::Xml, version));
        let mut deserializer = quick_xml::de::Deserializer::from_str(content);
        let response = serde::de::DeserializeSeed::deserialize(seed, &mut deserializer)
            .map_err(Error::custom)?;
        Ok(response)
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseBody {
    Empty,
    MusicFolders(MusicFolders),
    Indexes(Indexes),
    Directory(Directory),
    Genres(Genres),
    Artists(ArtistsID3),
    Artist(ArtistWithAlbumsID3),
    Album(AlbumWithSongsID3),
    Song(Child),
    Videos(Videos),
    VideoInfo(VideoInfo),
    NowPlaying(NowPlaying),
    SearchResult(SearchResult),
    SearchResult2(SearchResult2),
    SearchResult3(SearchResult3),
    Playlists(Playlists),
    Playlist(PlaylistWithSongs),
    JukeboxStatus(JukeboxStatus),
    JukeboxPlaylist(JukeboxPlaylist),
    License(License),
    Users(Users),
    User(User),
    ChatMessages(ChatMessages),
    AlbumList(AlbumList),
    AlbumList2(AlbumList2),
    RandomSongs(Songs),
    SongsByGenre(Songs),
    Lyrics(Lyrics),
    Podcasts(Podcasts),
    NewestPodcasts(NewestPodcasts),
    InternetRadioStations(InternetRadioStations),
    Bookmarks(Bookmarks),
    PlayQueue(PlayQueue),
    Shares(Shares),
    Starred(Starred),
    Starred2(Starred2),
    AlbumInfo(AlbumInfo),
    ArtistInfo(ArtistInfo),
    ArtistInfo2(ArtistInfo2),
    SimilarSongs(SimilarSongs),
    SimilarSongs2(SimilarSongs2),
    TopSongs(TopSongs),
    ScanStatus(ScanStatus),
    Error(Error),
}

const _: () = {
    impl SubsonicSerialize for ResponseBody {
        fn serialize<S>(
            &self,
            serializer: S,
            format: Format,
            version: Version,
        ) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeMap;
            let mut map = serializer.serialize_map(Some(1))?;
            match self {
                ResponseBody::Empty => {}
                ResponseBody::MusicFolders(v) => {
                    map.serialize_entry(
                        "musicFolders",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::Indexes(v) => {
                    map.serialize_entry("indexes", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::Directory(v) => {
                    map.serialize_entry(
                        "directory",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::Genres(v) => {
                    map.serialize_entry("genres", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::Artists(v) => {
                    map.serialize_entry("artists", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::Artist(v) => {
                    map.serialize_entry("artist", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::Album(v) => {
                    map.serialize_entry("album", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::Song(v) => {
                    map.serialize_entry("song", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::Videos(v) => {
                    map.serialize_entry("videos", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::VideoInfo(v) => {
                    map.serialize_entry(
                        "videoInfo",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::NowPlaying(v) => {
                    map.serialize_entry(
                        "nowPlaying",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::SearchResult(v) => {
                    map.serialize_entry(
                        "searchResult2",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::SearchResult2(v) => {
                    map.serialize_entry(
                        "searchResult2",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::SearchResult3(v) => {
                    map.serialize_entry(
                        "searchResult3",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::Playlists(v) => {
                    map.serialize_entry(
                        "playlists",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::Playlist(v) => {
                    map.serialize_entry("playlist", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::JukeboxStatus(v) => {
                    map.serialize_entry(
                        "jukeboxStatus",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::JukeboxPlaylist(v) => {
                    map.serialize_entry(
                        "jukeboxPlaylist",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::License(v) => {
                    map.serialize_entry("license", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::Users(v) => {
                    map.serialize_entry("users", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::User(v) => {
                    map.serialize_entry("user", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::ChatMessages(v) => {
                    map.serialize_entry(
                        "chatMessages",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::AlbumList(v) => {
                    map.serialize_entry(
                        "albumList",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::AlbumList2(v) => {
                    map.serialize_entry(
                        "albumList2",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::RandomSongs(v) => {
                    map.serialize_entry(
                        "randomSongs",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::SongsByGenre(v) => {
                    map.serialize_entry(
                        "songsByGenre",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::Lyrics(v) => {
                    map.serialize_entry("lyrics", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::Podcasts(v) => {
                    map.serialize_entry("podcasts", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::NewestPodcasts(v) => {
                    map.serialize_entry(
                        "newestPodcasts",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::InternetRadioStations(v) => {
                    map.serialize_entry(
                        "internetRadioStations",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::Bookmarks(v) => {
                    map.serialize_entry(
                        "bookmarks",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::PlayQueue(v) => {
                    map.serialize_entry(
                        "playQueue",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::Shares(v) => {
                    map.serialize_entry("shares", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::Starred(v) => {
                    map.serialize_entry("starred", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::Starred2(v) => {
                    map.serialize_entry("starred2", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::AlbumInfo(v) => {
                    map.serialize_entry(
                        "albumInfo",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::ArtistInfo(v) => {
                    map.serialize_entry(
                        "artistInfo",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::ArtistInfo2(v) => {
                    map.serialize_entry(
                        "artistInfo2",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::SimilarSongs(v) => {
                    map.serialize_entry(
                        "similarSongs",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::SimilarSongs2(v) => {
                    map.serialize_entry(
                        "similarSongs2",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::TopSongs(v) => {
                    map.serialize_entry("topSongs", &SubsonicSerializeWrapper(v, format, version))?;
                }
                ResponseBody::ScanStatus(v) => {
                    map.serialize_entry(
                        "scanStatus",
                        &SubsonicSerializeWrapper(v, format, version),
                    )?;
                }
                ResponseBody::Error(v) => {
                    map.serialize_entry("error", &SubsonicSerializeWrapper(v, format, version))?;
                }
            }
            map.end()
        }
    }

    pub struct ResponseBodySeed(Format, Version);
    impl From<(Format, Version)> for ResponseBodySeed {
        fn from((format, version): (Format, Version)) -> Self {
            Self(format, version)
        }
    }
    impl<'de> serde::de::Visitor<'de> for ResponseBodySeed {
        type Value = ResponseBody;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a subsonic response body")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "musicFolders" => {
                        let folders = map.next_value_seed(
                            <MusicFolders as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::MusicFolders(folders));
                    }
                    "indexes" => {
                        let indexes = map.next_value_seed(
                            <Indexes as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::Indexes(indexes));
                    }
                    "directory" => {
                        let directory = map.next_value_seed(
                            <Directory as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::Directory(directory));
                    }
                    "genres" => {
                        let genres = map.next_value_seed(
                            <Genres as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::Genres(genres));
                    }
                    "artists" => {
                        let artists = map.next_value_seed(
                            <ArtistsID3 as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::Artists(artists));
                    }
                    "artist" => {
                        let artist = map.next_value_seed(
                            <ArtistWithAlbumsID3 as SubsonicDeserialize>::Seed::from((
                                self.0, self.1,
                            )),
                        )?;
                        return Ok(ResponseBody::Artist(artist));
                    }
                    "album" => {
                        let album = map.next_value_seed(
                            <AlbumWithSongsID3 as SubsonicDeserialize>::Seed::from((
                                self.0, self.1,
                            )),
                        )?;
                        return Ok(ResponseBody::Album(album));
                    }
                    "song" => {
                        let song = map.next_value_seed(
                            <Child as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::Song(song));
                    }
                    "videos" => {
                        let videos = map.next_value_seed(
                            <Videos as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::Videos(videos));
                    }
                    "videoInfo" => {
                        let video_info = map.next_value_seed(
                            <VideoInfo as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::VideoInfo(video_info));
                    }
                    "nowPlaying" => {
                        let now_playing = map.next_value_seed(
                            <NowPlaying as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::NowPlaying(now_playing));
                    }
                    "searchResult" => {
                        let search_result = map.next_value_seed(
                            <SearchResult as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::SearchResult(search_result));
                    }
                    "searchResult2" => {
                        let search_result = map.next_value_seed(
                            <SearchResult2 as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::SearchResult2(search_result));
                    }
                    "searchResult3" => {
                        let search_result = map.next_value_seed(
                            <SearchResult3 as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::SearchResult3(search_result));
                    }
                    "playlists" => {
                        let playlists = map.next_value_seed(
                            <Playlists as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::Playlists(playlists));
                    }
                    "playlist" => {
                        let playlist = map.next_value_seed(
                            <PlaylistWithSongs as SubsonicDeserialize>::Seed::from((
                                self.0, self.1,
                            )),
                        )?;
                        return Ok(ResponseBody::Playlist(playlist));
                    }
                    "jukeboxStatus" => {
                        let jukebox_status = map.next_value_seed(
                            <JukeboxStatus as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::JukeboxStatus(jukebox_status));
                    }
                    "jukeboxPlaylist" => {
                        let jukebox_playlist = map.next_value_seed(
                            <JukeboxPlaylist as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::JukeboxPlaylist(jukebox_playlist));
                    }
                    "license" => {
                        let license = map.next_value_seed(
                            <License as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::License(license));
                    }
                    "users" => {
                        let users = map.next_value_seed(
                            <Users as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::Users(users));
                    }
                    "user" => {
                        let user = map.next_value_seed(
                            <User as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::User(user));
                    }
                    "chatMessages" => {
                        let chat_messages = map.next_value_seed(
                            <ChatMessages as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::ChatMessages(chat_messages));
                    }
                    "albumList" => {
                        let album_list = map.next_value_seed(
                            <AlbumList as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::AlbumList(album_list));
                    }
                    "albumList2" => {
                        let album_list = map.next_value_seed(
                            <AlbumList2 as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::AlbumList2(album_list));
                    }
                    "randomSongs" => {
                        let random_songs = map.next_value_seed(
                            <Songs as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::RandomSongs(random_songs));
                    }
                    "songsByGenre" => {
                        let songs_by_genre = map.next_value_seed(
                            <Songs as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::SongsByGenre(songs_by_genre));
                    }
                    "lyrics" => {
                        let lyrics = map.next_value_seed(
                            <Lyrics as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::Lyrics(lyrics));
                    }
                    "podcasts" => {
                        let podcasts = map.next_value_seed(
                            <Podcasts as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::Podcasts(podcasts));
                    }
                    "newestPodcasts" => {
                        let podcasts = map.next_value_seed(
                            <NewestPodcasts as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::NewestPodcasts(podcasts));
                    }
                    "internetRadioStations" => {
                        let stations = map.next_value_seed(
                            <InternetRadioStations as SubsonicDeserialize>::Seed::from((
                                self.0, self.1,
                            )),
                        )?;
                        return Ok(ResponseBody::InternetRadioStations(stations));
                    }
                    "bookmarks" => {
                        let bookmarks = map.next_value_seed(
                            <Bookmarks as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::Bookmarks(bookmarks));
                    }
                    "playQueue" => {
                        let play_queue = map.next_value_seed(
                            <PlayQueue as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::PlayQueue(play_queue));
                    }
                    "shares" => {
                        let shares = map.next_value_seed(
                            <Shares as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::Shares(shares));
                    }
                    "starred" => {
                        let starred = map.next_value_seed(
                            <Starred as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::Starred(starred));
                    }
                    "starred2" => {
                        let starred = map.next_value_seed(
                            <Starred2 as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::Starred2(starred));
                    }
                    "albumInfo" => {
                        let album_info = map.next_value_seed(
                            <AlbumInfo as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::AlbumInfo(album_info));
                    }
                    "artistInfo" => {
                        let artist_info = map.next_value_seed(
                            <ArtistInfo as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::ArtistInfo(artist_info));
                    }
                    "artistInfo2" => {
                        let artist_info = map.next_value_seed(
                            <ArtistInfo2 as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::ArtistInfo2(artist_info));
                    }
                    "similarSongs" => {
                        let similar_songs = map.next_value_seed(
                            <SimilarSongs as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::SimilarSongs(similar_songs));
                    }
                    "similarSongs2" => {
                        let similar_songs = map.next_value_seed(
                            <SimilarSongs2 as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::SimilarSongs2(similar_songs));
                    }
                    "topSongs" => {
                        let top_songs = map.next_value_seed(
                            <TopSongs as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::TopSongs(top_songs));
                    }
                    "scanStatus" => {
                        let scan_status = map.next_value_seed(
                            <ScanStatus as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::ScanStatus(scan_status));
                    }
                    "error" => {
                        let error = map.next_value_seed(
                            <Error as SubsonicDeserialize>::Seed::from((self.0, self.1)),
                        )?;
                        return Ok(ResponseBody::Error(error));
                    }
                    _ => {
                        map.next_value::<serde::de::IgnoredAny>()?;
                    }
                }
            }
            Ok(ResponseBody::Empty)
        }
    }
    impl<'de> serde::de::DeserializeSeed<'de> for ResponseBodySeed {
        type Value = ResponseBody;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_map(self)
        }
    }

    impl<'de> SubsonicDeserialize<'de> for ResponseBody {
        type Seed = ResponseBodySeed;
    }
};

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct License {
    #[subsonic(attribute)]
    pub valid: bool,
    #[subsonic(attribute)]
    pub email: Option<String>,
    #[subsonic(attribute)]
    pub license_expires: Option<DateTime>,
    #[subsonic(attribute)]
    pub trial_expires: Option<DateTime>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct MusicFolders {
    pub music_folder: Vec<MusicFolder>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct MusicFolder {
    #[subsonic(attribute)]
    pub id: u32,
    #[subsonic(attribute)]
    pub name: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Indexes {
    /// Note: Not sure that this is actually milliseconds
    #[subsonic(attribute)]
    pub last_modified: Milliseconds,
    #[subsonic(attribute, since = "1.10.0")]
    pub ignored_articles: String,
    pub shortcut: Vec<Artist>,
    pub index: Vec<Index>,
    #[subsonic(since = "1.7.0")]
    pub child: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Index {
    #[subsonic(attribute)]
    pub name: String,
    pub artist: Vec<Artist>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Artist {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute)]
    pub name: String,
    #[subsonic(attribute, since = "1.16.1")]
    pub artist_image_url: Option<String>,
    #[subsonic(attribute, since = "1.10.1")]
    pub starred: Option<DateTime>,
    #[subsonic(attribute, since = "1.13.0")]
    pub user_rating: Option<UserRating>,
    #[subsonic(attribute, since = "1.13.0")]
    pub average_rating: Option<AverageRating>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Genres {
    pub genre: Vec<Genre>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Genre {
    #[subsonic(attribute, since = "1.10.2")]
    pub song_count: u32,
    #[subsonic(attribute, since = "1.10.2")]
    pub album_count: u32,
    #[subsonic(value)]
    pub name: String,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct ArtistsID3 {
    pub index: Vec<IndexID3>,
    #[subsonic(attribute, since = "1.10.0")]
    pub ignored_articles: String,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct IndexID3 {
    #[subsonic(attribute)]
    pub name: String,
    pub artist: Vec<ArtistID3>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct ArtistID3 {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute)]
    pub name: String,
    #[subsonic(attribute)]
    pub cover_art: Option<String>,
    #[subsonic(attribute, since = "1.16.1")]
    pub artist_image_url: Option<String>,
    #[subsonic(attribute)]
    pub album_count: u32,
    #[subsonic(attribute)]
    pub starred: Option<DateTime>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct ArtistWithAlbumsID3 {
    #[subsonic(flatten)]
    pub artist: ArtistID3,
    pub album: Vec<AlbumID3>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct AlbumID3 {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute)]
    pub name: String,
    #[subsonic(attribute)]
    pub artist: Option<String>,
    #[subsonic(attribute)]
    pub artist_id: Option<String>,
    #[subsonic(attribute)]
    pub cover_art: Option<String>,
    #[subsonic(attribute)]
    pub song_count: u32,
    #[subsonic(attribute)]
    pub duration: u32,
    #[subsonic(attribute, since = "1.14.0")]
    pub play_count: Option<u64>,
    #[subsonic(attribute)]
    pub created: Option<DateTime>,
    #[subsonic(attribute)]
    pub starred: Option<DateTime>,
    #[subsonic(attribute, since = "1.10.1")]
    pub year: Option<u32>,
    #[subsonic(attribute, since = "1.10.1")]
    pub genre: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct AlbumWithSongsID3 {
    #[subsonic(flatten)]
    pub album: AlbumID3,
    pub song: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Videos {
    pub video: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct VideoInfo {
    #[subsonic(attribute)]
    pub id: String,
    pub captions: Vec<Captions>,
    pub audio_track: Vec<AudioTrack>,
    pub conversion: Vec<VideoConversion>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Captions {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute)]
    pub format: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct AudioTrack {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute)]
    pub name: Option<String>,
    #[subsonic(attribute)]
    pub language_code: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct VideoConversion {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute)]
    pub bit_rate: Option<u32>,
    #[subsonic(attribute)]
    pub audio_track_id: Option<u32>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Directory {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute)]
    pub parent: Option<String>,
    #[subsonic(attribute)]
    pub name: String,
    #[subsonic(attribute, since = "1.10.1")]
    pub starred: Option<DateTime>,
    #[subsonic(attribute, since = "1.13.0")]
    pub user_rating: Option<UserRating>,
    #[subsonic(attribute, since = "1.13.0")]
    pub average_rating: Option<AverageRating>,
    #[subsonic(attribute, since = "1.14.0")]
    pub play_count: Option<u64>,
    pub child: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Child {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute)]
    pub parent: Option<String>,
    #[subsonic(attribute)]
    pub is_dir: bool,
    #[subsonic(attribute)]
    pub title: String,
    #[subsonic(attribute)]
    pub album: Option<String>,
    #[subsonic(attribute)]
    pub artist: Option<String>,
    #[subsonic(attribute)]
    pub track: Option<u32>,
    #[subsonic(attribute)]
    pub year: Option<u32>,
    #[subsonic(attribute)]
    pub genre: Option<String>,
    #[subsonic(attribute)]
    pub cover_art: Option<String>,
    #[subsonic(attribute)]
    pub size: Option<u64>,
    #[subsonic(attribute)]
    pub content_type: Option<String>,
    #[subsonic(attribute)]
    pub suffix: Option<String>,
    #[subsonic(attribute)]
    pub transcoded_content_type: Option<String>,
    #[subsonic(attribute)]
    pub transcoded_suffix: Option<String>,
    #[subsonic(attribute)]
    pub duration: Option<Seconds>,
    #[subsonic(attribute)]
    pub bit_rate: Option<u32>,
    #[subsonic(attribute)]
    pub path: Option<String>,
    #[subsonic(attribute, since = "1.4.1")]
    pub is_video: Option<bool>,
    #[subsonic(attribute, since = "1.6.0")]
    pub user_rating: Option<UserRating>,
    #[subsonic(attribute, since = "1.6.0")]
    pub average_rating: Option<AverageRating>,
    #[subsonic(attribute, since = "1.14.0")]
    pub play_count: Option<u64>,
    #[subsonic(attribute, since = "1.8.0")]
    pub disc_number: Option<u32>,
    #[subsonic(attribute, since = "1.8.0")]
    pub created: Option<DateTime>,
    #[subsonic(attribute, since = "1.8.0")]
    pub starred: Option<DateTime>,
    #[subsonic(attribute, since = "1.8.0")]
    pub album_id: Option<String>,
    #[subsonic(attribute, since = "1.8.0")]
    pub artist_id: Option<String>,
    #[subsonic(rename = "type", attribute, since = "1.8.0")]
    pub media_type: Option<MediaType>,
    #[subsonic(attribute, since = "1.10.1")]
    pub bookmark_position: Option<u64>,
    #[subsonic(attribute, since = "1.13.0")]
    pub original_width: Option<u32>,
    #[subsonic(attribute, since = "1.13.0")]
    pub original_height: Option<u32>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct NowPlaying {
    pub entry: Vec<NowPlayingEntry>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct NowPlayingEntry {
    #[subsonic(flatten)]
    pub child: Child,
    #[subsonic(attribute)]
    pub username: String,
    #[subsonic(attribute)]
    pub minutes_ago: u32,
    #[subsonic(attribute)]
    pub player_id: u32,
    #[subsonic(attribute)]
    pub player_name: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct SearchResult {
    #[subsonic(attribute)]
    pub offset: u32,
    #[subsonic(attribute)]
    pub total_hits: u32,
    #[subsonic(rename = "match")]
    pub matches: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct SearchResult2 {
    pub artist: Vec<Artist>,
    pub album: Vec<Child>,
    pub song: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct SearchResult3 {
    pub artist: Vec<ArtistID3>,
    pub album: Vec<AlbumID3>,
    pub song: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Playlists {
    pub playlist: Vec<Playlist>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Playlist {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute)]
    pub name: String,
    #[subsonic(attribute, since = "1.8.0")]
    pub comment: Option<String>,
    #[subsonic(attribute, since = "1.8.0")]
    pub owner: Option<String>,
    #[subsonic(attribute, since = "1.8.0")]
    pub public: Option<bool>,
    #[subsonic(attribute, since = "1.8.0")]
    pub song_count: u32,
    #[subsonic(attribute, since = "1.8.0")]
    pub duration: Seconds,
    #[subsonic(attribute, since = "1.8.0")]
    pub created: DateTime,
    #[subsonic(attribute, since = "1.13.0")]
    pub changed: DateTime,
    #[subsonic(attribute, since = "1.11.0")]
    pub cover_art: Option<String>,
    #[subsonic(since = "1.8.0")]
    pub allowed_user: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct PlaylistWithSongs {
    #[subsonic(flatten)]
    pub playlist: Playlist,
    pub entry: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct JukeboxStatus {
    #[subsonic(attribute)]
    pub current_index: u32,
    #[subsonic(attribute)]
    pub playing: bool,
    #[subsonic(attribute)]
    pub gain: f32,
    #[subsonic(attribute, since = "1.7.0")]
    pub position: Option<u32>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct JukeboxPlaylist {
    #[subsonic(flatten)]
    pub status: JukeboxStatus,
    pub entry: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct ChatMessages {
    pub chat_message: Vec<ChatMessage>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct ChatMessage {
    #[subsonic(attribute)]
    pub username: String,
    #[subsonic(attribute)]
    pub time: DateTime,
    #[subsonic(attribute)]
    pub message: String,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct AlbumList {
    pub album: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct AlbumList2 {
    pub album: Vec<AlbumID3>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Songs {
    pub song: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Lyrics {
    #[subsonic(attribute)]
    pub artist: Option<String>,
    #[subsonic(attribute)]
    pub title: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Podcasts {
    pub channel: Vec<PodcastChannel>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct PodcastChannel {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute)]
    pub url: String,
    #[subsonic(attribute)]
    pub title: Option<String>,
    #[subsonic(attribute)]
    pub description: Option<String>,
    #[subsonic(attribute, since = "1.13.0")]
    pub cover_art: Option<String>,
    #[subsonic(attribute, since = "1.13.0")]
    pub original_image_url: Option<String>,
    #[subsonic(attribute)]
    pub status: PodcastStatus,
    #[subsonic(attribute)]
    pub error_message: Option<String>,
    pub episode: Vec<PodcastEpisode>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct NewestPodcasts {
    pub episode: Vec<PodcastEpisode>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct PodcastEpisode {
    #[subsonic(flatten)]
    pub child: Child,
    /// Use this ID for streaming the podcast
    #[subsonic(attribute)]
    pub stream_id: Option<String>,
    #[subsonic(attribute, since = "1.13.0")]
    pub channel_id: String,
    #[subsonic(attribute)]
    pub description: Option<String>,
    #[subsonic(attribute)]
    pub status: PodcastStatus,
    #[subsonic(attribute)]
    pub publish_date: Option<DateTime>,
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, SubsonicType,
)]
#[subsonic(serde)]
pub enum PodcastStatus {
    New,
    Downloading,
    Completed,
    Skipped,
    #[default]
    Error,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct InternetRadioStations {
    pub internet_radio_station: Vec<InternetRadioStation>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct InternetRadioStation {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute)]
    pub name: String,
    #[subsonic(attribute)]
    pub stream_url: String,
    #[subsonic(attribute)]
    pub home_page_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Bookmarks {
    pub bookmark: Vec<Bookmark>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Bookmark {
    #[subsonic(attribute)]
    pub position: u64,
    #[subsonic(attribute)]
    pub username: String,
    #[subsonic(attribute)]
    pub comment: Option<String>,
    #[subsonic(attribute)]
    pub created: DateTime,
    #[subsonic(attribute)]
    pub changed: DateTime,
    pub entry: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct PlayQueue {
    /// ID of the currently playing song
    #[subsonic(attribute)]
    pub current: Option<u64>, // TODO: u64?
    /// Position of the currently playing track
    #[subsonic(attribute)]
    pub position: Option<Milliseconds>,
    #[subsonic(attribute)]
    pub username: String,
    #[subsonic(attribute)]
    pub changed: DateTime,
    /// Name of client app
    #[subsonic(attribute)]
    pub changed_by: String,
    pub entry: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Shares {
    pub share: Vec<Share>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Share {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute)]
    pub url: String,
    #[subsonic(attribute)]
    pub description: Option<String>,
    #[subsonic(attribute)]
    pub username: String,
    #[subsonic(attribute)]
    pub created: DateTime,
    #[subsonic(attribute)]
    pub expires: Option<DateTime>,
    #[subsonic(attribute)]
    pub last_visited: Option<DateTime>,
    #[subsonic(attribute)]
    pub visit_count: u64,
    pub entry: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Starred {
    pub song: Vec<Child>,
    pub album: Vec<Child>,
    pub artist: Vec<Artist>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct AlbumInfo {
    pub notes: Vec<String>,
    pub music_brainz_id: Vec<String>,
    pub last_fm_url: Vec<String>,
    pub small_image_url: Vec<String>,
    pub medium_image_url: Vec<String>,
    pub large_image_url: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct ArtistInfoBase {
    pub biography: Vec<String>,
    pub music_brainz_id: Vec<String>,
    pub last_fm_url: Vec<String>,
    pub small_image_url: Vec<String>,
    pub medium_image_url: Vec<String>,
    pub large_image_url: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct ArtistInfo {
    #[subsonic(flatten)]
    pub info: ArtistInfoBase,
    pub similar_artist: Vec<Artist>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct ArtistInfo2 {
    #[subsonic(flatten)]
    pub info: ArtistInfoBase,
    pub similar_artist: Vec<ArtistID3>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct SimilarSongs {
    pub song: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct SimilarSongs2 {
    pub song: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct TopSongs {
    pub song: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Starred2 {
    pub song: Vec<Child>,
    pub album: Vec<AlbumID3>,
    pub artist: Vec<ArtistID3>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct ScanStatus {
    #[subsonic(attribute)]
    pub scanning: bool,
    #[subsonic(attribute)]
    pub count: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Users {
    pub user: Vec<User>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct User {
    #[subsonic(attribute)]
    pub username: String,
    #[subsonic(attribute, since = "1.6.0")]
    pub email: Option<String>,
    #[subsonic(attribute, since = "1.7.0")]
    pub scrobbling_enabled: bool,
    #[subsonic(attribute, since = "1.13.0")]
    pub max_bit_rate: Option<u64>,
    #[subsonic(attribute)]
    pub admin_role: bool,
    #[subsonic(attribute)]
    pub settings_role: bool,
    #[subsonic(attribute)]
    pub download_role: bool,
    #[subsonic(attribute)]
    pub upload_role: bool,
    #[subsonic(attribute)]
    pub playlist_role: bool,
    #[subsonic(attribute)]
    pub cover_art_role: bool,
    #[subsonic(attribute)]
    pub comment_role: bool,
    #[subsonic(attribute)]
    pub podcast_role: bool,
    #[subsonic(attribute)]
    pub stream_role: bool,
    #[subsonic(attribute)]
    pub jukebox_role: bool,
    #[subsonic(attribute, since = "1.7.0")]
    pub share_role: bool,
    #[subsonic(attribute, since = "1.14.0")]
    pub video_conversion_role: bool,
    #[subsonic(attribute, since = "1.14.0")]
    pub avatar_last_changed: Option<DateTime>,
    pub folder: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Error {
    #[subsonic(attribute)]
    pub code: ErrorCode,
    #[subsonic(attribute)]
    pub message: Option<String>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u32::from(self.code))?;
        if let Some(message) = &self.message {
            write!(f, ": {}", message)?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn new(code: ErrorCode) -> Self {
        Error {
            code,
            message: None,
        }
    }

    pub fn with_message(code: ErrorCode, message: impl Into<String>) -> Self {
        Error {
            code,
            message: Some(message.into()),
        }
    }

    pub fn custom(err: impl std::error::Error) -> Self {
        Error {
            code: ErrorCode::Generic,
            message: Some(err.to_string()),
        }
    }

    pub fn custom_with_code(code: ErrorCode, err: impl std::error::Error) -> Self {
        Error {
            code,
            message: Some(err.to_string()),
        }
    }
}

macro_rules! error_impl_from {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Error {
                fn from(err: $t) -> Self {
                    Error::custom(err)
                }
            }
        )*
    };
}
error_impl_from!(
    crate::common::InvalidFormat,
    crate::common::InvalidVersion,
    crate::request::lists::InvalidListType,
    crate::common::InvalidVideoSize,
    crate::common::InvalidUserRating,
    crate::common::InvalidAudioBitrate,
    crate::common::InvalidVideoBitrate,
    crate::common::InvalidAverageRating,
    crate::request::jukebox::InvalidJukeboxAction,
    crate::query::QueryParseError
);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SubsonicType)]
#[repr(u32)]
#[subsonic(serde)]
pub enum ErrorCode {
    #[default]
    Generic = 0,
    RequiredParameterMissing = 10,
    IncompatibleClient = 20,
    IncompatibleServer = 30,
    WrongUsernameOrPassword = 40,
    TokenAuthenticationNotSupported = 41,
    UserNotAuthorizedForTheGivenOperation = 50,
    TrialExpired = 60,
    DataNotFound = 70,
    Other(u32),
}

impl From<u32> for ErrorCode {
    fn from(code: u32) -> Self {
        match code {
            0 => ErrorCode::Generic,
            10 => ErrorCode::RequiredParameterMissing,
            20 => ErrorCode::IncompatibleClient,
            30 => ErrorCode::IncompatibleServer,
            40 => ErrorCode::WrongUsernameOrPassword,
            41 => ErrorCode::TokenAuthenticationNotSupported,
            50 => ErrorCode::UserNotAuthorizedForTheGivenOperation,
            60 => ErrorCode::TrialExpired,
            70 => ErrorCode::DataNotFound,
            _ => ErrorCode::Other(code),
        }
    }
}

impl From<ErrorCode> for u32 {
    fn from(code: ErrorCode) -> Self {
        match code {
            ErrorCode::Generic => 0,
            ErrorCode::RequiredParameterMissing => 10,
            ErrorCode::IncompatibleClient => 20,
            ErrorCode::IncompatibleServer => 30,
            ErrorCode::WrongUsernameOrPassword => 40,
            ErrorCode::TokenAuthenticationNotSupported => 41,
            ErrorCode::UserNotAuthorizedForTheGivenOperation => 50,
            ErrorCode::TrialExpired => 60,
            ErrorCode::DataNotFound => 70,
            ErrorCode::Other(code) => code,
        }
    }
}

impl serde::Serialize for ErrorCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(u32::from(*self))
    }
}

impl<'de> serde::Deserialize<'de> for ErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let code = u32::deserialize(deserializer)?;
        Ok(ErrorCode::from(code))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_ping() {
        let xml = r#"
            <subsonic-response status="ok" version="1.1.1"></subsonic-response>
        "#;

        let value = Response::from_xml(xml).unwrap();
        let expected = Response::ok_empty(Version::V1_1_1);
        assert_eq!(value, expected);

        let xml = r#"
        <subsonic-response status="ok" version="1.1.1" ignored="xyz"></subsonic-response>
        "#;

        let value = Response::from_xml(xml).unwrap();
        let expected = Response::ok_empty(Version::V1_1_1);
        assert_eq!(value, expected);
    }

    #[test]
    fn example_get_license() {
        let xml = r#"
        <subsonic-response status="ok" version="1.13.0">
            <license valid="true" email="foo@bar.com" licenseExpires="2019-09-03T14:46:43"/>
        </subsonic-response>
        "#;

        let value = Response::from_xml(xml).unwrap();
        let expected = Response::ok(
            Version::V1_13_0,
            ResponseBody::License(License {
                valid: true,
                email: Some("foo@bar.com".into()),
                license_expires: Some("2019-09-03T14:46:43".parse().unwrap()),
                ..Default::default()
            }),
        );
        eprintln!("{}", expected.to_xml().unwrap());
        assert_eq!(value, expected);
    }

    #[test]
    fn example_get_music_folders() {
        let xml = r#"
        <subsonic-response status="ok" version="1.1.1">
            <musicFolders>
                <musicFolder id="1" name="Music"/>
                <musicFolder id="2" name="Movies"/>
                <musicFolder id="3" name="Incoming"/>
            </musicFolders>
        </subsonic-response>
        "#;

        let value = Response::from_xml(xml).unwrap();
        let expected = Response::ok(
            Version::V1_1_1,
            ResponseBody::MusicFolders(MusicFolders {
                music_folder: vec![
                    MusicFolder {
                        id: 1,
                        name: Some("Music".into()),
                    },
                    MusicFolder {
                        id: 2,
                        name: Some("Movies".into()),
                    },
                    MusicFolder {
                        id: 3,
                        name: Some("Incoming".into()),
                    },
                ],
            }),
        );
        assert_eq!(value, expected);
    }

    #[test]
    fn example_get_indexes() {
        let xml = r#"
        <subsonic-response status="ok" version="1.10.1">
            <indexes lastModified="237462836472342" ignoredArticles="The El La Los Las Le Les">
                <shortcut id="11" name="Audio books"/>
                <shortcut id="10" name="Podcasts"/>
                <index name="A">
                    <artist id="1" name="ABBA"/>
                    <artist id="2" name="Alanis Morisette"/>
                    <artist id="3" name="Alphaville" starred="2013-11-02T12:30:00"/>
                </index>
                <index name="B">
                    <artist name="Bob Dylan" id="4"/>
                </index>
                <child id="111" parent="11" title="Dancing Queen" isDir="false" album="Arrival" artist="ABBA" track="7" year="1978" genre="Pop" coverArt="24" size="8421341" contentType="audio/mpeg" suffix="mp3" duration="146" bitRate="128" path="ABBA/Arrival/Dancing Queen.mp3"/>
                <child id="112" parent="11" title="Money, Money, Money" isDir="false" album="Arrival" artist="ABBA" track="7" year="1978" genre="Pop" coverArt="25" size="4910028" contentType="audio/flac" suffix="flac" transcodedContentType="audio/mpeg" transcodedSuffix="mp3" duration="208" bitRate="128" path="ABBA/Arrival/Money, Money, Money.mp3"/>
            </indexes>
        </subsonic-response>
        "#;

        let value = Response::from_xml(xml).unwrap();
        let expected = Response::ok(
            Version::new(1, 10, 1),
            ResponseBody::Indexes(Indexes {
                shortcut: vec![
                    Artist {
                        id: "11".into(),
                        name: "Audio books".into(),
                        ..Default::default()
                    },
                    Artist {
                        id: "10".into(),
                        name: "Podcasts".into(),
                        ..Default::default()
                    },
                ],
                index: vec![
                    Index {
                        name: "A".into(),
                        artist: vec![
                            Artist {
                                id: "1".into(),
                                name: "ABBA".into(),
                                ..Default::default()
                            },
                            Artist {
                                id: "2".into(),
                                name: "Alanis Morisette".into(),
                                ..Default::default()
                            },
                            Artist {
                                id: "3".into(),
                                name: "Alphaville".into(),
                                starred: Some("2013-11-02T12:30:00".parse().unwrap()),
                                ..Default::default()
                            },
                        ],
                    },
                    Index {
                        name: "B".into(),
                        artist: vec![Artist {
                            id: "4".into(),
                            name: "Bob Dylan".into(),
                            ..Default::default()
                        }],
                    },
                ],
                child: vec![
                    Child {
                        id: "111".into(),
                        parent: Some("11".into()),
                        title: "Dancing Queen".into(),
                        is_dir: false,
                        album: Some("Arrival".into()),
                        artist: Some("ABBA".into()),
                        track: Some(7),
                        year: Some(1978),
                        genre: Some("Pop".into()),
                        cover_art: Some("24".into()),
                        size: Some(8421341),
                        content_type: Some("audio/mpeg".into()),
                        suffix: Some("mp3".into()),
                        duration: Some(Seconds::new(146)),
                        bit_rate: Some(128),
                        path: Some("ABBA/Arrival/Dancing Queen.mp3".into()),
                        ..Default::default()
                    },
                    Child {
                        id: "112".into(),
                        parent: Some("11".into()),
                        title: "Money, Money, Money".into(),
                        is_dir: false,
                        album: Some("Arrival".into()),
                        artist: Some("ABBA".into()),
                        track: Some(7),
                        year: Some(1978),
                        genre: Some("Pop".into()),
                        cover_art: Some("25".into()),
                        size: Some(4910028),
                        content_type: Some("audio/flac".into()),
                        suffix: Some("flac".into()),
                        transcoded_content_type: Some("audio/mpeg".into()),
                        transcoded_suffix: Some("mp3".into()),
                        duration: Some(Seconds::new(208)),
                        bit_rate: Some(128),
                        path: Some("ABBA/Arrival/Money, Money, Money.mp3".into()),
                        ..Default::default()
                    },
                ],
                last_modified: Milliseconds::new(237462836472342),
                ignored_articles: "The El La Los Las Le Les".into(),
            }),
        );
        assert_eq!(value, expected);
    }

    #[test]
    fn exmple_get_music_directory_1() {
        let xml = r#"
        <subsonic-response status="ok" version="1.10.1">
            <directory id="10" parent="9" name="ABBA" starred="2013-11-02T12:30:00">
                <child id="11" parent="10" title="Arrival" artist="ABBA" isDir="true" coverArt="22"/>
                <child id="12" parent="10" title="Super Trouper" artist="ABBA" isDir="true" coverArt="23"/>
            </directory>
        </subsonic-response>
        "#;

        let value = Response::from_xml(xml).unwrap();
        let expected = Response::ok(
            Version::new(1, 10, 1),
            ResponseBody::Directory(Directory {
                id: "10".into(),
                parent: Some("9".into()),
                name: "ABBA".into(),
                starred: Some("2013-11-02T12:30:00".parse().unwrap()),
                child: vec![
                    Child {
                        id: "11".into(),
                        parent: Some("10".into()),
                        title: "Arrival".into(),
                        artist: Some("ABBA".into()),
                        is_dir: true,
                        cover_art: Some("22".into()),
                        ..Default::default()
                    },
                    Child {
                        id: "12".into(),
                        parent: Some("10".into()),
                        title: "Super Trouper".into(),
                        artist: Some("ABBA".into()),
                        is_dir: true,
                        cover_art: Some("23".into()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
        );
        assert_eq!(value, expected);
    }

    #[test]
    fn example_get_music_directory_2() {
        let xml = r#"
        <subsonic-response status="ok" version="1.4.0">
            <directory id="11" parent="1" name="Arrival">
                <child id="111" parent="11" title="Dancing Queen" isDir="false" album="Arrival" artist="ABBA" track="7" year="1978" genre="Pop" coverArt="24" size="8421341" contentType="audio/mpeg" suffix="mp3" duration="146" bitRate="128" path="ABBA/Arrival/Dancing Queen.mp3"/>
                <child id="112" parent="11" title="Money, Money, Money" isDir="false" album="Arrival" artist="ABBA" track="7" year="1978" genre="Pop" coverArt="25" size="4910028" contentType="audio/flac" suffix="flac" transcodedContentType="audio/mpeg" transcodedSuffix="mp3" duration="208" bitRate="128" path="ABBA/Arrival/Money, Money, Money.mp3"/>
            </directory>
        </subsonic-response>
        "#;

        let value = Response::from_xml(xml).unwrap();
        let expected = Response::ok(
            Version::new(1, 4, 0),
            ResponseBody::Directory(Directory {
                id: "11".into(),
                parent: Some("1".into()),
                name: "Arrival".into(),
                child: vec![
                    Child {
                        id: "111".into(),
                        parent: Some("11".into()),
                        title: "Dancing Queen".into(),
                        is_dir: false,
                        album: Some("Arrival".into()),
                        artist: Some("ABBA".into()),
                        track: Some(7),
                        year: Some(1978),
                        genre: Some("Pop".into()),
                        cover_art: Some("24".into()),
                        size: Some(8421341),
                        content_type: Some("audio/mpeg".into()),
                        suffix: Some("mp3".into()),
                        duration: Some(Seconds::new(146)),
                        bit_rate: Some(128),
                        path: Some("ABBA/Arrival/Dancing Queen.mp3".into()),
                        ..Default::default()
                    },
                    Child {
                        id: "112".into(),
                        parent: Some("11".into()),
                        title: "Money, Money, Money".into(),
                        is_dir: false,
                        album: Some("Arrival".into()),
                        artist: Some("ABBA".into()),
                        track: Some(7),
                        year: Some(1978),
                        genre: Some("Pop".into()),
                        cover_art: Some("25".into()),
                        size: Some(4910028),
                        content_type: Some("audio/flac".into()),
                        suffix: Some("flac".into()),
                        transcoded_content_type: Some("audio/mpeg".into()),
                        transcoded_suffix: Some("mp3".into()),
                        duration: Some(Seconds::new(208)),
                        bit_rate: Some(128),
                        path: Some("ABBA/Arrival/Money, Money, Money.mp3".into()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
        );
        assert_eq!(value, expected);
    }

    #[test]
    fn example_get_genres() {
        // The original example is here: http://www.subsonic.org/pages/inc/api/examples/genres_example_1.xml
        // I had to add the &amp; instead of just using & because it looks like that isn't valid.
        // https://stackoverflow.com/questions/12524908/how-can-i-escape-in-xml

        let xml = r#"
            <subsonic-response status="ok" version="1.10.2">
                <genres>
                    <genre songCount="28" albumCount="6">Electronic</genre>
                    <genre songCount="6" albumCount="2">Hard Rock</genre>
                    <genre songCount="8" albumCount="2">R&amp;B</genre>
                    <genre songCount="22" albumCount="2">Blues</genre>
                    <genre songCount="2" albumCount="2">Podcast</genre>
                    <genre songCount="11" albumCount="1">Brit Pop</genre>
                    <genre songCount="14" albumCount="1">Live</genre>
                </genres>
            </subsonic-response>
            "#;

        let value = Response::from_xml(xml).unwrap();
        let expected = Response::ok(
            Version::new(1, 10, 2),
            ResponseBody::Genres(Genres {
                genre: vec![
                    Genre {
                        name: "Electronic".into(),
                        song_count: 28,
                        album_count: 6,
                    },
                    Genre {
                        name: "Hard Rock".into(),
                        song_count: 6,
                        album_count: 2,
                    },
                    Genre {
                        name: "R&B".into(),
                        song_count: 8,
                        album_count: 2,
                    },
                    Genre {
                        name: "Blues".into(),
                        song_count: 22,
                        album_count: 2,
                    },
                    Genre {
                        name: "Podcast".into(),
                        song_count: 2,
                        album_count: 2,
                    },
                    Genre {
                        name: "Brit Pop".into(),
                        song_count: 11,
                        album_count: 1,
                    },
                    Genre {
                        name: "Live".into(),
                        song_count: 14,
                        album_count: 1,
                    },
                ],
            }),
        );
        assert_eq!(value, expected);
    }

    #[test]
    fn example_get_artists() {
        let xml = r#"
        <subsonic-response status="ok" version="1.10.1">
            <artists ignoredArticles="The El La Los Las Le Les">
                <index name="A">
                    <artist id="5449" name="A-Ha" coverArt="ar-5449" albumCount="4"/>
                    <artist id="5421" name="ABBA" coverArt="ar-5421" albumCount="6"/>
                    <artist id="5432" name="AC/DC" coverArt="ar-5432" albumCount="15"/>
                    <artist id="6633" name="Aaron Neville" coverArt="ar-6633" albumCount="1"/>
                </index>
                <index name="B">
                    <artist id="5950" name="Bob Marley" coverArt="ar-5950" albumCount="8"/>
                    <artist id="5957" name="Bruce Dickinson" coverArt="ar-5957" albumCount="2"/>
                </index>
            </artists>
        </subsonic-response>
        "#;

        let value = Response::from_xml(xml).unwrap();
        let expected = Response::ok(
            Version::new(1, 10, 1),
            ResponseBody::Artists(ArtistsID3 {
                ignored_articles: "The El La Los Las Le Les".into(),
                index: vec![
                    IndexID3 {
                        name: "A".into(),
                        artist: vec![
                            ArtistID3 {
                                id: "5449".into(),
                                name: "A-Ha".into(),
                                cover_art: Some("ar-5449".into()),
                                album_count: 4,
                                ..Default::default()
                            },
                            ArtistID3 {
                                id: "5421".into(),
                                name: "ABBA".into(),
                                cover_art: Some("ar-5421".into()),
                                album_count: 6,
                                ..Default::default()
                            },
                            ArtistID3 {
                                id: "5432".into(),
                                name: "AC/DC".into(),
                                cover_art: Some("ar-5432".into()),
                                album_count: 15,
                                ..Default::default()
                            },
                            ArtistID3 {
                                id: "6633".into(),
                                name: "Aaron Neville".into(),
                                cover_art: Some("ar-6633".into()),
                                album_count: 1,
                                ..Default::default()
                            },
                        ],
                    },
                    IndexID3 {
                        name: "B".into(),
                        artist: vec![
                            ArtistID3 {
                                id: "5950".into(),
                                name: "Bob Marley".into(),
                                cover_art: Some("ar-5950".into()),
                                album_count: 8,
                                ..Default::default()
                            },
                            ArtistID3 {
                                id: "5957".into(),
                                name: "Bruce Dickinson".into(),
                                cover_art: Some("ar-5957".into()),
                                album_count: 2,
                                ..Default::default()
                            },
                        ],
                    },
                ],
            }),
        );
        assert_eq!(value, expected);
    }

    #[test]
    fn example_get_artist() {
        let xml = r#"
        <subsonic-response status="ok" version="1.8.0">
            <artist id="5432" name="AC/DC" coverArt="ar-5432" albumCount="15">
                <album id="11047" name="Back In Black" coverArt="al-11047" songCount="10" created="2004-11-08T23:33:11" duration="2534" artist="AC/DC" artistId="5432"/>
                <album id="11048" name="Black Ice" coverArt="al-11048" songCount="15" created="2008-10-30T09:20:52" duration="3332" artist="AC/DC" artistId="5432"/>
                <album id="11050" name="Flick Of The Switch" coverArt="al-11050" songCount="10" created="2004-11-27T19:22:51" duration="2222" artist="AC/DC" artistId="5432"/>
                <album id="11051" name="Fly On The Wall" coverArt="al-11051" songCount="10" created="2004-11-27T19:22:57" duration="2405" artist="AC/DC" artistId="5432"/>
                <album id="11052" name="For Those About To Rock" coverArt="al-11052" songCount="10" created="2004-11-08T23:35:02" duration="2403" artist="AC/DC" artistId="5432"/>
                <album id="11053" name="High Voltage" coverArt="al-11053" songCount="8" created="2004-11-27T20:23:32" duration="2414" artist="AC/DC" artistId="5432"/>
                <album id="10489" name="Highway To Hell" coverArt="al-10489" songCount="12" created="2009-06-15T09:41:54" duration="2745" artist="AC/DC" artistId="5432"/>
                <album id="11054" name="If You Want Blood..." coverArt="al-11054" songCount="1" created="2004-11-27T20:23:32" duration="304" artist="AC/DC" artistId="5432"/>
            </artist>
        </subsonic-response>
        "#;

        let expected = Response::ok(
            Version::new(1, 8, 0),
            ResponseBody::Artist(ArtistWithAlbumsID3 {
                artist: ArtistID3 {
                    id: "5432".into(),
                    name: "AC/DC".into(),
                    cover_art: Some("ar-5432".into()),
                    album_count: 15,
                    ..Default::default()
                },
                album: vec![
                    AlbumID3 {
                        id: "11047".into(),
                        name: "Back In Black".into(),
                        cover_art: Some("al-11047".into()),
                        song_count: 10,
                        created: Some("2004-11-08T23:33:11".parse().unwrap()),
                        duration: 2534,
                        artist: Some("AC/DC".into()),
                        artist_id: Some("5432".into()),
                        ..Default::default()
                    },
                    AlbumID3 {
                        id: "11048".into(),
                        name: "Black Ice".into(),
                        cover_art: Some("al-11048".into()),
                        song_count: 15,
                        created: Some("2008-10-30T09:20:52".parse().unwrap()),
                        duration: 3332,
                        artist: Some("AC/DC".into()),
                        artist_id: Some("5432".into()),
                        ..Default::default()
                    },
                    AlbumID3 {
                        id: "11050".into(),
                        name: "Flick Of The Switch".into(),
                        cover_art: Some("al-11050".into()),
                        song_count: 10,
                        created: Some("2004-11-27T19:22:51".parse().unwrap()),
                        duration: 2222,
                        artist: Some("AC/DC".into()),
                        artist_id: Some("5432".into()),
                        ..Default::default()
                    },
                    AlbumID3 {
                        id: "11051".into(),
                        name: "Fly On The Wall".into(),
                        cover_art: Some("al-11051".into()),
                        song_count: 10,
                        created: Some("2004-11-27T19:22:57".parse().unwrap()),
                        duration: 2405,
                        artist: Some("AC/DC".into()),
                        artist_id: Some("5432".into()),
                        ..Default::default()
                    },
                    AlbumID3 {
                        id: "11052".into(),
                        name: "For Those About To Rock".into(),
                        cover_art: Some("al-11052".into()),
                        song_count: 10,
                        created: Some("2004-11-08T23:35:02".parse().unwrap()),
                        duration: 2403,
                        artist: Some("AC/DC".into()),
                        artist_id: Some("5432".into()),
                        ..Default::default()
                    },
                    AlbumID3 {
                        id: "11053".into(),
                        name: "High Voltage".into(),
                        cover_art: Some("al-11053".into()),
                        song_count: 8,
                        created: Some("2004-11-27T20:23:32".parse().unwrap()),
                        duration: 2414,
                        artist: Some("AC/DC".into()),
                        artist_id: Some("5432".into()),
                        ..Default::default()
                    },
                    AlbumID3 {
                        id: "10489".into(),
                        name: "Highway To Hell".into(),
                        cover_art: Some("al-10489".into()),
                        song_count: 12,
                        created: Some("2009-06-15T09:41:54".parse().unwrap()),
                        duration: 2745,
                        artist: Some("AC/DC".into()),
                        artist_id: Some("5432".into()),
                        ..Default::default()
                    },
                    AlbumID3 {
                        id: "11054".into(),
                        name: "If You Want Blood...".into(),
                        cover_art: Some("al-11054".into()),
                        song_count: 1,
                        created: Some("2004-11-27T20:23:32".parse().unwrap()),
                        duration: 304,
                        artist: Some("AC/DC".into()),
                        artist_id: Some("5432".into()),
                        ..Default::default()
                    },
                ],
            }),
        );
        let value = Response::from_xml(xml).unwrap();

        assert_eq!(expected, value);
    }
}
