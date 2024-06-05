#[cfg(test)]
mod tests {
    use rand::Rng;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct A {
        pub a: i32,
        pub b: i64,
        pub c: f32,
        pub d: f64,
        pub e: Vec<(i32, char, String)>,
        pub f: String,
    }

    impl A {
        #[inline]
        pub fn rand() -> Self {
            let mut rng = rand::thread_rng();

            Self {
                a: rng.gen(),
                b: rng.gen(),
                c: rng.gen(),
                d: rng.gen(),
                e: (0..rng.gen_range(4..32))
                    .into_iter()
                    .map(|idx| {
                        (
                            idx,
                            rng.gen(),
                            Self::random_string(&mut rng),
                        )
                    })
                    .collect(),
                f: Self::random_string(&mut rng),
            }
        }

        #[inline]
        fn random_string(rng: &mut rand::rngs::ThreadRng) -> String {
            let length = rng.gen_range(32..128);
            let mut bytes = Vec::with_capacity(length);

            for _ in 0..length {
                bytes.push(rng.gen());
            }

            String::from_utf8_lossy(&bytes).to_string()
        }
    }

    #[test]
    fn to_vec_then_from_slice() -> anyhow::Result<()> {
        const BATCH_SIZE: usize = 128;

        for _ in 0..BATCH_SIZE {
            let origin = A::rand();

            let bytes = crate::to_vec(&origin)?;
            let parsed: A = crate::from_slice(&bytes)?;

            assert_eq!(origin, parsed);
        }

        Ok(())
    }

    #[cfg(feature = "base64")]
    #[test]
    fn to_base64_then_from_base64() -> anyhow::Result<()> {
        const BATCH_SIZE: usize = 128;

        for _ in 0..BATCH_SIZE {
            let origin = A::rand();

            let bytes = crate::to_base64(&origin)?;
            let parsed: A = crate::from_base64(&bytes)?;

            assert_eq!(origin, parsed);
        }

        Ok(())
    }

    #[cfg(feature = "bs58")]
    #[test]
    fn to_base58_then_from_base58() -> anyhow::Result<()> {
        const BATCH_SIZE: usize = 128;

        for _ in 0..BATCH_SIZE {
            let origin = A::rand();

            let bytes = crate::to_base58(&origin)?;
            let parsed: A = crate::from_base58(&bytes)?;

            assert_eq!(origin, parsed);
        }

        Ok(())
    }

    #[cfg(all(feature = "serde_json", feature = "base64"))]
    #[test]
    fn to_json_then_from_json_with_base64() -> anyhow::Result<()> {
        const BATCH_SIZE: usize = 128;

        for _ in 0..BATCH_SIZE {
            let origin = A::rand();

            let bytes = crate::to_json_base64(&origin)?.to_vec()?;
            let parsed: A = crate::from_json_slice(&bytes)?.to_value()?;

            assert_eq!(origin, parsed);
        }

        for _ in 0..BATCH_SIZE {
            let origin = A::rand();

            let bytes = crate::to_json(&origin)?.to_vec()?;
            let parsed: A = crate::from_json_slice(&bytes)?.to_value()?;

            assert_eq!(origin, parsed);
        }

        Ok(())
    }

    #[cfg(all(feature = "serde_json", feature = "bs58"))]
    #[test]
    fn to_json_then_from_json_with_base58() -> anyhow::Result<()> {
        const BATCH_SIZE: usize = 128;

        for _ in 0..BATCH_SIZE {
            let origin = A::rand();

            let bytes = crate::to_json_base58(&origin)?.to_vec()?;
            let parsed: A = crate::from_json_slice(&bytes)?.to_value()?;

            assert_eq!(origin, parsed);
        }

        Ok(())
    }
}
