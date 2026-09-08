use std::time::{SystemTime, UNIX_EPOCH};

use crate::domain::time::UnixSeconds;

pub fn now() -> UnixSeconds {
    UnixSeconds::from_seconds(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64,
    )
}
