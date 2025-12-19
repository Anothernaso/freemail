use anyhow::Result;
use serde::{Deserialize, Serialize};
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

    pub fn as_str(&self) -> &str {
        &self.username
    }

    pub fn validate(&self) -> Result<(), UsernameError> {
        validate_username(&self.username)
    }
}

pub fn validate_username(username: &str) -> Result<(), UsernameError> {
    if username.trim().is_empty() {
        return Err(UsernameError::Blank);
    }

    username
        .chars()
        .find(|c| !CHAR_WHITELIST.contains(c))
        .map_or(Ok(()), |c| Err(UsernameError::InvalidChar { char: c }))
}

#[derive(Debug, Error)]
pub enum UsernameError {
    #[error("password cannot be blank")]
    Blank,

    #[error("invalid character: {char}")]
    InvalidChar { char: char },
}
