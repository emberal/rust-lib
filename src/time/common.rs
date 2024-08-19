use chrono::NaiveDateTime;
use derive_more::Constructor;

#[derive(Debug, Clone, Copy, Constructor)]
pub struct DateTimeInterval {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}
