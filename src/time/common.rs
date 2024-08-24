use chrono::NaiveDateTime;
use derive_more::{Constructor, From};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Constructor, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DateTimeInterval {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

impl DateTimeInterval {
    /// Creates a new `DateTimeInterval` from the given `start` and `end` `NaiveDateTime`s.
    /// The `start` must be before the `end`.
    /// If `start` is equal to or after `end`, this function will return `None`.
    /// # Arguments
    /// * `start` - The start of the interval.
    /// * `end` - The end of the interval.
    /// # Returns
    /// A new `DateTimeInterval` if `start` is before `end`, otherwise `None`.
    /// # Examples
    /// ```
    /// use chrono::{NaiveDateTime, Duration, Utc};
    /// use lib::time::common::DateTimeInterval;
    ///
    /// let start = Utc::now().naive_utc();
    /// let end = start + Duration::days(1);
    /// let interval = DateTimeInterval::new_safe(start, end);
    /// assert_eq!(interval, Some((start, end).into()));
    /// ```
    /// Illegal interval:
    /// ```
    /// use chrono::{NaiveDateTime, Utc};
    /// use lib::time::common::DateTimeInterval;
    /// let start = Utc::now().naive_utc();
    /// let end = start;
    /// let interval = DateTimeInterval::new_safe(start, end);
    /// assert_eq!(interval, None);
    pub fn new_safe(start: NaiveDateTime, end: NaiveDateTime) -> Option<Self> {
        if start < end {
            Some(Self::new(start, end))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;

    #[test]
    fn test_new_safe() {
        let start = DateTime::from_timestamp(0, 0).unwrap().naive_utc();
        let end = DateTime::from_timestamp(1, 0).unwrap().naive_utc();
        assert_eq!(
            DateTimeInterval::new_safe(start, end),
            Some(DateTimeInterval::new(start, end))
        );
        assert_eq!(DateTimeInterval::new_safe(end, start), None);
    }
}
