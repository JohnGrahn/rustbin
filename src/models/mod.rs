use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize)]
pub struct PasteData {
    pub id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ExpirationTime {
    FiveMinutes,
    TenMinutes,
    ThirtyMinutes,
    OneHour,
    TwelveHours,
    OneDay,
    OneWeek,
    TwoWeeks,
    OneMonth,
}

impl ExpirationTime {
    #[cfg(feature = "server")]
    pub fn to_duration(&self) -> chrono::Duration {
        use chrono::Duration;
        match self {
            ExpirationTime::FiveMinutes => Duration::minutes(5),
            ExpirationTime::TenMinutes => Duration::minutes(10),
            ExpirationTime::ThirtyMinutes => Duration::minutes(30),
            ExpirationTime::OneHour => Duration::hours(1),
            ExpirationTime::TwelveHours => Duration::hours(12),
            ExpirationTime::OneDay => Duration::days(1),
            ExpirationTime::OneWeek => Duration::weeks(1),
            ExpirationTime::TwoWeeks => Duration::weeks(2),
            ExpirationTime::OneMonth => Duration::days(30),
        }
    }
}