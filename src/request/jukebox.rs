use serde::{Deserialize, Serialize};
use subsonic_macro::SubsonicRequest;

use crate::{impl_from_query_value_for_parse, impl_to_query_value_for_display};

#[derive(Debug)]
pub struct InvalidJukeboxAction;

impl std::fmt::Display for InvalidJukeboxAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid jukebox action")
    }
}

impl std::error::Error for InvalidJukeboxAction {}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
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
impl_from_query_value_for_parse!(JukeboxAction);
impl_to_query_value_for_display!(JukeboxAction);

impl JukeboxAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            JukeboxAction::Get => "get",
            JukeboxAction::Status => "status",
            JukeboxAction::Set => "set",
            JukeboxAction::Start => "start",
            JukeboxAction::Stop => "stop",
            JukeboxAction::Skip => "skip",
            JukeboxAction::Add => "add",
            JukeboxAction::Clear => "clear",
            JukeboxAction::Remove => "remove",
            JukeboxAction::Shuffle => "shuffle",
            JukeboxAction::SetGain => "setGain",
        }
    }
}

impl std::fmt::Display for JukeboxAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for JukeboxAction {
    type Err = InvalidJukeboxAction;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "get" => Ok(JukeboxAction::Get),
            "status" => Ok(JukeboxAction::Status),
            "set" => Ok(JukeboxAction::Set),
            "start" => Ok(JukeboxAction::Start),
            "stop" => Ok(JukeboxAction::Stop),
            "skip" => Ok(JukeboxAction::Skip),
            "add" => Ok(JukeboxAction::Add),
            "clear" => Ok(JukeboxAction::Clear),
            "remove" => Ok(JukeboxAction::Remove),
            "shuffle" => Ok(JukeboxAction::Shuffle),
            "setGain" => Ok(JukeboxAction::SetGain),
            _ => Err(InvalidJukeboxAction),
        }
    }
}

impl serde::Serialize for JukeboxAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for JukeboxAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
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

#[cfg(test)]
mod tests {
    use super::super::tests::test_request_encode;
    use super::*;

    #[test]
    fn test_jukebox_control() {
        let request = JukeboxControl {
            action: JukeboxAction::Get,
            index: None,
            offset: None,
            id: vec![],
            gain: None,
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "action=get");

        let request = JukeboxControl {
            action: JukeboxAction::Status,
            index: None,
            offset: None,
            id: vec![],
            gain: None,
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "action=status");

        let request = JukeboxControl {
            action: JukeboxAction::Set,
            index: Some(1),
            offset: Some(20),
            id: vec!["1".to_string(), "2".to_string()],
            gain: None,
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "action=set&index=1&offset=20&id=1&id=2");

        let request = JukeboxControl {
            action: JukeboxAction::Start,
            index: None,
            offset: None,
            id: vec![],
            gain: Some(13.0),
        };
        let query = test_request_encode(&request);
        assert_eq!(query, "action=start&gain=13");
    }
}
