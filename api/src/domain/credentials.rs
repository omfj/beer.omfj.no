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

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct Password(String);

impl Password {
    pub fn parse(value: String) -> Result<Self, InvalidCredential> {
        if (3..=255).contains(&value.len()) {
            Ok(Self(value))
        } else {
            Err(InvalidCredential::Password)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Password, Username};

    #[test]
    fn validates_usernames_like_the_svelte_app() {
        assert!(Username::parse("Olem123".into()).is_ok());
        assert!(Username::parse("ab".into()).is_err());
        assert!(Username::parse("not-valid".into()).is_err());
    }

    #[test]
    fn validates_passwords_like_the_svelte_app() {
        assert!(Password::parse("abc".into()).is_ok());
        assert!(Password::parse("ab".into()).is_err());
    }
}
