use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;
use thiserror::Error;

const CHAR_WHITELIST: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.',
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Username {
    username: String,
}

impl Username {
    pub fn new(username: String) -> Result<Self, UsernameError> {
        validate_username(&username)?;

        Ok(Self { username })
    }

    pub fn validate(&self) -> Result<(), UsernameError> {
        validate_username(&self.username)
    }
}

impl Deref for Username {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.username
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.username)
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.username
    }
}

pub fn validate_username(username: &str) -> Result<(), UsernameError> {
    username
        .chars()
        .find(|c| !CHAR_WHITELIST.contains(c))
        .map_or(Ok(()), |c| Err(UsernameError::InvalidCharacter { char: c }))
}

#[derive(Debug, Error)]
pub enum UsernameError {
    #[error("invalid character: {char}")]
    InvalidCharacter { char: char },
}
