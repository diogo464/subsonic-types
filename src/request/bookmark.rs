use serde::{Deserialize, Serialize};
use subsonic_macro::SubsonicRequest;

use crate::common::Milliseconds;

/// Returns all bookmarks for this user. A bookmark is a position within a certain media file.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getBookmarks>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.9.0", path = "getBookmarks")]
pub struct GetBookmarks;

/// Creates or updates a bookmark (a position within a media file). Bookmarks are personal and not visible to other users.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#createBookmark>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.9.0", path = "createBookmark")]
pub struct CreateBookmark {
    /// ID of the media file to bookmark.
    /// If a bookmark already exists for this file it will be overwritten.
    pub id: String,
    /// The position within the media file.
    pub position: Milliseconds,
    /// A user-defined comment.
    pub comment: Option<String>,
}

/// Deletes the bookmark for a given file.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#deleteBookmark>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.9.0", path = "deleteBookmark")]
pub struct DeleteBookmark {
    /// ID of the media file for which to delete the bookmark.
    /// Other users' bookmarks are not affected.
    pub id: String,
}

/// Returns the state of the play queue for this user (as set by [`SavePlayQueue`]).
/// This includes the tracks in the play queue, the currently playing track, and the position within this track.
/// Typically used to allow a user to move between different clients/apps while retaining the same play queue (for instance when listening to an audio book).
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getPlayQueue>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.12.0", path = "getPlayQueue")]
pub struct GetPlayQueue;

///  Saves the state of the play queue for this user.
/// This includes the tracks in the play queue, the currently playing track, and the position within this track.
/// Typically used to allow a user to move between different clients/apps while retaining the same play queue (for instance when listening to an audio book).
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#savePlayQueue>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.12.0", path = "savePlayQueue")]
pub struct SavePlayQueue {
    /// ID of a song in the play queue.
    #[serde(default)]
    pub id: Vec<String>,
    /// The ID of the current playing song.
    pub current: Option<String>,
    /// The position in milliseconds within the currently playing song.
    pub position: Option<Milliseconds>,
}
