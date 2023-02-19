use serde::{Deserialize, Serialize};
use subsonic_macro::SubsonicRequest;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JukeboxAction {
    Get,
    /// Since 1.7.0
    Status,
    /// Since 1.7.0
    Set,
    Start,
    Stop,
    Skip,
    Add,
    Clear,
    Remove,
    Shuffle,
    SetGain,
}

/// Controls the jukebox, i.e., playback directly on the server's audio hardware.
/// Note: The user must be authorized to control the jukebox (see Settings > Users > User is allowed to play files in jukebox mode).
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#jukeboxControl>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.2.0", path = "jukeboxControl")]
pub struct JukeboxControl {
    /// The operation to perform.
    pub action: JukeboxAction,
    /// Used by [`JukeboxAction::Skip`] and [`JukeboxAction::Remove`].
    /// Zero-based index of the song to skip to or remove.
    pub index: Option<u32>,
    /// Since 1.7.0
    /// Used by [`JukeboxAction::Skip`].
    /// Start playing this many seconds into the track.
    pub offset: Option<u32>,
    /// Used by [`JukeboxAction::Add`] and [`JukeboxAction::Set`].
    /// ID of song to add to the jukebox playlist.
    /// [`JukeboxAction::Set`] is similar to a [`JukeboxAction::Clear`] followed by a [`JukeboxAction::Add`], but will not change the currently playing track.)
    #[serde(default)]
    pub id: Vec<String>,
    /// Used by [`JukeboxAction::SetGain`] to control the playback volume.
    /// A float value between 0.0 and 1.0.
    pub gain: Option<f32>,
}
