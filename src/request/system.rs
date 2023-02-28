//! System requests.

use serde::{Deserialize, Serialize};
use subsonic_types_macro::{FromQuery, SubsonicRequest, ToQuery};

/// <http://www.subsonic.org/pages/api.jsp#ping>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.0.0", path = "ping")]
pub struct Ping;

/// <http://www.subsonic.org/pages/api.jsp#getLicense>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.0.0", path = "getLicense")]
pub struct GetLicense;
