use serde::{Deserialize, Serialize};

/// An instant measured in whole seconds since the Unix epoch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct UnixSeconds(i64);

impl UnixSeconds {
    pub fn from_seconds(seconds: i64) -> Self {
        Self(seconds)
    }

    pub fn as_seconds(self) -> i64 {
        self.0
    }
}

/// A session expires after 30 days and can renew during its final 15 days.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SessionExpiry(UnixSeconds);

impl SessionExpiry {
    const LIFETIME_SECONDS: i64 = 60 * 60 * 24 * 30;
    const RENEWAL_WINDOW_SECONDS: i64 = 60 * 60 * 24 * 15;

    pub fn from_timestamp(timestamp: UnixSeconds) -> Self {
        Self(timestamp)
    }

    pub fn new(now: UnixSeconds) -> Self {
        Self(UnixSeconds(now.0.saturating_add(Self::LIFETIME_SECONDS)))
    }

    pub fn timestamp(self) -> UnixSeconds {
        self.0
    }

    pub fn is_expired(self, now: UnixSeconds) -> bool {
        now >= self.0
    }

    pub fn should_renew(self, now: UnixSeconds) -> bool {
        !self.is_expired(now) && now.0 >= self.0.0.saturating_sub(Self::RENEWAL_WINDOW_SECONDS)
    }

    pub fn remaining_seconds(self, now: UnixSeconds) -> i64 {
        self.0.0.saturating_sub(now.0).max(0)
    }
}

/// The UTC calendar year used by the leaderboard's SQLite query.
/// Any integer remains accepted, preserving existing query behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LeaderboardYear(i64);

impl LeaderboardYear {
    pub fn new(year: i64) -> Self {
        Self(year)
    }

    pub fn value(self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{SessionExpiry, UnixSeconds};

    #[test]
    fn session_lifetime_and_renewal_boundaries() {
        let start = 1_700_000_000;
        let expiry = SessionExpiry::new(UnixSeconds::from_seconds(start));
        let end = start + 30 * 24 * 60 * 60;
        let renewal = start + 15 * 24 * 60 * 60;
        assert_eq!(expiry.timestamp().as_seconds(), end);
        assert!(!expiry.should_renew(UnixSeconds::from_seconds(renewal - 1)));
        assert!(expiry.should_renew(UnixSeconds::from_seconds(renewal)));
        assert!(expiry.should_renew(UnixSeconds::from_seconds(end - 1)));
        assert!(!expiry.is_expired(UnixSeconds::from_seconds(end - 1)));
        assert!(expiry.is_expired(UnixSeconds::from_seconds(end)));
        assert!(!expiry.should_renew(UnixSeconds::from_seconds(end)));
        assert!(!expiry.should_renew(UnixSeconds::from_seconds(end + 1)));
        assert_eq!(
            expiry.remaining_seconds(UnixSeconds::from_seconds(end - 1)),
            1
        );
        assert_eq!(expiry.remaining_seconds(UnixSeconds::from_seconds(end)), 0);
        assert_eq!(
            expiry.remaining_seconds(UnixSeconds::from_seconds(end + 1)),
            0
        );
        let renewed = SessionExpiry::new(UnixSeconds::from_seconds(renewal));
        assert_eq!(
            renewed.timestamp().as_seconds(),
            renewal + 30 * 24 * 60 * 60
        );
    }

    #[test]
    fn extreme_timestamps_do_not_overflow() {
        let earliest = UnixSeconds::from_seconds(i64::MIN);
        let latest = UnixSeconds::from_seconds(i64::MAX);
        let expiry = SessionExpiry::new(latest);
        assert_eq!(expiry.timestamp(), latest);
        assert_eq!(expiry.remaining_seconds(earliest), i64::MAX);
        assert!(!SessionExpiry::from_timestamp(earliest).should_renew(latest));
    }
}
