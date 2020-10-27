use super::protocol::SerdeValue;
use super::IntegerObject;
use super::StringObject;
use crate::impl_object;
use crate::Error;
use crate::Result;
use chrono::offset::FixedOffset;
use chrono::offset::Offset;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono_humanize::HumanTime;
use std::fmt;

#[derive(Copy, Clone)]
pub struct TimestampObject {
    time: i64,
    offset: i32,
}

impl TimestampObject {
    pub fn to_datetime(&self) -> Result<DateTime<FixedOffset>> {
        Ok(DateTime::from_utc(self.to_naive()?, self.to_offset()?))
    }

    pub fn now() -> Self {
        let now = chrono::offset::Local::now();
        let offset = now.offset().fix().local_minus_utc() / 60;
        let time = now.timestamp();
        Self { time, offset }
    }

    fn to_offset(&self) -> Result<FixedOffset> {
        // Git timestamp uses minute offset. chrono uses seconds.
        FixedOffset::east_opt(self.offset * 60)
            .ok_or_else(|| Error::InvalidTime(format!("invalid timezone offset: {}", self.offset)))
    }

    fn to_naive(&self) -> Result<NaiveDateTime> {
        NaiveDateTime::from_timestamp_opt(self.time, 0)
            .ok_or_else(|| Error::InvalidTime(format!("invalid timestamp epoch: {}", self.time)))
    }
}

impl From<git2::Time> for TimestampObject {
    fn from(value: git2::Time) -> Self {
        Self {
            time: value.seconds(),
            offset: value.offset_minutes(),
        }
    }
}

impl fmt::Display for TimestampObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.to_datetime() {
            Ok(datetime) => datetime.fmt(f),
            Err(_) => write!(f, "{} {}", self.time, self.offset),
        }
    }
}

impl_object! {
    impl TimestampObject {
        pub fn add(&self, seconds: &IntegerObject) -> Self {
            let mut value = self.clone();
            value.time = value.time.saturating_add(seconds.to_i64());
            value
        }

        pub fn lt(&self, other: &Self) -> bool {
            self.time < other.time
        }

        pub fn epoch(&self) -> i64 {
            self.time
        }

        pub fn offset(&self) -> i64 {
            self.offset as _
        }

        pub fn rfc3339(&self) -> Result<String> {
            Ok(self.to_datetime()?.to_rfc3339())
        }

        pub fn rfc2822(&self) -> Result<String> {
            Ok(self.to_datetime()?.to_rfc2822())
        }

        pub fn short(&self) -> Result<String> {
            Ok(self.to_datetime()?.format("%Y-%m-%d").to_string())
        }

        pub fn strftime(&self, format: &StringObject) -> Result<String> {
            Ok(self.to_datetime()?.format(format.as_ref()).to_string())
        }

        pub fn relative(&self) -> Result<String> {
            let human_time = HumanTime::from(self.to_datetime()?);
            Ok(human_time.to_string())
        }

        fn to_serde_value(&self) -> Result<SerdeValue> {
            Ok(self.rfc3339()?.into())
        }
    }
}
