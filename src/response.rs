use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::SubsonicType;

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
    #[subsonic(flatten)]
    pub body: ResponseBody,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub enum ResponseBody {
    MusicFolders(MusicFolders),
    License(License),
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct License {
    #[subsonic(attribute)]
    pub valid: bool,
    #[subsonic(attribute, optional)]
    pub email: Option<String>,
    #[subsonic(attribute, optional)]
    pub licence_expires: Option<DateTime<Utc>>,
    #[subsonic(attribute, optional)]
    pub trial_expires: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct InvalidVersion;

impl std::fmt::Display for InvalidVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid version")
    }
}

impl std::error::Error for InvalidVersion {}

#[derive(Debug, Default, Copy, Clone, PartialEq, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}
impl_subsonic_for_serde!(Version);

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl std::str::FromStr for Version {
    type Err = InvalidVersion;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let major = parts
            .next()
            .ok_or(InvalidVersion)?
            .parse()
            .map_err(|_| InvalidVersion)?;
        let minor = parts
            .next()
            .ok_or(InvalidVersion)?
            .parse()
            .map_err(|_| InvalidVersion)?;
        let patch = parts
            .next()
            .ok_or(InvalidVersion)?
            .parse()
            .map_err(|_| InvalidVersion)?;
        Ok(Self::new(major, minor, patch))
    }
}

impl<N1, N2, N3> From<(N1, N2, N3)> for Version
where
    N1: Into<u64>,
    N2: Into<u64>,
    N3: Into<u64>,
{
    fn from(value: (N1, N2, N3)) -> Self {
        Self::new(value.0.into(), value.1.into(), value.2.into())
    }
}

impl serde::Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct MusicFolders {
    pub music_folder: Vec<MusicFolder>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct MusicFolder {
    #[subsonic(attribute)]
    pub id: u32,
    #[subsonic(attribute, optional)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Indexes {
    pub shortcut: Vec<Artist>,
    pub index: Vec<Index>,
    pub child: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Index {
    #[subsonic(attribute)]
    pub name: String,
    pub artist: Vec<Artist>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Artist {
    #[subsonic(attribute)]
    pub id: u32,
    #[subsonic(attribute)]
    pub name: String,
    #[subsonic(attribute, optional)]
    pub artist_image_url: Option<String>,
    #[subsonic(attribute, optional)]
    pub starred: Option<DateTime<Utc>>,
    #[subsonic(attribute, optional)]
    pub user_rating: Option<UserRating>,
    #[subsonic(attribute, optional)]
    pub averageRating: Option<AverageRating>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Genres {
    pub genre: Vec<Genre>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Genre {
    #[subsonic(attribute)]
    pub song_count: u32,
    #[subsonic(attribute)]
    pub album_count: u32,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct ArtistsID3 {
    pub index: Vec<IndexID3>,
    #[subsonic(attribute)]
    pub ignored_articles: String,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct IndexID3 {
    #[subsonic(attribute)]
    pub name: String,
    pub artist: Vec<ArtistID3>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct ArtistID3 {
    #[subsonic(attribute)]
    pub id: u32,
    #[subsonic(attribute)]
    pub name: String,
    #[subsonic(attribute, optional)]
    pub cover_art: Option<String>,
    #[subsonic(attribute, optional)]
    pub artist_image_url: Option<String>,
    #[subsonic(attribute)]
    pub album_count: u32,
    #[subsonic(attribute, optional)]
    pub starred: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct ArtistWithAlbumsID3 {
    #[subsonic(flatten)]
    pub artist: ArtistID3,
    pub album: Vec<AlbumID3>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct AlbumID3 {
    #[subsonic(attribute)]
    pub id: u32,
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
    pub created: Option<DateTime<Utc>>,
    #[subsonic(attribute, optional)]
    pub starred: Option<DateTime<Utc>>,
    #[subsonic(attribute, optional)]
    pub year: Option<u32>,
    #[subsonic(attribute, optional)]
    pub genre: Option<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct AlbumWithSongsID3 {
    #[subsonic(flatten)]
    pub album: AlbumID3,
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Videos {
    pub video: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct VideoInfo {
    #[subsonic(attribute)]
    pub id: String,
    pub captions: Vec<Captions>,
    pub audio_track: Vec<AudioTrack>,
    pub conversion: Vec<VideoConversion>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Captions {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute, optional)]
    pub format: Option<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct AudioTrack {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute, optional)]
    pub name: Option<String>,
    #[subsonic(attribute, optional)]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct VideoConversion {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute, optional)]
    pub bit_rate: Option<u32>,
    #[subsonic(attribute, optional)]
    pub audio_track_id: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Directory {
    #[subsonic(attribute)]
    pub id: String,
    #[subsonic(attribute, optional)]
    pub parent: Option<String>,
    #[subsonic(attribute)]
    pub name: String,
    #[subsonic(attribute, optional)]
    pub starred: Option<DateTime<Utc>>,
    #[subsonic(attribute, optional)]
    pub user_rating: Option<UserRating>,
    #[subsonic(attribute, optional)]
    pub average_rating: Option<AverageRating>,
    #[subsonic(attribute, optional)]
    pub play_count: Option<u64>,
    pub child: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
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
    pub created: Option<DateTime<Utc>>,
    #[subsonic(attribute, optional)]
    pub starred: Option<DateTime<Utc>>,
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MediaType {
    Music,
    Podcast,
    AudioBook,
    Video,
}
impl_subsonic_for_serde!(MediaType);

#[derive(Debug)]
pub struct InvalidUserRating;

impl std::fmt::Display for InvalidUserRating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid user rating")
    }
}

impl std::error::Error for InvalidUserRating {}

// TODO: Move this to common
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct UserRating(u32);
impl_subsonic_for_serde!(UserRating);

impl UserRating {
    pub fn new(value: u32) -> Result<Self, InvalidUserRating> {
        if value > 5 || value < 1 {
            Err(InvalidUserRating)
        } else {
            Ok(UserRating(value))
        }
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for UserRating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UserRating {
    type Err = InvalidUserRating;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse().map_err(|_| InvalidUserRating)?;
        UserRating::new(value)
    }
}

impl From<UserRating> for u32 {
    fn from(value: UserRating) -> Self {
        value.0
    }
}

impl Serialize for UserRating {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.0)
    }
}

impl<'de> Deserialize<'de> for UserRating {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        UserRating::new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug)]
pub struct InvalidAverageRating;

impl std::fmt::Display for InvalidAverageRating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid average rating")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AverageRating(f32);
impl_subsonic_for_serde!(AverageRating);

impl AverageRating {
    pub fn new(value: f32) -> Result<Self, InvalidAverageRating> {
        if value > 5.0 || value < 1.0 {
            Err(InvalidAverageRating)
        } else {
            Ok(AverageRating(value))
        }
    }

    pub fn value(self) -> f32 {
        self.0
    }
}

impl std::fmt::Display for AverageRating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for AverageRating {
    type Err = InvalidAverageRating;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse().map_err(|_| InvalidAverageRating)?;
        AverageRating::new(value)
    }
}

impl From<AverageRating> for f32 {
    fn from(value: AverageRating) -> Self {
        value.0
    }
}

impl Serialize for AverageRating {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f32(self.0)
    }
}

impl<'de> Deserialize<'de> for AverageRating {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = f32::deserialize(deserializer)?;
        AverageRating::new(value).map_err(serde::de::Error::custom)
    }
}

impl std::hash::Hash for AverageRating {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

impl std::cmp::Eq for AverageRating {}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct NowPlaying {
    pub entry: Vec<NowPlayingEntry>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
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

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct SearchResult {
    #[subsonic(attribute)]
    pub offset: u32,
    #[subsonic(attribute)]
    pub total_hits: u32,
    #[subsonic(rename = "match")]
    pub matches: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct SearchResult2 {
    pub artist: Vec<Artist>,
    pub album: Vec<Child>,
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct SearchResult3 {
    pub artist: Vec<ArtistID3>,
    pub album: Vec<AlbumID3>,
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Playlists {
    pub playlists: Vec<Playlist>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
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
    pub created: DateTime<Utc>,
    #[subsonic(attribute)]
    pub changed: DateTime<Utc>,
    #[subsonic(attribute, optional)]
    pub cover_art: Option<String>,
    pub allowed_user: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct PlaylistWithSongs {
    #[subsonic(flatten)]
    pub playlist: Playlist,
    pub entry: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
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

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct JukeboxPlaylist {
    #[subsonic(flatten)]
    pub status: JukeboxStatus,
    pub entry: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct ChatMessages {
    pub chat_message: Vec<ChatMessage>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct ChatMessage {
    #[subsonic(attribute)]
    pub username: String,
    #[subsonic(attribute)]
    pub time: DateTime<Utc>,
    #[subsonic(attribute)]
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct AlbumList {
    pub album: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct AlbumList2 {
    pub album: Vec<AlbumID3>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Songs {
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Lyrics {
    #[subsonic(attribute)]
    pub artist: Option<String>,
    #[subsonic(attribute)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Podcasts {
    pub channel: Vec<PodcastChannel>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
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

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct NewestPodcasts {
    pub episode: Vec<PodcastEpisode>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
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
    pub publish_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PodcastStatus {
    New,
    Downloading,
    Completed,
    Error,
    Skipped,
}
impl_subsonic_for_serde!(PodcastStatus);

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct InternetRadioStations {
    pub internet_radio_station: Vec<InternetRadioStation>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
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

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Bookmark {
    #[subsonic(attribute)]
    pub position: u64,
    #[subsonic(attribute)]
    pub username: String,
    #[subsonic(attribute, optional)]
    pub comment: Option<String>,
    #[subsonic(attribute)]
    pub created: DateTime<Utc>,
    #[subsonic(attribute)]
    pub changed: DateTime<Utc>,
    pub entry: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct PlayQueue {
    #[subsonic(attribute)]
    pub current: Option<u64>,
    #[subsonic(attribute, optional)]
    pub position: Option<u64>,
    #[subsonic(attribute)]
    pub username: String,
    #[subsonic(attribute)]
    pub changed: DateTime<Utc>,
    /// Name of client app
    #[subsonic(attribute)]
    pub changed_by: String,
    pub entry: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Shares {
    pub share: Vec<Share>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
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
    pub created: DateTime<Utc>,
    #[subsonic(attribute, optional)]
    pub expires: Option<DateTime<Utc>>,
    #[subsonic(attribute, optional)]
    pub last_visited: Option<DateTime<Utc>>,
    #[subsonic(attribute)]
    pub visit_count: u64,
    pub entry: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Starred {
    pub song: Vec<Child>,
    pub album: Vec<Child>,
    pub artist: Vec<Artist>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct AlbumInfo {
    pub notes: Vec<String>,
    pub music_brainz_id: Vec<String>,
    pub last_fm_url: Vec<String>,
    pub small_image_url: Vec<String>,
    pub medium_image_url: Vec<String>,
    pub large_image_url: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct ArtistInfoBase {
    pub biography: Vec<String>,
    pub music_brainz_id: Vec<String>,
    pub last_fm_url: Vec<String>,
    pub small_image_url: Vec<String>,
    pub medium_image_url: Vec<String>,
    pub large_image_url: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
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

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct SimilarSongs {
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct SimilarSongs2 {
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct TopSongs {
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Starred2 {
    pub song: Vec<Child>,
    pub album: Vec<AlbumID3>,
    pub artist: Vec<ArtistID3>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
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

#[derive(Debug, Clone, PartialEq, SubsonicType)]
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
    pub avatar_last_changed: Option<DateTime<Utc>>,
    pub folder: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Error {
    #[subsonic(attribute)]
    pub code: u32,
    #[subsonic(attribute)]
    pub message: Option<String>,
}
