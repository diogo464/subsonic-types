use crate::common::{Milliseconds, MusicFolderId};

/// Returns all configured top-level music folders. Takes no extra parameters.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getMusicFolders>
#[derive(Debug, Clone, PartialEq)]
pub struct GetMusicFolders;

/// Returns an indexed structure of all artists.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getIndexes>
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub struct GetMusicDirectory {
    /// A string which uniquely identifies the music folder. Obtained by calls to getIndexes or getMusicDirectory.
    pub id: String,
}

/// Returns all genres.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getGenres>
#[derive(Debug, Clone, PartialEq)]
pub struct GetGenres;

/// Represents the parameters for the `getArtists` request.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getArtists>.
#[derive(Debug, Clone, PartialEq)]
pub struct GetArtists {
    /// If specified, only return artists in the music folder with the given ID.
    /// See [`GetMusicFolders`].
    pub music_folder_id: Option<MusicFolderId>,
}

/// Returns details for an artist, including a list of albums.
///
/// See: <http://www.subsonic.org/pages/api.jsp#getArtist>
#[derive(Debug, Clone, PartialEq)]
pub struct GetArtist {
    /// The artist ID.
    pub id: String,
}

/// Returns details for an album, including a list of songs.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getAlbum>
#[derive(Debug, Clone, PartialEq)]
pub struct GetAlbum {
    /// The album ID.
    pub id: String,
}

/// Returns details for a song.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getSong>
#[derive(Debug, Clone, PartialEq)]
pub struct GetSong {
    /// The ID of the song to retrieve.
    pub id: String,
}

/// Represents a request to retrieve all video files.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getVideos>
#[derive(Debug, Clone, PartialEq)]
pub struct GetVideos;

/// Returns details for a video, including information about available audio tracks, subtitles (captions) and conversions.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getVideoInfo>
#[derive(Debug, Clone, PartialEq)]
pub struct GetVideoInfo {
    pub id: String,
}

/// Returns artist info with biography, image URLs and similar artists, using data from <http://last.fm>.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getArtistInfo>
#[derive(Debug, Clone, PartialEq)]
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

/// Similar to [`GetAristInfo`], but organizes music according to ID3 tags.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getArtistInfo2>
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub struct GetAlbumInfo {
    /// The album or song ID.
    pub id: String,
}

/// Similar to [`GetAlbumInfo`], but organizes music according to ID3 tags.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getAlbumInfo2>
#[derive(Debug, Clone, PartialEq)]
pub struct GetAlbumInfo2 {
    /// The album or song ID.
    pub id: String,
}

/// Returns a random collection of songs from the given artist and similar artists, using data from last.fm. Typically used for artist radio features.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getSimilarSongs>
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
pub struct GetTopSongs {
    /// The artist name.
    pub id: String,
    /// Max number of songs to return.
    pub count: Option<u32>,
}

impl GetTopSongs {
    pub const DEFAULT_COUNT: u32 = 50;
}

