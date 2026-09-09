//! Opaque identifiers. Existing values remain accepted regardless of generator format.
//!
//! Distinct types prevent mixing identifiers across entities:
//! ```compile_fail
//! use beer_domain::id::{EventId, UserId};
//! let user: UserId = "user".into();
//! let event: EventId = user;
//! ```

use std::fmt;

use serde::{Deserialize, Serialize};

macro_rules! make_id {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.into())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

make_id!(UserId);
make_id!(EventId);
make_id!(DrinkId);
make_id!(ImageId);
