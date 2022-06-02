use chrono::{DateTime, NaiveDateTime, Utc};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use time::OffsetDateTime;

pub(crate) type JsCallback<T> = ThreadsafeFunction<T, ErrorStrategy::CalleeHandled>;

pub(crate) fn to_datetime(time: OffsetDateTime) -> DateTime<Utc> {
    DateTime::from_utc(NaiveDateTime::from_timestamp(time.unix_timestamp(), 0), Utc)
}

pub(crate) fn from_datetime(time: DateTime<Utc>) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(time.timestamp()).expect("invalid timestamp")
}
