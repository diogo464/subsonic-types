use serde::{Deserialize, Serialize};
use subsonic_macro::{FromQuery, SubsonicRequest, ToQuery};

use crate::common::{VideoBitrate, VideoSize};
#[allow(unused)]
use crate::{
    common::Seconds,
    request::browsing::{GetMusicDirectory, GetVideoInfo},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.0.0", path = "getCoverArt")]
pub struct Stream {
    /// A string which uniquely identifies the file to stream. Obtained by calls to [`GetMusicDirectory`].
    pub id: String,
    /// Since 1.2.0
    /// If specified, the server will attempt to limit the bitrate to this value, in kilobits per second. If set to zero, no limit is imposed.
    pub max_bit_rate: Option<u32>,
    /// Since 1.6.0
    /// Specifies the preferred target format (e.g., "mp3" or "flv") in case there are multiple applicable transcodings. Starting with 1.9.0 you can use the special value "raw" to disable transcoding.
    pub format: Option<String>,
    /// Only applicable to video streaming.
    /// If specified, start streaming at the given offset (in seconds) into the video.
    /// Typically used to implement video skipping.
    pub time_offset: Option<Seconds>,
    /// Since 1.6.0
    /// Only applicable to video streaming.
    /// Requested video size specified as WxH, for instance "640x480".
    pub size: Option<VideoSize>,
    /// Since 1.8.0
    /// If set to "true", the Content-Length HTTP header will be set to an estimated value for transcoded or downsampled media.
    pub estimate_content_length: Option<bool>,
    /// Since 1.14.0
    /// Only applicable to video streaming.
    /// Subsonic can optimize videos for streaming by converting them to MP4.
    /// If a conversion exists for the video in question, then setting this parameter to "true" will cause the converted video to be returned instead of the original.
    pub converted: Option<bool>,
}

/// Downloads a given media file.
/// Similar to [`Stream`], but this method returns the original media data without transcoding or downsampling.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#download>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.0.0", path = "download")]
pub struct Download {
    /// A string which uniquely identifies the file to download.
    /// Obtained by calls to [`GetMusicDirectory`].
    pub id: String,
}

/// Creates an HLS (HTTP Live Streaming) playlist used for streaming video or audio.
/// HLS is a streaming protocol implemented by Apple and works by breaking the overall stream into a sequence of small HTTP-based file downloads.
/// It's supported by iOS and newer versions of Android.
/// This method also supports adaptive bitrate streaming, see the bitRate parameter.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#hls>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.8.0", path = "hls")]
pub struct Hls {
    /// A string which uniquely identifies the file to stream. Obtained by calls to [`GetMusicDirectory`].
    pub id: String,
    /// If specified, the server will attempt to limit the bitrate to this value, in kilobits per second.
    /// If this parameter is specified more than once, the server will create a variant playlist, suitable for adaptive bitrate streaming.
    /// The playlist will support streaming at all the specified bitrates.
    /// The server will automatically choose video dimensions that are suitable for the given bitrates.
    /// Since 1.9.0 you may explicitly request a certain width (480) and height (360) like so: bitRate=1000@480x360
    pub bit_rate: Option<VideoBitrate>,
    /// The ID of the audio track to use. See [`GetVideoInfo`] for how to get the list of available audio tracks for a video.
    pub audio_track: Option<String>,
}

/// Returns captions (subtitles) for a video. Use getVideoInfo to get a list of available captions.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getCaptions>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.14.0", path = "getCaptions")]
pub struct GetCaptions {
    /// The ID of the video.
    pub id: String,
    /// Preferred captions format ("srt" or "vtt").
    pub format: Option<String>,
}

/// Returns a cover art image.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getCoverArt>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.0.0", path = "getCoverArt")]
pub struct GetCoverArt {
    /// The ID of a song, album or artist.
    pub id: String,
    /// If specified, scale image to this size.
    pub size: Option<String>,
}

/// Searches for and returns lyrics for a given song.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getLyrics>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.2.0", path = "getLyrics")]
pub struct GetLyrics {
    /// The artist name.
    pub artist: Option<String>,
    /// The song title.
    pub title: Option<String>,
}

/// Returns the avatar (personal image) for a user.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getAvatar>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.8.0", path = "getAvatar")]
pub struct GetAvatar {
    /// The user in question.
    pub username: String,
}
