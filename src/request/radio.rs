use serde::{Deserialize, Serialize};

///Returns all internet radio stations. Takes no extra parameters.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getInternetRadioStations>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInternetRadioStations;

/// Adds a new internet radio station.
/// Only users with admin privileges are allowed to call this method.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#createInternetRadioStation>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateInternetRadioStation {
    /// The stream URL for the station.
    pub stream_url: String,
    /// The user-defined name for the station.
    pub name: String,
    /// The home page URL for the station.
    pub homepage_url: Option<String>,
}

/// Updates an existing internet radio station.
/// Only users with admin privileges are allowed to call this method.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#updateInternetRadioStation>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateRadioStation {
    /// The ID for the station.
    pub id: String,
    /// The stream URL for the station.
    pub stream_url: String,
    /// The user-defined name for the station.
    pub name: String,
    /// The home page URL for the station.
    pub homepage_url: Option<String>,
}

/// Deletes an existing internet radio station.
/// Only users with admin privileges are allowed to call this method.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#deleteInternetRadioStation>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteInternetRadioStation {
    /// The ID for the station.
    pub id: String,
}
