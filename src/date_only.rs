//! A date-only type for Kiota SDKs (no time component).

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::KiotaError;

/// Represents a date without a time component (ISO 8601 format: YYYY-MM-DD).
/// Wraps `chrono::NaiveDate` with Kiota-specific serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DateOnly(chrono::NaiveDate);

impl DateOnly {
    /// Creates a new `DateOnly` from year, month, and day.
    pub fn new(year: i32, month: u32, day: u32) -> Option<Self> {
        chrono::NaiveDate::from_ymd_opt(year, month, day).map(DateOnly)
    }

    /// Returns the underlying `chrono::NaiveDate`.
    pub fn inner(&self) -> chrono::NaiveDate {
        self.0
    }

    /// Returns the year.
    pub fn year(&self) -> i32 {
        self.0.year()
    }

    /// Returns the month (1-12).
    pub fn month(&self) -> u32 {
        self.0.month()
    }

    /// Returns the day of the month (1-31).
    pub fn day(&self) -> u32 {
        self.0.day()
    }
}

use chrono::Datelike;

impl fmt::Display for DateOnly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

impl FromStr for DateOnly {
    type Err = KiotaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map(DateOnly)
            .map_err(|e| KiotaError::SerializationError(format!("Invalid date '{}': {}", s, e)))
    }
}

impl From<chrono::NaiveDate> for DateOnly {
    fn from(date: chrono::NaiveDate) -> Self {
        DateOnly(date)
    }
}

impl From<DateOnly> for chrono::NaiveDate {
    fn from(date: DateOnly) -> Self {
        date.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_only_new() {
        let date = DateOnly::new(2026, 6, 23).unwrap();
        assert_eq!(date.year(), 2026);
        assert_eq!(date.month(), 6);
        assert_eq!(date.day(), 23);
    }

    #[test]
    fn test_date_only_display() {
        let date = DateOnly::new(2026, 1, 5).unwrap();
        assert_eq!(date.to_string(), "2026-01-05");
    }

    #[test]
    fn test_date_only_from_str() {
        let date: DateOnly = "2026-06-23".parse().unwrap();
        assert_eq!(date.year(), 2026);
        assert_eq!(date.month(), 6);
        assert_eq!(date.day(), 23);
    }

    #[test]
    fn test_date_only_invalid() {
        let result: Result<DateOnly, _> = "not-a-date".parse();
        assert!(result.is_err());
    }
}
