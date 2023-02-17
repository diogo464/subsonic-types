use serde::{Deserialize, Serialize};

/// Returns the current status for media library scanning. Takes no extra parameters.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getScanStatus>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScanStatus;

/// Initiates a rescan of the media libraries. Takes no extra parameters.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#startScan>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartScan;
