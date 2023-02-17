use serde::{Deserialize, Serialize};

use crate::{
    common::{AverageRating, DateTime, MediaType, UserRating, Version},
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
    #[subsonic(flatten, complex)]
    pub body: ResponseBody,
}

impl Response {
    pub fn ok(version: Version, body: ResponseBody) -> Self {
        Self {
            status: ResponseStatus::Ok,
            version,
            body,
        }
    }

    pub fn failed(version: Version, error: Error) -> Self {
        Self {
            status: ResponseStatus::Failed,
            version,
            body: ResponseBody::Error(error),
        }
    }
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub enum ResponseBody {
    MusicFolders(MusicFolders),
    Indexes(Indexes),
    Directory(Directory),
    Genres(Genres),
    Artists(ArtistsID3),
    Artist(Artist),
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
    pub licence_expires: Option<DateTime>,
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
    pub id: u32,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PodcastStatus {
    New,
    Downloading,
    Completed,
    Error,
    Skipped,
}
impl_subsonic_for_serde!(PodcastStatus);

impl Default for PodcastStatus {
    fn default() -> Self {
        PodcastStatus::Error
    }
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
    pub code: u32,
    #[subsonic(attribute)]
    pub message: Option<String>,
}

impl Error {
    pub fn new(code: u32) -> Self {
        Error {
            code,
            message: None,
        }
    }

    pub fn with_message(code: u32, message: impl Into<String>) -> Self {
        Error {
            code,
            message: Some(message.into()),
        }
    }
}
