use base64::Engine;
use serde::{Deserialize, Serialize};
use crate::error;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Base {
    #[serde(rename = "base58")]
    Base58,
    #[serde(rename = "base64")]
    Base64,
}

impl Base {
    #[inline]
    pub(crate) fn base<T: AsRef<[u8]>>(&self, bytes: T) -> error::Result<String> {
        match *self {
            Self::Base58 => Self::base58(bytes),
            Self::Base64 => Self::base64(bytes),
        }
    }

    #[inline]
    pub(crate) fn from_base<T: AsRef<[u8]>>(&self, s: T) -> error::Result<Vec<u8>> {
        match *self {
            Self::Base58 => Self::from_base58(s),
            Self::Base64 => Self::from_base64(s),
        }
    }

    #[inline]
    fn base58<T: AsRef<[u8]>>(bytes: T) -> error::Result<String> {
        #[cfg(feature = "bs58")]
        {
            return Ok(bs58::encode(bytes).into_string());
        }

        #[cfg(not(feature = "bs58"))]
        {
            return Err(error::ErrorKind::CustomError("feature bs58 is not enabled".to_string()));
        }
    }

    #[inline]
    fn from_base58<T: AsRef<[u8]>>(s: T) -> error::Result<Vec<u8>> {
        #[cfg(feature = "bs58")]
        {
            return Ok(bs58::decode(s).into_vec()?)
        }

        #[cfg(not(feature = "bs58"))]
        {
            return Err(error::ErrorKind::CustomError("feature bs58 is not enabled".to_string()));
        }
    }

    #[inline]
    fn base64<T: AsRef<[u8]>>(bytes: T) -> error::Result<String> {
        #[cfg(feature = "base64")]
        {
            return Ok(base64::prelude::BASE64_URL_SAFE_NO_PAD.encode(bytes))
        }

        #[cfg(not(feature = "base64"))]
        {
            return Err(error::ErrorKind::CustomError("feature base64 is not enabled".to_string()))
        }
    }

    #[inline]
    fn from_base64<T: AsRef<[u8]>>(s: T) -> error::Result<Vec<u8>> {
        #[cfg(feature = "base64")]
        {
            return Ok(base64::prelude::BASE64_URL_SAFE_NO_PAD.decode(s)?)
        }

        #[cfg(not(feature = "base64"))]
        {
            return Err(error::ErrorKind::CustomError("feature base64 is not enabled".to_string()))
        }
    }
}

#[inline]
pub(crate) fn to_base<T: AsRef<[u8]>>(bytes: T, base: Base) -> error::Result<String> {
    base.base(bytes)
}

#[inline]
pub(crate) fn from_base<T: AsRef<[u8]>>(s: T, base: Base) -> error::Result<Vec<u8>> {
    base.from_base(s)
}

#[cfg(feature = "base64")]
/// convert data to base64 string
pub fn to_base64<T>(data: &T) -> error::Result<String>
    where T: Serialize
{
    let bytes = crate::to_vec(data)?;
    to_base(bytes, Base::Base64)
}

#[cfg(feature = "base64")]
/// convert data from base64 string to T
pub fn from_base64<T>(s: &str) -> error::Result<T>
    where
        T: for<'de> Deserialize<'de>,
{
    let bytes = from_base(s, Base::Base64)?;
    crate::from_slice(&bytes)
}

#[cfg(feature = "bs58")]
/// convert data to base58 string
pub fn to_base58<T>(data: &T) -> error::Result<String>
    where T: Serialize
{
    let bytes = crate::to_vec(data)?;
    to_base(bytes, Base::Base58)
}

#[cfg(feature = "bs58")]
/// convert data from base58 string to T
pub fn from_base58<T>(s: &str) -> error::Result<T>
    where T: for<'de> Deserialize<'de>
{
    let bytes = from_base(s, Base::Base58)?;
    crate::from_slice(&bytes)
}
