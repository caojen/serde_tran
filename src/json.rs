use serde::{Deserialize, Serialize};
use crate::{Base, error, from_base58, from_base64, to_base58, to_base64};

pub type Format = Base;

#[derive(Serialize, Deserialize, Debug, Clone)]
/// [Json] stores the data and encoding.
pub struct Json {
    #[serde(rename = "f")]
    pub(crate) _format: Format,
    #[serde(rename = "v")]
    /// data is a basex string, see [Base]
    pub(crate) _data: String,
}

impl Json {
    #[inline]
    pub(crate) fn new(format: Format, data: String) -> Self {
        Self {
            _format: format,
            _data: data,
        }
    }

    /// return the format ([Format]) of the [data] function
    #[inline]
    pub fn format(&self) -> Format {
        self._format
    }

    /// return the data, which is a string in [format], see [Format]
    #[inline]
    pub fn data(&self) -> &str {
        &self._data
    }

    #[cfg(feature = "serde_json")]
    /// return the json string, using [serde_json::to_string]
    pub fn to_string(&self) -> error::Result<String> {
        let data = serde_json::to_string(&self)?;
        Ok(data)
    }

    #[cfg(feature = "serde_json")]
    /// return the bytes (in json format), using [serde_json::to_vec]
    pub fn to_vec(&self) -> error::Result<Vec<u8>> {
        let bytes = serde_json::to_vec(&self)?;
        Ok(bytes)
    }

    #[cfg(feature = "serde_json")]
    /// convert [Self] back to user data T
    pub fn to_value<T>(&self) -> error::Result<T>
        where T: for<'de> Deserialize<'de>
    {
        match self.format() {
            Base::Base58 => {
                #[cfg(feature = "bs58")]
                { from_base58(self.data()) }
                #[cfg(not(feature = "bs58"))]
                { Err(error::ErrorKind::CustomError("unknown data format")) }
            },
            Base::Base64 => {
                #[cfg(feature = "base64")]
                { from_base64(self.data()) }
                #[cfg(not(feature = "base64"))]
                { Err(error::ErrorKind::CustomError("unknown data format")) }
            },
        }
    }
}

#[cfg(feature = "serde_json")]
/// convert given data into [Json], then you can use [Json::to_string] or [Json::to_vec]
/// to get a json string.
///
/// feature: it first tries [to_json_base64], then tries [to_json_base58].
pub fn to_json<T>(data: &T) -> error::Result<Json>
    where T: Serialize
{
    #[cfg(feature = "base64")]
    {
        return to_json_base64(data);
    }

    #[cfg(feature = "bs58")]
    {
        return to_json_base58(data);
    }

    unreachable!("when serde_json is enabled, base64 and bs58 should be enabled at least one");
}

#[cfg(all(feature = "serde_json", feature = "base64"))]
/// convert given data into [Json], where format is [Base::Base64]
pub fn to_json_base64<T>(data: &T) -> error::Result<Json>
    where T: Serialize
{
    let data = to_base64(data)?;
    Ok(Json::new(Base::Base64, data))
}

#[cfg(all(feature = "serde_json", feature = "bs58"))]
/// convert given data into [Json], where format is [Base::Base58]
pub fn to_json_base58<T>(data: &T) -> error::Result<Json>
    where T: Serialize
{
    let data = to_base58(data)?;
    Ok(Json::new(Base::Base58, data))
}

#[cfg(feature = "serde_json")]
/// convert bytes back to [Json], then you can use [Json::to_value] to get your custom data.
pub fn from_json_slice<T: AsRef<[u8]>>(bytes: T) -> error::Result<Json> {
    let json: Json = serde_json::from_slice(bytes.as_ref())?;

    Ok(json)
}
