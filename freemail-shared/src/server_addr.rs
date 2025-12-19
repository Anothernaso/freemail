use crate::misc::BLANK_CHARS;
use anyhow::Result;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServerAddr {
    host: String,
    port: u16,
}

impl ServerAddr {
    pub fn new(host: String, port: u16) -> Result<Self, ServerAddrError> {
        if host.trim().is_empty() {
            return Err(ServerAddrError::HostError(HostError::Blank));
        }

        if host.chars().any(|c| BLANK_CHARS.contains(&c)) {
            return Err(ServerAddrError::HostError(HostError::HasBlankChar));
        }

        Ok(ServerAddr { host, port })
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> &u16 {
        &self.port
    }

    pub fn parse(addr: &str) -> Result<Self, ServerAddrError> {
        if addr.trim().is_empty() {
            return Err(ServerAddrError::ParseError(ParseError::Blank));
        }

        if addr.chars().any(|c| BLANK_CHARS.contains(&c)) {
            return Err(ServerAddrError::ParseError(ParseError::HasBlankChar));
        }

        let delimiters = addr.chars().filter(|&c| c == ':').count();
        if delimiters != 1 {
            return Err(ServerAddrError::ParseError(
                ParseError::InvalidDelimiterCount {
                    expected: 1,
                    supplied: delimiters,
                },
            ));
        }

        let mut parts = addr.splitn(2, ':');
        let host = parts.next().expect("unreachable");
        let port = match parts.next().expect("unreachable").parse::<u16>() {
            Ok(value) => value,
            Err(err) => {
                return Err(ServerAddrError::ParseError(ParseError::InvalidPort(
                    anyhow!(err),
                )));
            }
        };

        Ok(Self::new(host.to_string(), port)?)
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", &self.host, &self.port)
    }
}

#[derive(Debug, Error)]
pub enum ServerAddrError {
    #[error("host error: {0}")]
    HostError(HostError),

    #[error("parse error: {0}")]
    ParseError(ParseError),
}

#[derive(Debug, Error)]
pub enum HostError {
    #[error("host cannot be blank")]
    Blank,

    #[error("host cannot contain blank characters")]
    HasBlankChar,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("server address cannot be blank")]
    Blank,

    #[error("server address cannot contain blank characters")]
    HasBlankChar,

    #[error("port must be a valid u16")]
    InvalidPort(anyhow::Error),

    #[error("expected {expected} delimiters but {supplied} were supplied")]
    InvalidDelimiterCount { expected: usize, supplied: usize },
}
