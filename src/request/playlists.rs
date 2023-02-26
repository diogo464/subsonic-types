use serde::{Deserialize, Serialize};
use subsonic_macro::{FromQuery, SubsonicRequest, ToQuery};

/// Returns all playlists a user is allowed to play.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getPlaylists>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.0.0", path = "getPlaylists")]
pub struct GetPlaylists {
    /// Since 1.8.0
    /// If specified, return playlists for this user rather than for the authenticated user. The authenticated user must have admin role if this parameter is used.
    pub username: Option<String>,
}

/// Returns a listing of files in a saved playlist.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getPlaylist>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.0.0", path = "getPlaylist")]
pub struct GetPlaylist {
    /// ID of the playlist to return, as obtained by [`GetPlaylists`].
    pub id: String,
}

/// Creates (or updates) a playlist.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#createPlaylist>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.2.0", path = "createPlaylist")]
pub struct CreatePlaylist {
    /// The playlist ID.
    /// Required if updating.
    pub paylist_id: Option<String>,
    /// The human-readable name of the playlist.
    /// Required if creating.
    pub name: Option<String>,
    /// The list of song IDs to include in the playlist.
    #[serde(default)]
    pub song_id: Vec<String>,
}

/// Updates a playlist. Only the owner of a playlist is allowed to update it.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#updatePlaylist>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.8.0", path = "updatePlaylist")]
pub struct UpdatePlaylist {
    /// The playlist ID.
    pub playlist_id: String,
    /// The human-readable name of the playlist.
    pub name: Option<String>,
    /// The playlist comment.
    pub comment: Option<String>,
    /// Whether this playlist is visible to all users.
    pub public: Option<bool>,
    /// Add this song with this ID to the playlist.
    #[serde(default)]
    pub song_id_to_add: Vec<String>,
    /// Remove the song at this position in the playlist.
    #[serde(default)]
    pub song_index_to_remove: Vec<u32>,
}

/// Deletes a saved playlist.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#deletePlaylist>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.2.0", path = "deletePlaylist")]
pub struct DeletePlaylist {
    /// ID of the playlist to delete, as obtained by [`GetPlaylists`].
    pub id: String,
}
