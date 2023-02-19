use std::{str::FromStr, time::Duration};

use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

/// A date and time.
/// Use [`time::PrimitiveDateTime`] to convert to and from [`DateTime`].
#[derive(Debug, Clone, PartialEq)]
pub struct DateTime(PrimitiveDateTime);
//pub struct DateTime(chrono::DateTime<chrono::FixedOffset>);
impl_subsonic_for_serde!(DateTime);

impl From<PrimitiveDateTime> for DateTime {
    fn from(datetime: PrimitiveDateTime) -> Self {
        Self(datetime)
    }
}

impl From<DateTime> for PrimitiveDateTime {
    fn from(datetime: DateTime) -> Self {
        datetime.0
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self::from(PrimitiveDateTime::new(
            time::macros::date!(1970 - 01 - 01),
            time::macros::time!(00:00:00),
        ))
    }
}

impl FromStr for DateTime {
    type Err = time::error::Parse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PrimitiveDateTime::parse(
            s,
            time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]"),
        )
        .map(Self::from)
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .0
            .format(time::macros::format_description!(
                "[year]-[month]-[day]T[hour]:[minute]:[second]"
            ))
            .map_err(|_| std::fmt::Error)?;
        write!(f, "{string}")
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// A duration in milliseconds.
/// When used to represent an instant in time, it is relative to the Unix epoch.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Milliseconds(u64);
impl_subsonic_for_serde!(Milliseconds);

impl Milliseconds {
    pub fn new(milliseconds: u64) -> Self {
        Self(milliseconds)
    }

    pub fn to_duration(&self) -> std::time::Duration {
        self.into_duration()
    }

    pub fn into_duration(self) -> std::time::Duration {
        Duration::from_millis(self.0)
    }
}

impl From<u64> for Milliseconds {
    fn from(milliseconds: u64) -> Self {
        Self::new(milliseconds)
    }
}

impl From<Milliseconds> for Duration {
    fn from(milliseconds: Milliseconds) -> Self {
        milliseconds.into_duration()
    }
}

impl FromStr for Milliseconds {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.parse()?))
    }
}

/// A duration in seconds.
/// When used to represent an instant in time, it is relative to the Unix epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Seconds(u64);

impl Seconds {
    pub fn new(seconds: u64) -> Self {
        Self(seconds)
    }

    pub fn to_duration(&self) -> std::time::Duration {
        self.into_duration()
    }

    pub fn into_duration(self) -> std::time::Duration {
        Duration::from_secs(self.0)
    }
}

impl From<u64> for Seconds {
    fn from(seconds: u64) -> Self {
        Self::new(seconds)
    }
}

impl From<Seconds> for Duration {
    fn from(seconds: Seconds) -> Self {
        seconds.into_duration()
    }
}

impl FromStr for Seconds {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.parse()?))
    }
}

#[derive(Debug)]
pub struct InvalidVideoSize;

impl std::fmt::Display for InvalidVideoSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid video size")
    }
}

impl std::error::Error for InvalidVideoSize {}

/// A video size in pixels containing a width and a height.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VideoSize {
    pub width: u32,
    pub height: u32,
}

impl VideoSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl From<(u32, u32)> for VideoSize {
    fn from((width, height): (u32, u32)) -> Self {
        Self::new(width, height)
    }
}

impl From<VideoSize> for (u32, u32) {
    fn from(size: VideoSize) -> Self {
        (size.width, size.height)
    }
}

impl std::fmt::Display for VideoSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl std::str::FromStr for VideoSize {
    type Err = InvalidVideoSize;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('x');
        let width = parts
            .next()
            .ok_or(InvalidVideoSize)?
            .parse()
            .map_err(|_| InvalidVideoSize)?;
        let height = parts
            .next()
            .ok_or(InvalidVideoSize)?
            .parse()
            .map_err(|_| InvalidVideoSize)?;
        if parts.next().is_some() {
            return Err(InvalidVideoSize);
        }
        Ok(Self::new(width, height))
    }
}

impl Serialize for VideoSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for VideoSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug)]
pub struct InvalidVideoBitrate;

impl std::fmt::Display for InvalidVideoBitrate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid video bitrate")
    }
}

impl std::error::Error for InvalidVideoBitrate {}

/// A video bitrate, in kilobits per second, optionally containing a video size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VideoBitrate {
    pub bitrate: u32,
    pub size: Option<VideoSize>,
}

impl VideoBitrate {
    pub fn new(bitrate: u32, size: Option<VideoSize>) -> Self {
        Self { bitrate, size }
    }

    pub fn without_size(bitrate: u32) -> Self {
        Self::new(bitrate, None)
    }
}

impl std::fmt::Display for VideoBitrate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(size) = self.size {
            write!(f, "{}@{}", self.bitrate, size)
        } else {
            write!(f, "{}", self.bitrate)
        }
    }
}

impl std::str::FromStr for VideoBitrate {
    type Err = InvalidVideoBitrate;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('@');
        let bitrate = parts
            .next()
            .ok_or(InvalidVideoBitrate)?
            .parse()
            .map_err(|_| InvalidVideoBitrate)?;
        let size = parts
            .next()
            .map(|s| s.parse())
            .transpose()
            .map_err(|_| InvalidVideoBitrate)?;
        if parts.next().is_some() {
            return Err(InvalidVideoBitrate);
        }
        Ok(Self::new(bitrate, size))
    }
}

impl Serialize for VideoBitrate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for VideoBitrate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

/// An audio bitrate in kbit/s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AudioBitrate {
    /// No limit.
    NoLimit,
    /// 32 kbit/s.
    Kbps32,
    /// 40 kbit/s.
    Kbps40,
    /// 48 kbit/s.
    Kbps48,
    /// 56 kbit/s.
    Kbps56,
    /// 64 kbit/s.
    Kbps64,
    /// 80 kbit/s.
    Kbps80,
    /// 96 kbit/s.
    Kbps96,
    /// 112 kbit/s.
    Kbps112,
    /// 128 kbit/s.
    Kbps128,
    /// 160 kbit/s.
    Kbps160,
    /// 192 kbit/s.
    Kbps192,
    /// 224 kbit/s.
    Kbps224,
    /// 256 kbit/s.
    Kbps256,
    /// 320 kbit/s.
    Kbps320,
    /// Other bitrate.
    Other(u32),
}

impl AudioBitrate {
    pub fn new(bitrate: u32) -> Self {
        match bitrate {
            0 => Self::NoLimit,
            32 => Self::Kbps32,
            40 => Self::Kbps40,
            48 => Self::Kbps48,
            56 => Self::Kbps56,
            64 => Self::Kbps64,
            80 => Self::Kbps80,
            96 => Self::Kbps96,
            112 => Self::Kbps112,
            128 => Self::Kbps128,
            160 => Self::Kbps160,
            192 => Self::Kbps192,
            224 => Self::Kbps224,
            256 => Self::Kbps256,
            320 => Self::Kbps320,
            _ => Self::Other(bitrate),
        }
    }

    pub fn to_kbps(&self) -> u32 {
        match self {
            Self::NoLimit => 0,
            Self::Kbps32 => 32,
            Self::Kbps40 => 40,
            Self::Kbps48 => 48,
            Self::Kbps56 => 56,
            Self::Kbps64 => 64,
            Self::Kbps80 => 80,
            Self::Kbps96 => 96,
            Self::Kbps112 => 112,
            Self::Kbps128 => 128,
            Self::Kbps160 => 160,
            Self::Kbps192 => 192,
            Self::Kbps224 => 224,
            Self::Kbps256 => 256,
            Self::Kbps320 => 320,
            Self::Other(bitrate) => *bitrate,
        }
    }
}

impl From<u32> for AudioBitrate {
    fn from(bitrate: u32) -> Self {
        Self::new(bitrate)
    }
}

impl From<AudioBitrate> for u32 {
    fn from(bitrate: AudioBitrate) -> Self {
        bitrate.to_kbps()
    }
}

impl Serialize for AudioBitrate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.to_kbps())
    }
}

impl<'de> Deserialize<'de> for AudioBitrate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bitrate = u32::deserialize(deserializer)?;
        Ok(Self::new(bitrate))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MediaType {
    Music,
    Podcast,
    AudioBook,
    Video,
}
impl_subsonic_for_serde!(MediaType);

#[derive(Debug)]
pub struct InvalidUserRating;

impl std::fmt::Display for InvalidUserRating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid user rating")
    }
}

impl std::error::Error for InvalidUserRating {}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct UserRating(u32);
impl_subsonic_for_serde!(UserRating);

impl UserRating {
    pub fn new(value: u32) -> Result<Self, InvalidUserRating> {
        if !(1..=5).contains(&value) {
            Err(InvalidUserRating)
        } else {
            Ok(UserRating(value))
        }
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for UserRating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UserRating {
    type Err = InvalidUserRating;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse().map_err(|_| InvalidUserRating)?;
        UserRating::new(value)
    }
}

impl From<UserRating> for u32 {
    fn from(value: UserRating) -> Self {
        value.0
    }
}

impl Serialize for UserRating {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.0)
    }
}

impl<'de> Deserialize<'de> for UserRating {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        UserRating::new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug)]
pub struct InvalidAverageRating;

impl std::fmt::Display for InvalidAverageRating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid average rating")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AverageRating(f32);
impl_subsonic_for_serde!(AverageRating);

impl AverageRating {
    pub fn new(value: f32) -> Result<Self, InvalidAverageRating> {
        if !(1.0..=5.0).contains(&value) {
            Err(InvalidAverageRating)
        } else {
            Ok(AverageRating(value))
        }
    }

    pub fn value(self) -> f32 {
        self.0
    }
}

impl std::fmt::Display for AverageRating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for AverageRating {
    type Err = InvalidAverageRating;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse().map_err(|_| InvalidAverageRating)?;
        AverageRating::new(value)
    }
}

impl From<AverageRating> for f32 {
    fn from(value: AverageRating) -> Self {
        value.0
    }
}

impl Serialize for AverageRating {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f32(self.0)
    }
}

impl<'de> Deserialize<'de> for AverageRating {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = f32::deserialize(deserializer)?;
        AverageRating::new(value).map_err(serde::de::Error::custom)
    }
}

impl std::hash::Hash for AverageRating {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

#[derive(Debug)]
pub struct InvalidVersion;

impl std::fmt::Display for InvalidVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid version")
    }
}

impl std::error::Error for InvalidVersion {}

/// An API version.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}
impl_subsonic_for_serde!(Version);

impl Version {
    pub const LATEST: Self = Self::V1_16_1;
    pub const V1_16_1: Self = Self::new(1, 16, 1);
    pub const V1_16_0: Self = Self::new(1, 16, 0);
    pub const V1_15_0: Self = Self::new(1, 15, 0);
    pub const V1_14_0: Self = Self::new(1, 14, 0);
    pub const V1_13_0: Self = Self::new(1, 13, 0);
    pub const V1_12_0: Self = Self::new(1, 12, 0);
    pub const V1_11_0: Self = Self::new(1, 11, 0);
    pub const V1_10_2: Self = Self::new(1, 10, 2);
    pub const V1_9_0: Self = Self::new(1, 9, 0);
    pub const V1_8_0: Self = Self::new(1, 8, 0);
    pub const V1_7_0: Self = Self::new(1, 7, 0);
    pub const V1_6_0: Self = Self::new(1, 6, 0);
    pub const V1_5_0: Self = Self::new(1, 5, 0);
    pub const V1_4_0: Self = Self::new(1, 4, 0);
    pub const V1_3_0: Self = Self::new(1, 3, 0);
    pub const V1_2_0: Self = Self::new(1, 2, 0);
    pub const V1_1_1: Self = Self::new(1, 1, 1);
    pub const V1_1_0: Self = Self::new(1, 1, 0);

    pub const fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl std::str::FromStr for Version {
    type Err = InvalidVersion;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let major = parts
            .next()
            .ok_or(InvalidVersion)?
            .parse()
            .map_err(|_| InvalidVersion)?;
        let minor = parts
            .next()
            .ok_or(InvalidVersion)?
            .parse()
            .map_err(|_| InvalidVersion)?;
        let patch = parts
            .next()
            .ok_or(InvalidVersion)?
            .parse()
            .map_err(|_| InvalidVersion)?;
        Ok(Self::new(major, minor, patch))
    }
}

impl<N1, N2, N3> From<(N1, N2, N3)> for Version
where
    N1: Into<u8>,
    N2: Into<u8>,
    N3: Into<u8>,
{
    fn from(value: (N1, N2, N3)) -> Self {
        Self::new(value.0.into(), value.1.into(), value.2.into())
    }
}

impl serde::Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_milliseconds() {
        let ms = "123456789";
        let ms = ms.parse::<Milliseconds>().unwrap();
        assert_eq!(ms.0, 123456789);

        let ms = Milliseconds::new(123456789);
        let ms = serde_json::to_string(&ms).unwrap();
        assert_eq!(ms, "123456789");
    }

    #[test]
    fn test_seconds() {
        let s = "123456789";
        let s = s.parse::<Seconds>().unwrap();
        assert_eq!(s.0, 123456789);

        let s = Seconds::new(123456789);
        let s = serde_json::to_string(&s).unwrap();
        assert_eq!(s, "123456789");
    }

    #[test]
    fn test_video_size() {
        let s = "1920x1080";
        let s = s.parse::<VideoSize>().unwrap();
        assert_eq!(s.width, 1920);
        assert_eq!(s.height, 1080);

        let s = VideoSize::new(1920, 1080);
        let s = serde_json::to_string(&s).unwrap();
        assert_eq!(s, "\"1920x1080\"");
    }

    #[test]
    fn test_video_bitrate() {
        let b = "123456789";
        let b = b.parse::<VideoBitrate>().unwrap();
        assert_eq!(b.bitrate, 123456789);
        assert_eq!(b.size, None);

        let b = "123456789@1920x1080";
        let b = b.parse::<VideoBitrate>().unwrap();
        assert_eq!(b.bitrate, 123456789);
        assert_eq!(b.size, Some(VideoSize::new(1920, 1080)));
    }
}
