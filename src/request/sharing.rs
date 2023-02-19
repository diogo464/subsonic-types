use serde::{Deserialize, Serialize};
use subsonic_macro::SubsonicRequest;

use crate::common::Milliseconds;

/// Returns information about shared media this user is allowed to manage. Takes no extra parameters.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getShares>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.6.0", path = "getShares")]
pub struct GetShares;

/// Creates a public URL that can be used by anyone to stream music or video from the Subsonic server.
/// The URL is short and suitable for posting on Facebook, Twitter etc.
/// Note: The user must be authorized to share (see Settings > Users > User is allowed to share files with anyone).
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#createShare>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.6.0", path = "createShare")]
pub struct CreateShare {
    /// ID of a song, album or video to share. Use one id parameter for each entry to share
    #[serde(default)]
    pub id: Vec<String>,
    /// A user-defined description that will be displayed to people visiting the shared media.
    pub description: Option<String>,
    /// The time at which the share expires.
    pub expires: Option<Milliseconds>,
}

/// Updates the description and/or expiration date for an existing share.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#updateShare>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.6.0", path = "updateShare")]
pub struct UpdateShare {
    /// ID of the share to update.
    pub id: String,
    /// A user-defined description that will be displayed to people visiting the shared media.
    pub description: Option<String>,
    /// The time at which the share expires. Set to zero to remove the expiration.
    pub expires: Option<Milliseconds>,
}

/// Deletes an existing share.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#deleteShare>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.6.0", path = "deleteShare")]
pub struct DeleteShare {
    /// ID of the share to delete.
    pub id: String,
}
