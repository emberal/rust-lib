use chrono::NaiveDateTime;
use derive_more::Constructor;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Constructor)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DateTimeInterval {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}
