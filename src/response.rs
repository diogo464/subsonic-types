use serde::{Deserialize, Serialize};

use crate::{
    common::{AverageRating, DateTime, MediaType, Milliseconds, UserRating, Version},
    SubsonicType,
};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Ok,
    Failed,
}
impl_subsonic_for_serde!(ResponseStatus);

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Response {
    #[subsonic(attribute)]
    pub status: ResponseStatus,
    #[subsonic(attribute)]
    pub version: Version,
    #[subsonic(flatten, choice)]
    pub body: Option<ResponseBody>,
}

impl Response {
    pub fn ok(version: Version, body: ResponseBody) -> Self {
        Self {
            status: ResponseStatus::Ok,
            version,
            body: Some(body),
        }
    }

    pub fn ok_empty(version: Version) -> Self {
        Self {
            status: ResponseStatus::Ok,
            version,
            body: None,
        }
    }

    pub fn failed(version: Version, error: Error) -> Self {
        Self {
            status: ResponseStatus::Failed,
            version,
            body: Some(ResponseBody::Error(error)),
        }
    }

    pub fn to_json(&self) -> Result<String, crate::SerdeError> {
        crate::to_json(self)
    }

    pub fn from_json(content: &str) -> Result<Self, crate::SerdeError> {
        crate::from_json(content)
    }

    pub fn to_xml(&self) -> Result<String, crate::SerdeError> {
        crate::to_xml(self)
    }

    pub fn from_xml(content: &str) -> Result<Self, crate::SerdeError> {
        crate::from_xml(content)
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub enum ResponseBody {
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
    Playlist(Playlist),
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

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct License {
    #[subsonic(attribute)]
    pub valid: bool,
    #[subsonic(attribute, optional)]
    pub email: Option<String>,
    #[subsonic(attribute, optional)]
    pub license_expires: Option<DateTime>,
    #[subsonic(attribute, optional)]
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
    #[subsonic(attribute, optional)]
    pub name: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Indexes {
    /// Note: Not sure that this is actually milliseconds
    #[subsonic(attribute)]
    pub last_modified: Milliseconds,
    #[subsonic(attribute)]
    pub ignored_articles: String,
    pub shortcut: Vec<Artist>,
    pub index: Vec<Index>,
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
    #[subsonic(attribute, optional)]
    pub artist_image_url: Option<String>,
    #[subsonic(attribute, optional)]
    pub starred: Option<DateTime>,
    #[subsonic(attribute, optional)]
    pub user_rating: Option<UserRating>,
    #[subsonic(attribute, optional)]
    pub average_rating: Option<AverageRating>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Genres {
    pub genre: Vec<Genre>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Genre {
    #[subsonic(attribute)]
    pub song_count: u32,
    #[subsonic(attribute)]
    pub album_count: u32,
    #[subsonic(value)]
    pub name: String,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct ArtistsID3 {
    pub index: Vec<IndexID3>,
    #[subsonic(attribute)]
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
    #[subsonic(attribute, optional)]
    pub cover_art: Option<String>,
    #[subsonic(attribute, optional)]
    pub artist_image_url: Option<String>,
    #[subsonic(attribute)]
    pub album_count: u32,
    #[subsonic(attribute, optional)]
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
    #[subsonic(attribute, optional)]
    pub artist: Option<String>,
    #[subsonic(attribute, optional)]
    pub artist_id: Option<String>,
    #[subsonic(attribute, optional)]
    pub cover_art: Option<String>,
    #[subsonic(attribute)]
    pub song_count: u32,
    #[subsonic(attribute)]
    pub duration: u32,
    #[subsonic(attribute, optional)]
    pub play_count: Option<u64>,
    #[subsonic(attribute, optional)]
    pub created: Option<DateTime>,
    #[subsonic(attribute, optional)]
    pub starred: Option<DateTime>,
    #[subsonic(attribute, optional)]
    pub year: Option<u32>,
    #[subsonic(attribute, optional)]
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
    #[subsonic(attribute, optional)]
    pub format: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct AudioTrack {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute, optional)]
    pub name: Option<String>,
    #[subsonic(attribute, optional)]
    pub language_code: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct VideoConversion {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute, optional)]
    pub bit_rate: Option<u32>,
    #[subsonic(attribute, optional)]
    pub audio_track_id: Option<u32>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Directory {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute, optional)]
    pub parent: Option<String>,
    #[subsonic(attribute)]
    pub name: String,
    #[subsonic(attribute, optional)]
    pub starred: Option<DateTime>,
    #[subsonic(attribute, optional)]
    pub user_rating: Option<UserRating>,
    #[subsonic(attribute, optional)]
    pub average_rating: Option<AverageRating>,
    #[subsonic(attribute, optional)]
    pub play_count: Option<u64>,
    pub child: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Child {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute, optional)]
    pub parent: Option<String>,
    #[subsonic(attribute)]
    pub is_dir: bool,
    #[subsonic(attribute)]
    pub title: String,
    #[subsonic(attribute, optional)]
    pub album: Option<String>,
    #[subsonic(attribute, optional)]
    pub artist: Option<String>,
    #[subsonic(attribute, optional)]
    pub track: Option<u32>,
    #[subsonic(attribute, optional)]
    pub year: Option<u32>,
    #[subsonic(attribute, optional)]
    pub genre: Option<String>,
    #[subsonic(attribute, optional)]
    pub cover_art: Option<String>,
    #[subsonic(attribute, optional)]
    pub size: Option<u64>,
    #[subsonic(attribute, optional)]
    pub content_type: Option<String>,
    #[subsonic(attribute, optional)]
    pub suffix: Option<String>,
    #[subsonic(attribute, optional)]
    pub transcoded_content_type: Option<String>,
    #[subsonic(attribute, optional)]
    pub transcoded_suffix: Option<String>,
    #[subsonic(attribute, optional)]
    pub duration: Option<u32>,
    #[subsonic(attribute, optional)]
    pub bit_rate: Option<u32>,
    #[subsonic(attribute, optional)]
    pub path: Option<String>,
    #[subsonic(attribute, optional)]
    pub is_video: Option<bool>,
    #[subsonic(attribute, optional)]
    pub user_rating: Option<UserRating>,
    #[subsonic(attribute, optional)]
    pub average_rating: Option<AverageRating>,
    #[subsonic(attribute, optional)]
    pub play_count: Option<u64>,
    #[subsonic(attribute, optional)]
    pub disc_number: Option<u32>,
    #[subsonic(attribute, optional)]
    pub created: Option<DateTime>,
    #[subsonic(attribute, optional)]
    pub starred: Option<DateTime>,
    #[subsonic(attribute, optional)]
    pub album_id: Option<String>,
    #[subsonic(attribute, optional)]
    pub artist_id: Option<String>,
    #[subsonic(attribute, optional)]
    pub media_type: Option<MediaType>,
    #[subsonic(attribute, optional)]
    pub bookmark_position: Option<u64>,
    #[subsonic(attribute, optional)]
    pub original_width: Option<u32>,
    #[subsonic(attribute, optional)]
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
    #[subsonic(attribute, optional)]
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
    pub playlists: Vec<Playlist>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct Playlist {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute)]
    pub name: String,
    #[subsonic(attribute, optional)]
    pub comment: Option<String>,
    #[subsonic(attribute, optional)]
    pub owner: Option<String>,
    #[subsonic(attribute, optional)]
    pub public: Option<bool>,
    #[subsonic(attribute)]
    pub song_count: u32,
    #[subsonic(attribute)]
    pub duration: u32,
    #[subsonic(attribute)]
    pub created: DateTime,
    #[subsonic(attribute)]
    pub changed: DateTime,
    #[subsonic(attribute, optional)]
    pub cover_art: Option<String>,
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
    #[subsonic(attribute, optional)]
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
    #[subsonic(attribute, optional)]
    pub cover_art: Option<String>,
    #[subsonic(attribute, optional)]
    pub original_image_url: Option<String>,
    #[subsonic(attribute)]
    pub status: PodcastStatus,
    #[subsonic(attribute, optional)]
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
    #[subsonic(attribute, optional)]
    pub stream_id: Option<String>,
    #[subsonic(attribute)]
    pub channel_id: String,
    #[subsonic(attribute, optional)]
    pub description: Option<String>,
    #[subsonic(attribute)]
    pub status: PodcastStatus,
    #[subsonic(attribute, optional)]
    pub publish_date: Option<DateTime>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PodcastStatus {
    New,
    Downloading,
    Completed,
    Skipped,
    #[default]
    Error,
}
impl_subsonic_for_serde!(PodcastStatus);

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
    #[subsonic(attribute, optional)]
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
    #[subsonic(attribute, optional)]
    pub comment: Option<String>,
    #[subsonic(attribute)]
    pub created: DateTime,
    #[subsonic(attribute)]
    pub changed: DateTime,
    pub entry: Vec<Child>,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct PlayQueue {
    #[subsonic(attribute)]
    pub current: Option<u64>,
    #[subsonic(attribute, optional)]
    pub position: Option<u64>,
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
    #[subsonic(attribute, optional)]
    pub description: Option<String>,
    #[subsonic(attribute)]
    pub username: String,
    #[subsonic(attribute)]
    pub created: DateTime,
    #[subsonic(attribute, optional)]
    pub expires: Option<DateTime>,
    #[subsonic(attribute, optional)]
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
    #[subsonic(attribute, optional)]
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
    #[subsonic(attribute, optional)]
    pub email: Option<String>,
    #[subsonic(attribute)]
    pub scrobbling_enabled: bool,
    #[subsonic(attribute, optional)]
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
    #[subsonic(attribute)]
    pub share_role: bool,
    #[subsonic(attribute)]
    pub video_conversion_role: bool,
    #[subsonic(attribute, optional)]
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
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
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

impl_subsonic_for_serde!(ErrorCode);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_ping() {
        let xml = r#"
            <subsonic-response status="ok" version="1.1.1"> </subsonic-response>
        "#;

        let value = crate::from_xml(xml).unwrap();
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

        let value = crate::from_xml(xml).unwrap();
        let expected = Response::ok(
            Version::V1_13_0,
            ResponseBody::License(License {
                valid: true,
                email: Some("foo@bar.com".into()),
                license_expires: Some("2019-09-03T14:46:43".parse().unwrap()),
                ..Default::default()
            }),
        );
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

        let value = crate::from_xml(xml).unwrap();
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

        let value = crate::from_xml(xml).unwrap();
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
                        duration: Some(146),
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
                        duration: Some(208),
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

        let value = crate::from_xml(xml).unwrap();
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

        let value = crate::from_xml(xml).unwrap();
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
                        duration: Some(146),
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
                        duration: Some(208),
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

        let value = crate::from_xml(xml).unwrap();
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

        let value = crate::from_xml(xml).unwrap();
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
                    // <album id="11050" name="Flick Of The Switch" coverArt="al-11050" songCount="10" created="2004-11-27T19:22:51" duration="2222" artist="AC/DC" artistId="5432"/>
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
                    // <album id="11051" name="Fly On The Wall" coverArt="al-11051" songCount="10" created="2004-11-27T19:22:57" duration="2405" artist="AC/DC" artistId="5432"/>
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
                    // <album id="11052" name="For Those About To Rock" coverArt="al-11052" songCount="10" created="2004-11-08T23:35:02" duration="2403" artist="AC/DC" artistId="5432"/>
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
                    // <album id="11053" name="High Voltage" coverArt="al-11053" songCount="8" created="2004-11-27T20:23:32" duration="2414" artist="AC/DC" artistId="5432"/>
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
                    // <album id="10489" name="Highway To Hell" coverArt="al-10489" songCount="12" created="2009-06-15T09:41:54" duration="2745" artist="AC/DC" artistId="5432"/>
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
                    // <album id="11054" name="If You Want Blood..." coverArt="al-11054" songCount="1" created="2004-11-27T20:23:32" duration="304" artist="AC/DC" artistId="5432"/>
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
        let value = crate::from_xml(xml).unwrap();

        eprintln!("{:#?}", expected);
        eprintln!("{:#?}", value);

        assert_eq!(expected, value);
    }
}
