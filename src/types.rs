use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::SubsonicType;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Ok,
    Failed,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct Response {
    #[subsonic(xml(rename = "@status"))]
    pub status: ResponseStatus,
    #[subsonic(xml(rename = "@version"))]
    pub version: Version,
    #[subsonic_field]
    #[subsonic(common(flatten))]
    pub body: ResponseBody,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub enum ResponseBody {
    MusicFolders(#[subsonic_field] MusicFolders),
    License(#[subsonic_field] License),
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct License {
    #[subsonic(xml(rename = "@valid"))]
    pub valid: bool,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@email")
    )]
    pub email: Option<String>,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@licenseExpires")
    )]
    pub licence_expires: Option<DateTime<Utc>>,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@trialExpires")
    )]
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
#[subsonic(common(rename_all = "camelCase"))]
pub struct MusicFolders {
    #[subsonic_field]
    pub music_folder: Vec<MusicFolder>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct MusicFolder {
    #[subsonic(xml(rename = "@id"))]
    pub id: u32,
    #[subsonic(common(skip_serializing_if = "Option::is_none"), xml(rename = "@name"))]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct Indexes {
    #[subsonic_field]
    pub shortcut: Vec<Artist>,
    #[subsonic_field]
    pub index: Vec<Index>,
    #[subsonic_field]
    pub child: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct Index {
    #[subsonic(xml(rename = "@name"))]
    pub name: String,
    #[subsonic_field]
    pub artist: Vec<Artist>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct Artist {
    #[subsonic(xml(rename = "@id"))]
    pub id: u32,
    #[subsonic(xml(rename = "@name"))]
    pub name: String,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@artistImageUrl")
    )]
    pub artist_image_url: Option<String>,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@starred")
    )]
    pub starred: Option<DateTime<Utc>>,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@userRating")
    )]
    pub user_rating: Option<UserRating>,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@averageRating")
    )]
    pub averageRating: Option<AverageRating>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct Genres {
    #[subsonic_field]
    pub genre: Vec<Genre>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct Genre {
    #[subsonic(xml(rename = "@songCount"))]
    pub song_count: u32,
    #[subsonic(xml(rename = "@albumCount"))]
    pub album_count: u32,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct ArtistsID3 {
    #[subsonic_field]
    pub index: Vec<IndexID3>,
    #[subsonic(xml(rename = "@ignoredArticles"))]
    pub ignored_articles: String,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct IndexID3 {
    #[subsonic(xml(rename = "@name"))]
    pub name: String,
    #[subsonic_field]
    pub artist: Vec<ArtistID3>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct ArtistID3 {
    #[subsonic(xml(rename = "@id"))]
    pub id: u32,
    #[subsonic(xml(rename = "@name"))]
    pub name: String,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@coverArt")
    )]
    pub cover_art: Option<String>,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@artistImageUrl")
    )]
    pub artist_image_url: Option<String>,
    #[subsonic(xml(rename = "@albumCount"))]
    pub album_count: u32,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@starred")
    )]
    pub starred: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct ArtistWithAlbumsID3 {
    #[subsonic(common(flatten))]
    #[subsonic_field]
    pub artist: ArtistID3,
    #[subsonic_field]
    pub album: Vec<AlbumID3>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct AlbumID3 {
    #[subsonic(xml(rename = "@id"))]
    pub id: u32,
    #[subsonic(xml(rename = "@name"))]
    pub name: String,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@artist")
    )]
    pub artist: Option<String>,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@artistId")
    )]
    pub artist_id: Option<String>,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@coverArt")
    )]
    pub cover_art: Option<String>,
    #[subsonic(xml(rename = "@songCount"))]
    pub song_count: u32,
    #[subsonic(xml(rename = "@duration"))]
    pub duration: u32,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@playCount")
    )]
    pub play_count: Option<u64>,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@created")
    )]
    pub created: Option<DateTime<Utc>>,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@starred")
    )]
    pub starred: Option<DateTime<Utc>>,
    #[subsonic(common(skip_serializing_if = "Option::is_none"), xml(rename = "@year"))]
    pub year: Option<u32>,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@genre")
    )]
    pub genre: Option<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct AlbumWithSongsID3 {
    #[subsonic(common(flatten))]
    #[subsonic_field]
    pub album: AlbumID3,
    #[subsonic_field]
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Videos {
    #[subsonic_field]
    pub video: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct VideoInfo {
    #[subsonic(xml(rename = "@id"))]
    pub id: String,
    #[subsonic_field]
    pub captions: Vec<Captions>,
    #[subsonic(xml(rename = "audioTrack"))]
    #[subsonic_field]
    pub audio_track: Vec<AudioTrack>,
    #[subsonic_field]
    pub conversion: Vec<VideoConversion>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Captions {
    #[subsonic(xml(rename = "@id"))]
    pub id: String,
    #[subsonic(xml(rename = "@name"))]
    pub format: Option<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct AudioTrack {
    #[subsonic(xml(rename = "@id"))]
    pub id: String,
    #[subsonic(xml(rename = "@name"))]
    pub name: Option<String>,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@languageCode")
    )]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct VideoConversion {
    #[subsonic(xml(rename = "@id"))]
    pub id: String,
    #[subsonic(xml(rename = "@name"))]
    pub bit_rate: Option<u32>,
    #[subsonic(xml(rename = "@audioTrackId"))]
    pub audio_track_id: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Directory {
    #[subsonic(xml(rename = "@id"))]
    pub id: String,
    #[subsonic(xml(rename = "@parent"))]
    pub parent: Option<String>,
    #[subsonic(xml(rename = "@name"))]
    pub name: String,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@starred")
    )]
    pub starred: Option<DateTime<Utc>>,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@userRating")
    )]
    pub user_rating: Option<UserRating>,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@averageRating")
    )]
    pub average_rating: Option<AverageRating>,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none"),
        xml(rename = "@playCount")
    )]
    pub play_count: Option<u64>,
    #[subsonic_field]
    pub child: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Child {
    #[subsonic(xml(rename = "@id"))]
    pub id: String,
    #[subsonic(xml(rename = "@parent"))]
    pub parent: Option<String>,
    #[subsonic(xml(rename = "@isDir"))]
    pub is_dir: bool,
    #[subsonic(xml(rename = "@title"))]
    pub title: String,
    #[subsonic(xml(rename = "@album"))]
    pub album: Option<String>,
    #[subsonic(xml(rename = "@artist"))]
    pub artist: Option<String>,
    #[subsonic(xml(rename = "@track"))]
    pub track: Option<u32>,
    #[subsonic(xml(rename = "@year"))]
    pub year: Option<u32>,
    #[subsonic(xml(rename = "@genre"))]
    pub genre: Option<String>,
    #[subsonic(xml(rename = "@coverArt"))]
    pub cover_art: Option<String>,
    #[subsonic(xml(rename = "@size"))]
    pub size: Option<u64>,
    #[subsonic(xml(rename = "@contentType"))]
    pub content_type: Option<String>,
    #[subsonic(xml(rename = "@suffix"))]
    pub suffix: Option<String>,
    #[subsonic(xml(rename = "@transcodedContentType"))]
    pub transcoded_content_type: Option<String>,
    #[subsonic(xml(rename = "@transcodedSuffix"))]
    pub transcoded_suffix: Option<String>,
    #[subsonic(xml(rename = "@duration"))]
    pub duration: Option<u32>,
    #[subsonic(xml(rename = "@bitRate"))]
    pub bit_rate: Option<u32>,
    #[subsonic(xml(rename = "@path"))]
    pub path: Option<String>,
    #[subsonic(xml(rename = "@isVideo"))]
    pub is_video: Option<bool>,
    #[subsonic(xml(rename = "@userRating"))]
    pub user_rating: Option<UserRating>,
    #[subsonic(xml(rename = "@averageRating"))]
    pub average_rating: Option<AverageRating>,
    #[subsonic(xml(rename = "@playCount"))]
    pub play_count: Option<u64>,
    #[subsonic(xml(rename = "@discNumber"))]
    pub disc_number: Option<u32>,
    #[subsonic(xml(rename = "@created"))]
    pub created: Option<DateTime<Utc>>,
    #[subsonic(xml(rename = "@starred"))]
    pub starred: Option<DateTime<Utc>>,
    #[subsonic(xml(rename = "@albumId"))]
    pub album_id: Option<String>,
    #[subsonic(xml(rename = "@artistId"))]
    pub artist_id: Option<String>,
    #[subsonic(xml(rename = "@type"))]
    pub media_type: Option<MediaType>,
    #[subsonic(xml(rename = "@bookmarkPosition"))]
    pub bookmark_position: Option<u64>,
    #[subsonic(xml(rename = "@originalWidth"))]
    pub original_width: Option<u32>,
    #[subsonic(xml(rename = "@originalHeight"))]
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

#[derive(Debug)]
pub struct InvalidUserRating;

impl std::fmt::Display for InvalidUserRating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid user rating")
    }
}

impl std::error::Error for InvalidUserRating {}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct UserRating(u32);

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
    #[subsonic_field]
    pub entry: Vec<NowPlayingEntry>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct NowPlayingEntry {
    #[subsonic(common(flatten))]
    #[subsonic_field]
    pub child: Child,
    #[subsonic(xml(rename = "@username"))]
    pub username: String,
    #[subsonic(xml(rename = "@minutesAgo"))]
    pub minutes_ago: u32,
    #[subsonic(xml(rename = "@playerId"))]
    pub player_id: u32,
    #[subsonic(
        common(skip_serializing_if = "Option::is_none",),
        xml(rename = "@playerName")
    )]
    pub player_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct SearchResult {
    #[subsonic(xml(rename = "@offset"))]
    pub offset: u32,
    #[subsonic(xml(rename = "@totalHits"))]
    pub total_hits: u32,
    #[subsonic(xml(rename = "match"))]
    #[subsonic_field]
    pub matches: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct SearchResult2 {
    #[subsonic_field]
    pub artist: Vec<Artist>,
    #[subsonic_field]
    pub album: Vec<Child>,
    #[subsonic_field]
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct SearchResult3 {
    #[subsonic_field]
    pub artist: Vec<ArtistID3>,
    #[subsonic_field]
    pub album: Vec<AlbumID3>,
    #[subsonic_field]
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Playlists {
    #[subsonic_field]
    pub playlists: Vec<Playlist>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct Playlist {
    #[subsonic(xml(rename = "@id"))]
    pub id: String,
    #[subsonic(xml(rename = "@name"))]
    pub name: String,
    #[subsonic(xml(rename = "@comment"))]
    pub comment: Option<String>,
    #[subsonic(xml(rename = "@owner"))]
    pub owner: Option<String>,
    #[subsonic(xml(rename = "@public"))]
    pub public: Option<bool>,
    #[subsonic(xml(rename = "@songCount"))]
    pub song_count: u32,
    #[subsonic(xml(rename = "@duration"))]
    pub duration: u32,
    #[subsonic(xml(rename = "@created"))]
    pub created: DateTime<Utc>,
    #[subsonic(xml(rename = "@changed"))]
    pub changed: DateTime<Utc>,
    #[subsonic(xml(rename = "@coverArt"))]
    pub cover_art: Option<String>,
    pub allowed_user: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct PlaylistWithSongs {
    #[subsonic(common(flatten))]
    #[subsonic_field]
    pub playlist: Playlist,
    #[subsonic_field]
    pub entry: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct JukeboxStatus {
    #[subsonic(xml(rename = "@currentIndex"))]
    pub current_index: u32,
    #[subsonic(xml(rename = "@playing"))]
    pub playing: bool,
    #[subsonic(xml(rename = "@gain"))]
    pub gain: f32,
    #[subsonic(xml(rename = "@position"))]
    pub position: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct JukeboxPlaylist {
    #[subsonic(common(flatten))]
    #[subsonic_field]
    pub status: JukeboxStatus,
    #[subsonic_field]
    pub entry: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct ChatMessages {
    #[subsonic_field]
    pub chat_message: Vec<ChatMessage>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct ChatMessage {
    #[subsonic(xml(rename = "@username"))]
    pub username: String,
    #[subsonic(xml(rename = "@time"))]
    pub time: DateTime<Utc>,
    #[subsonic(xml(rename = "@message"))]
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct AlbumList {
    #[subsonic_field]
    pub album: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct AlbumList2 {
    #[subsonic_field]
    pub album: Vec<AlbumID3>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Songs {
    #[subsonic_field]
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Lyrics {
    #[subsonic(xml(rename = "@artist"))]
    pub artist: Option<String>,
    #[subsonic(xml(rename = "@title"))]
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Podcasts {
    #[subsonic_field]
    pub channel: Vec<PodcastChannel>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct PodcastChannel {
    #[subsonic(xml(rename = "@id"))]
    pub id: String,
    #[subsonic(xml(rename = "@url"))]
    pub url: String,
    #[subsonic(xml(rename = "@title"))]
    pub title: Option<String>,
    #[subsonic(xml(rename = "@description"))]
    pub description: Option<String>,
    #[subsonic(xml(rename = "@coverArt"))]
    pub cover_art: Option<String>,
    #[subsonic(xml(rename = "@originalImageUrl"))]
    pub original_image_url: Option<String>,
    #[subsonic(xml(rename = "@status"))]
    pub status: PodcastStatus,
    #[subsonic(xml(rename = "@errorMessage"))]
    pub error_message: Option<String>,
    #[subsonic_field]
    pub episode: Vec<PodcastEpisode>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct NewestPodcasts {
    #[subsonic_field]
    pub episode: Vec<PodcastEpisode>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct PodcastEpisode {
    #[subsonic(common(flatten))]
    #[subsonic_field]
    pub child: Child,
    #[subsonic(xml(rename = "@streamId"))]
    pub stream_id: Option<String>,
    #[subsonic(xml(rename = "@channelId"))]
    pub channel_id: String,
    #[subsonic(xml(rename = "@description"))]
    pub description: Option<String>,
    #[subsonic(xml(rename = "@status"))]
    pub status: PodcastStatus,
    #[subsonic(xml(rename = "@publishDate"))]
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

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct InternetRadioStations {
    #[subsonic_field]
    pub internet_radio_station: Vec<InternetRadioStation>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct InternetRadioStation {
    #[subsonic(xml(rename = "@id"))]
    pub id: String,
    #[subsonic(xml(rename = "@name"))]
    pub name: String,
    #[subsonic(xml(rename = "@streamUrl"))]
    pub stream_url: String,
    #[subsonic(xml(rename = "@homePageUrl"))]
    pub home_page_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Bookmarks {
    #[subsonic_field]
    pub bookmark: Vec<Bookmark>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Bookmark {
    #[subsonic(xml(rename = "@position"))]
    pub position: u64,
    #[subsonic(xml(rename = "@username"))]
    pub username: String,
    #[subsonic(xml(rename = "@comment"))]
    pub comment: Option<String>,
    #[subsonic(xml(rename = "@created"))]
    pub created: DateTime<Utc>,
    #[subsonic(xml(rename = "@changed"))]
    pub changed: DateTime<Utc>,
    #[subsonic_field]
    pub entry: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct PlayQueue {
    #[subsonic(xml(rename = "@current"))]
    pub current: Option<u64>,
    #[subsonic(xml(rename = "@position"))]
    pub position: Option<u64>,
    #[subsonic(xml(rename = "@username"))]
    pub username: String,
    #[subsonic(xml(rename = "@changed"))]
    pub changed: DateTime<Utc>,
    /// Name of client app
    #[subsonic(xml(rename = "@changedBy"))]
    pub changed_by: String,
    #[subsonic_field]
    pub entry: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Shares {
    #[subsonic_field]
    pub share: Vec<Share>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Share {
    #[subsonic(xml(rename = "@id"))]
    pub id: String,
    #[subsonic(xml(rename = "@url"))]
    pub url: String,
    #[subsonic(xml(rename = "@description"))]
    pub description: Option<String>,
    #[subsonic(xml(rename = "@username"))]
    pub username: String,
    #[subsonic(xml(rename = "@created"))]
    pub created: DateTime<Utc>,
    #[subsonic(xml(rename = "@expires"))]
    pub expires: Option<DateTime<Utc>>,
    #[subsonic(xml(rename = "@lastVisited"))]
    pub last_visited: Option<DateTime<Utc>>,
    #[subsonic(xml(rename = "@visitCount"))]
    pub visit_count: u64,
    #[subsonic_field]
    pub entry: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Starred {
    #[subsonic_field]
    pub song: Vec<Child>,
    #[subsonic_field]
    pub album: Vec<Child>,
    #[subsonic_field]
    pub artist: Vec<Artist>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct AlbumInfo {
    pub notes: Vec<String>,
    #[subsonic(xml(rename = "musicBrainzId"))]
    pub music_brainz_id: Vec<String>,
    #[subsonic(xml(rename = "lastFmUrl"))]
    pub last_fm_url: Vec<String>,
    #[subsonic(xml(rename = "smallImageUrl"))]
    pub small_image_url: Vec<String>,
    #[subsonic(xml(rename = "mediumImageUrl"))]
    pub medium_image_url: Vec<String>,
    #[subsonic(xml(rename = "largeImageUrl"))]
    pub large_image_url: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct ArtistInfoBase {
    #[subsonic(xml(rename = "biography"))]
    pub biography: Vec<String>,
    #[subsonic(xml(rename = "musicBrainzId"))]
    pub music_brainz_id: Vec<String>,
    #[subsonic(xml(rename = "lastFmUrl"))]
    pub last_fm_url: Vec<String>,
    #[subsonic(xml(rename = "smallImageUrl"))]
    pub small_image_url: Vec<String>,
    #[subsonic(xml(rename = "mediumImageUrl"))]
    pub medium_image_url: Vec<String>,
    #[subsonic(xml(rename = "largeImageUrl"))]
    pub large_image_url: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct ArtistInfo {
    #[subsonic_field]
    #[subsonic(common(flatten))]
    pub info: ArtistInfoBase,
    #[subsonic_field]
    pub similar_artist: Vec<Artist>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct ArtistInfo2 {
    #[subsonic_field]
    #[subsonic(common(flatten))]
    pub info: ArtistInfoBase,
    #[subsonic_field]
    pub similar_artist: Vec<ArtistID3>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct SimilarSongs {
    #[subsonic_field]
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct SimilarSongs2 {
    #[subsonic_field]
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct TopSongs {
    #[subsonic_field]
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Starred2 {
    #[subsonic_field]
    pub song: Vec<Child>,
    #[subsonic_field]
    pub album: Vec<AlbumID3>,
    #[subsonic_field]
    pub artist: Vec<ArtistID3>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct ScanStatus {
    #[subsonic(xml(rename = "@scanning"))]
    pub scanning: bool,
    #[subsonic(xml(rename = "@count"))]
    pub count: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Users {
    #[subsonic_field]
    pub user: Vec<User>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct User {
    #[subsonic(xml(rename = "@username"))]
    pub username: String,
    #[subsonic(xml(rename = "@email"))]
    pub email: Option<String>,
    #[subsonic(xml(rename = "@scrobblingEnabled"))]
    pub scrobbling_enabled: bool,
    #[subsonic(xml(rename = "@maxBitRate"))]
    pub max_bit_rate: Option<u64>,
    #[subsonic(xml(rename = "@adminRole"))]
    pub admin_role: bool,
    #[subsonic(xml(rename = "@settingsRole"))]
    pub settings_role: bool,
    #[subsonic(xml(rename = "@downloadRole"))]
    pub download_role: bool,
    #[subsonic(xml(rename = "@uploadRole"))]
    pub upload_role: bool,
    #[subsonic(xml(rename = "@playlistRole"))]
    pub playlist_role: bool,
    #[subsonic(xml(rename = "@coverArtRole"))]
    pub cover_art_role: bool,
    #[subsonic(xml(rename = "@commentRole"))]
    pub comment_role: bool,
    #[subsonic(xml(rename = "@podcastRole"))]
    pub podcast_role: bool,
    #[subsonic(xml(rename = "@streamRole"))]
    pub stream_role: bool,
    #[subsonic(xml(rename = "@jukeboxRole"))]
    pub jukebox_role: bool,
    #[subsonic(xml(rename = "@shareRole"))]
    pub share_role: bool,
    #[subsonic(xml(rename = "@videoConversionRole"))]
    pub video_conversion_role: bool,
    #[subsonic(xml(rename = "@avatarLastChanged"))]
    pub avatar_last_changed: Option<DateTime<Utc>>,
    pub folder: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Error {
    #[subsonic(xml(rename = "@code"))]
    pub code: u32,
    #[subsonic(xml(rename = "@message"))]
    pub message: Option<String>,
}
