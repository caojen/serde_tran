use std::fmt::{Display, Formatter};

pub type Result<T> = core::result::Result<T, ErrorKind>;

#[derive(Debug)]
pub enum ErrorKind {
    IOError(std::io::Error),
    /// See [bincode::Error] or [bincode::ErrorKind]
    BincodeError(bincode::Error),
    /// Hash error when deserializing. (expected, got)
    HashError(u64, u64),
    /// Custom error
    CustomError(String),
    #[cfg(feature = "bs58")]
    /// base58 decode error
    Base58DecodeError(bs58::decode::Error),
    #[cfg(feature = "base64")]
    /// base64 decode error
    Base64DecodeError(base64::DecodeError),
    #[cfg(feature = "serde_json")]
    /// serde_json decode and encode error
    SerdeJsonError(serde_json::Error),
}

fn error_kind_feature_display_arm(kind: &ErrorKind, f: &mut Formatter<'_>) -> std::fmt::Result {
    #[cfg(feature = "bs58")]
    if let ErrorKind::Base58DecodeError(err) = kind {
        return Display::fmt(err, f)
    }

    #[cfg(feature = "base64")]
    if let ErrorKind::Base64DecodeError(err) = kind {
        return Display::fmt(err, f)
    }

    #[cfg(feature = "serde_json")]
    if let ErrorKind::SerdeJsonError(err) = kind {
        return Display::fmt(err, f)
    }

    unreachable!()
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) => write!(f, "io::error: {}", err),
            Self::BincodeError(err) => write!(f, "bincode::error: {}", err),
            Self::HashError(expected, got) => write!(f, "hash error: expected {} but got {}", expected, got),
            Self::CustomError(s) => Display::fmt(s, f),

            _ => error_kind_feature_display_arm(self, f)
        }
    }
}

impl std::error::Error for ErrorKind {}

impl From<bincode::Error> for ErrorKind {
    fn from(err: bincode::Error) -> Self {
        Self::BincodeError(err)
    }
}

impl From<std::io::Error> for ErrorKind {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

#[cfg(feature = "bs58")]
impl From<bs58::decode::Error> for ErrorKind {
    #[inline]
    fn from(err: bs58::decode::Error) -> Self {
        Self::Base58DecodeError(err)
    }
}

#[cfg(feature = "base64")]
impl From<base64::DecodeError> for ErrorKind {
    #[inline]
    fn from(err: base64::DecodeError) -> Self {
        Self::Base64DecodeError(err)
    }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Error> for ErrorKind {
    #[inline]
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJsonError(err)
    }
}
