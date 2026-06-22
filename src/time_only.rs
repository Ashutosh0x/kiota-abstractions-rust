//! A time-only type for Kiota SDKs (no date component).

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::KiotaError;

/// Represents a time without a date component (ISO 8601 format: HH:MM:SS).
/// Wraps `chrono::NaiveTime` with Kiota-specific serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TimeOnly(chrono::NaiveTime);

impl TimeOnly {
    /// Creates a new `TimeOnly` from hours, minutes, and seconds.
    pub fn new(hour: u32, min: u32, sec: u32) -> Option<Self> {
        chrono::NaiveTime::from_hms_opt(hour, min, sec).map(TimeOnly)
    }

    /// Returns the underlying `chrono::NaiveTime`.
    pub fn inner(&self) -> chrono::NaiveTime {
        self.0
    }

    /// Returns the hour (0-23).
    pub fn hour(&self) -> u32 {
        self.0.hour()
    }

    /// Returns the minute (0-59).
    pub fn minute(&self) -> u32 {
        self.0.minute()
    }

    /// Returns the second (0-59).
    pub fn second(&self) -> u32 {
        self.0.second()
    }
}

use chrono::Timelike;

impl fmt::Display for TimeOnly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%H:%M:%S"))
    }
}

impl FromStr for TimeOnly {
    type Err = KiotaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        chrono::NaiveTime::parse_from_str(s, "%H:%M:%S")
            .map(TimeOnly)
            .map_err(|e| KiotaError::SerializationError(format!("Invalid time '{}': {}", s, e)))
    }
}

impl From<chrono::NaiveTime> for TimeOnly {
    fn from(time: chrono::NaiveTime) -> Self {
        TimeOnly(time)
    }
}

impl From<TimeOnly> for chrono::NaiveTime {
    fn from(time: TimeOnly) -> Self {
        time.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_only_new() {
        let time = TimeOnly::new(14, 30, 0).unwrap();
        assert_eq!(time.hour(), 14);
        assert_eq!(time.minute(), 30);
        assert_eq!(time.second(), 0);
    }

    #[test]
    fn test_time_only_display() {
        let time = TimeOnly::new(9, 5, 3).unwrap();
        assert_eq!(time.to_string(), "09:05:03");
    }

    #[test]
    fn test_time_only_from_str() {
        let time: TimeOnly = "14:30:00".parse().unwrap();
        assert_eq!(time.hour(), 14);
        assert_eq!(time.minute(), 30);
    }
}
