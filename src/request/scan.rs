use serde::{Deserialize, Serialize};
use subsonic_types_macro::{FromQuery, SubsonicRequest, ToQuery};

/// Returns the current status for media library scanning. Takes no extra parameters.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getScanStatus>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.15.0", path = "getScanStatus")]
pub struct GetScanStatus;

/// Initiates a rescan of the media libraries. Takes no extra parameters.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#startScan>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.15.0", path = "startScan")]
pub struct StartScan;
