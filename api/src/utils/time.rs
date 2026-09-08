use std::time::{SystemTime, UNIX_EPOCH};

use crate::domain::time::UnixSeconds;

pub fn now() -> UnixSeconds {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    UnixSeconds::from_seconds(i64::try_from(seconds).unwrap_or(i64::MAX))
}
