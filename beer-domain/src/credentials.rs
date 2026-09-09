use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidCredential {
    #[error("username must be 3 to 255 ASCII alphanumeric characters")]
    Username,
    #[error("password must be 3 to 255 characters")]
    Password,
}

pub struct Username(String);

impl Username {
    /// Parses a username.
    ///
    /// # Errors
    /// Returns an error unless the value is 3–255 ASCII alphanumeric bytes.
    pub fn parse(value: String) -> Result<Self, InvalidCredential> {
        if (3..=255).contains(&value.len())
            && value
                .chars()
                .all(|character| character.is_ascii_alphanumeric())
        {
            Ok(Self(value))
        } else {
            Err(InvalidCredential::Username)
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct Password(String);

impl Password {
    /// Parses a password using the existing byte-length validation.
    ///
    /// # Errors
    /// Returns an error unless the value is 3–255 bytes long.
    pub fn parse(value: String) -> Result<Self, InvalidCredential> {
        if (3..=255).contains(&value.len()) {
            Ok(Self(value))
        } else {
            Err(InvalidCredential::Password)
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Username;

    #[test]
    fn validates_usernames() {
        assert!(Username::parse("Olem123".into()).is_ok());
        assert!(Username::parse("ab".into()).is_err());
        assert!(Username::parse("not-valid".into()).is_err());
    }
}
