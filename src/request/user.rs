use serde::{Deserialize, Serialize};
use subsonic_types_macro::{FromQuery, SubsonicRequest, ToQuery};

use crate::common::AudioBitrate;

/// Get details about a given user, including which authorization roles and folder access it has.
/// Can be used to enable/disable certain features in the client, such as jukebox control.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getUser>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.3.0", path = "getUser")]
pub struct GetUser {
    /// The name of the user to retrieve.
    /// You can only retrieve your own user unless you have admin privileges.
    pub username: String,
}

///  Get details about all users, including which authorization roles and folder access they have.
/// Only users with admin privileges are allowed to call this method.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getUsers>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.8.0", path = "getUsers")]
pub struct GetUsers;

/// Creates a new Subsonic user, using the following parameters.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#createUser>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.1.0", path = "createUser")]
pub struct CreateUser {
    /// The name of the new user.
    pub username: String,
    /// The password of the new user, either in clear text of hex-encoded.
    pub password: String,
    /// The email address of the new user.
    pub email: String,
    /// Whether the user is authenicated in LDAP.
    pub ldap_authenticated: Option<bool>,
    /// Whether the user is administrator.
    pub admin_role: Option<bool>,
    /// Whether the user is allowed to change personal settings and password.
    pub settings_role: Option<bool>,
    /// Whether the user is allowed to play files.
    pub stream_role: Option<bool>,
    /// Whether the user is allowed to play files in jukebox mode.
    pub jukebox_role: Option<bool>,
    /// Whether the user is allowed to download files.
    pub download_role: Option<bool>,
    /// Whether the user is allowed to upload files.
    pub upload_role: Option<bool>,
    /// Whether the user is allowed to create and delete playlists. Since 1.8.0, changing this role has no effect.
    pub playlist_role: Option<bool>,
    /// Whether the user is allowed to change cover art and tags.
    pub covert_art_role: Option<bool>,
    /// Whether the user is allowed to create and edit comments and ratings.
    pub comment_role: Option<bool>,
    /// Whether the user is allowed to administrate Podcasts.
    pub podcast_role: Option<bool>,
    /// Since 1.8.0
    /// Whether the user is allowed to share files with anyone.
    pub share_role: Option<bool>,
    /// Since 1.15.0
    /// Whether the user is allowed to start video conversions.
    pub video_conversion_role: Option<bool>,
    /// Since 1.12.0
    /// IDs of the music folders the user is allowed access to. Include the parameter once for each folder.
    #[serde(default)]
    pub music_folder_id: Vec<String>,
}

///  Modifies an existing Subsonic user, using the following parameters.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#updateUser>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.10.1", path = "updateUser")]
pub struct UpdateUser {
    /// The name of the user.
    pub username: String,
    /// The password of the new user, either in clear text of hex-encoded.
    pub password: Option<String>,
    /// The email address of the new user.
    pub email: Option<String>,
    /// Whether the user is authenicated in LDAP.
    pub ldap_authenticated: Option<bool>,
    /// Whether the user is administrator.
    pub admin_role: Option<bool>,
    /// Whether the user is allowed to change personal settings and password.
    pub settings_role: Option<bool>,
    /// Whether the user is allowed to play files.
    pub stream_role: Option<bool>,
    /// Whether the user is allowed to play files in jukebox mode.
    pub jukebox_role: Option<bool>,
    /// Whether the user is allowed to download files.
    pub download_role: Option<bool>,
    /// Whether the user is allowed to upload files.
    pub upload_role: Option<bool>,
    /// Whether the user is allowed to change cover art and tags.
    pub covert_art_role: Option<bool>,
    /// Whether the user is allowed to create and edit comments and ratings.
    pub comment_role: Option<bool>,
    /// Whether the user is allowed to administrate Podcasts.
    pub podcast_role: Option<bool>,
    /// Whether the user is allowed to share files with anyone.
    pub share_role: Option<bool>,
    /// Since 1.15.0
    /// Whether the user is allowed to start video conversions.
    pub video_conversion_role: Option<bool>,
    /// Since 1.12.0
    /// IDs of the music folders the user is allowed access to.
    #[serde(default)]
    pub music_folder_id: Vec<String>,
    /// Since 1.13.0
    /// he maximum bit rate (in Kbps) for the user.
    /// Audio streams of higher bit rates are automatically downsampled to this bit rate.
    /// Legal values: 0 (no limit), 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320.
    pub max_bit_rate: Option<AudioBitrate>,
}

/// Deletes an existing Subsonic user, using the following parameters.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#deleteUser>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.3.0", path = "deleteUser")]
pub struct DeleteUser {
    /// The name of the user to delete.
    pub username: String,
}

/// Changes the password of an existing Subsonic user, using the following parameters.
/// You can only change your own password unless you have admin privileges.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.1.0", path = "changePassword")]
pub struct ChangePassword {
    /// The name of the user which should change its password.
    pub username: String,
    /// The new password of the new user, either in clear text of hex-encoded.
    pub password: String,
}
