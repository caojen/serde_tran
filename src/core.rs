use std::hash::Hasher;
use serde::{Deserialize, Serialize};
use crate::error;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataHash {
    pub data: Vec<u8>,
    pub hash: u64,
}

impl DataHash {
    #[inline]
    fn hasher() -> impl Hasher {
        std::hash::DefaultHasher::new()
    }

    pub fn new(data: Vec<u8>) -> Self {
        let hash = if cfg!(feature = "no-hash-validate") {
            0u64
        } else {
            let mut hasher = Self::hasher();
            hasher.write(&data);
            hasher.finish()
        };

        Self {
            data,
            hash,
        }
    }

    pub fn validate(&self) -> error::Result<()> {
        #[cfg(feature = "no-hash-validate")]
        { return Ok(()) }

        #[cfg(not(feature = "no-hash-validate"))]
        {
            let mut hasher = Self::hasher();
            hasher.write(&self.data);

            let expected = hasher.finish();

            if self.hash == expected {
                Ok(())
            } else {
                Err(error::ErrorKind::HashError(expected, self.hash))
            }
        }
    }
}

/// convert data to bytes
pub fn to_vec<T>(data: &T) -> error::Result<Vec<u8>>
    where T: Serialize + ?Sized,
{
    // serialize given data into bytes
    let bytes = bincode::serialize(data)?;

    // then, use DataHash to store bytes, calculate hash, and finally convert into bytes.
    let data_hash = DataHash::new(bytes);
    let bytes = bincode::serialize(&data_hash)?;

    Ok(bytes)
}

/// convert bytes into T
pub fn from_slice<T>(slice: &[u8]) -> error::Result<T>
    where T: for <'de> Deserialize<'de>
{
    // deserialize from bytes to DataHash, and do validate
    let data_hash: DataHash = bincode::deserialize(slice)?;
    data_hash.validate()?;

    // convert back to given data
    let data = bincode::deserialize(data_hash.data.as_slice())?;
    Ok(data)
}
