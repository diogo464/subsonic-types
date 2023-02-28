use serde::{Deserialize, Serialize};
use subsonic_types_macro::{FromQuery, SubsonicRequest, ToQuery};

use crate::common::Milliseconds;

/// Returns all bookmarks for this user. A bookmark is a position within a certain media file.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getBookmarks>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.9.0", path = "getBookmarks")]
pub struct GetBookmarks;

/// Creates or updates a bookmark (a position within a media file). Bookmarks are personal and not visible to other users.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#createBookmark>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.12.0", path = "getPlayQueue")]
pub struct GetPlayQueue;

///  Saves the state of the play queue for this user.
/// This includes the tracks in the play queue, the currently playing track, and the position within this track.
/// Typically used to allow a user to move between different clients/apps while retaining the same play queue (for instance when listening to an audio book).
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#savePlayQueue>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
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

#[cfg(test)]
mod tests {
    use super::super::tests::test_request_encode;
    use super::*;

    #[test]
    fn test_get_bookmarks() {
        let request = super::GetBookmarks;
        let query = test_request_encode(&request);
        assert_eq!("", query);
    }

    #[test]
    fn test_create_bookmark() {
        let request = super::CreateBookmark {
            id: "1".to_string(),
            position: Milliseconds::new(1000),
            comment: None,
        };
        let query = test_request_encode(&request);
        assert_eq!("id=1&position=1000", query);

        // Test with comment
        let request = super::CreateBookmark {
            id: "1".to_string(),
            position: Milliseconds::new(1000),
            comment: Some("test".to_string()),
        };
        let query = test_request_encode(&request);
        assert_eq!("id=1&position=1000&comment=test", query);
    }

    #[test]
    fn test_delete_bookmarks() {
        let request = super::DeleteBookmark {
            id: "1".to_string(),
        };
        let query = test_request_encode(&request);
        assert_eq!("id=1", query);
    }

    #[test]
    fn test_get_play_queue() {
        let request = super::GetPlayQueue;
        let query = test_request_encode(&request);
        assert_eq!("", query);
    }

    #[test]
    fn test_save_play_queue() {
        let request = super::SavePlayQueue {
            id: vec!["1".to_string(), "2".to_string()],
            current: Some("1".to_string()),
            position: Some(Milliseconds::new(1000)),
        };
        let query = test_request_encode(&request);
        assert_eq!("id=1&id=2&current=1&position=1000", query);

        // Test without current
        let request = super::SavePlayQueue {
            id: vec!["1".to_string(), "2".to_string()],
            current: None,
            position: Some(Milliseconds::new(1000)),
        };
        let query = test_request_encode(&request);
        assert_eq!("id=1&id=2&position=1000", query);

        // Test without position
        let request = super::SavePlayQueue {
            id: vec!["1".to_string(), "2".to_string()],
            current: Some("1".to_string()),
            position: None,
        };
        let query = test_request_encode(&request);
        assert_eq!("id=1&id=2&current=1", query);
    }
}
