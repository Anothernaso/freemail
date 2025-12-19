use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const CHAR_WHITELIST: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '.', '_', '-', '+', '=', '!', '?', '@', '#', '$', '%', '&', '*', '(',
    ')', '[', ']', '{', '}', ':', ';', ',', '/',
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Password {
    password: String,
}

impl Password {
    pub fn new(password: String) -> Result<Self, PasswordError> {
        validate_password(&password)?;

        Ok(Self { password })
    }

    pub fn as_str(&self) -> &str {
        &self.password
    }

    pub fn validate(&self) -> Result<(), PasswordError> {
        validate_password(&self.password)
    }
}

pub fn validate_password(password: &str) -> Result<(), PasswordError> {
    if password.trim().is_empty() {
        return Err(PasswordError::Blank);
    }

    password
        .chars()
        .find(|c| !CHAR_WHITELIST.contains(c))
        .map_or(Ok(()), |c| Err(PasswordError::InvalidChar { char: c }))
}

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("password cannot be blank")]
    Blank,

    #[error("invalid character: {char}")]
    InvalidChar { char: char },
}
