//! System requests.

use serde::{Deserialize, Serialize};
use subsonic_macro::SubsonicRequest;

/// <http://www.subsonic.org/pages/api.jsp#ping>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.0.0", path = "ping")]
pub struct Ping;

/// <http://www.subsonic.org/pages/api.jsp#getLicense>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.0.0", path = "getLicense")]
pub struct GetLicense;
