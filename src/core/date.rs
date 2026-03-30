use std::fmt;

use chrono::NaiveDate;
use serde::Deserialize;

/// A date with no time-of-day or timezone information, used for article publication and update dates.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(try_from = "String")]
pub struct Date {
    inner: NaiveDate,
}

impl Date {
    /// Creates a Date from year, month, and day components.
    /// Returns None if the date is invalid.
    pub fn from_year_month_day(year: i32, month: u32, day: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, day).map(|inner| Self { inner })
    }
}

impl TryFrom<String> for Date {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        NaiveDate::parse_from_str(&value, "%Y-%m-%d")
            .map(|inner| Self { inner })
            .map_err(|e| e.to_string())
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner.format("%Y-%m-%d"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_date_as_iso_8601_string() {
        let date = Date::from_year_month_day(2026, 3, 28).unwrap();

        assert_eq!(date.to_string(), "2026-03-28");
    }

    #[test]
    fn earlier_date_is_less_than_later_date() {
        let earlier = Date::from_year_month_day(2026, 3, 28).unwrap();
        let later = Date::from_year_month_day(2026, 3, 30).unwrap();

        assert!(earlier < later);
    }

    #[test]
    fn deserializes_from_iso_8601_json_string() {
        let date: Date = serde_json::from_str(r#""2026-03-28""#).unwrap();

        assert_eq!(date, Date::from_year_month_day(2026, 3, 28).unwrap());
    }
}