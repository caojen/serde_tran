#[cfg(test)]
mod tests {
    use std::time;
    use rand::{Rng, thread_rng};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct A {
        pub a: i32,
        pub b: i64,
        pub c: f32,
        pub d: f64,
        pub e1: Vec<u8>,
        pub e2: String,
        pub e3: Vec<u128>,
        pub long_key_11111111111111111111111111: String,
        pub long_key_2222222222222222222222222222222: String,
        pub long_key_333333333333: String,
        pub long_key_4x4x4x4x4x4x4x4x4x4x44x4x44x4x444x4x: String,
    }

    impl A {
        fn random_string(rng: &mut rand::rngs::ThreadRng, length: usize) -> String {
            let mut bytes = Vec::with_capacity(length);

            for _ in 0..length {
                bytes.push(rng.gen());
            }

            String::from_utf8_lossy(&bytes).to_string()
        }

        // n: how many [A]
        // string_len: the length of string that stored in [A]
        pub fn create_batches(n: usize, string_len: usize) -> Vec<Self> {
            let mut rng = thread_rng();

            let data = (0..n).into_iter()
                .map(|_| {
                    A {
                        a: rng.gen(),
                        b: rng.gen(),
                        c: rng.gen(),
                        d: rng.gen(),
                        e1: (0..string_len).into_iter().map(|_| rng.gen()).collect(),
                        e2: Self::random_string(&mut rng, string_len),
                        e3: (0..string_len).into_iter().map(|_| rng.gen()).collect(),
                        long_key_11111111111111111111111111: Self::random_string(&mut rng, string_len),
                        long_key_2222222222222222222222222222222: Self::random_string(&mut rng, string_len),
                        long_key_333333333333: Self::random_string(&mut rng, string_len),
                        long_key_4x4x4x4x4x4x4x4x4x4x44x4x44x4x444x4x: Self::random_string(&mut rng, string_len),
                    }
                })
                .collect();

            data
        }
    }

    #[inline]
    fn json_encode<T: Serialize>(a: &T) -> String {
        serde_json::to_string(a).unwrap()
    }

    #[inline]
    fn tran_encode<T: Serialize>(a: &T) -> String {
        crate::to_json(a).unwrap().to_string().unwrap()
    }

    #[test]
    fn benchmark() {
        let n = [1, 10, 100, 1024, 2048, 5120, 10240];
        let len = [10, 200, 400];

        for i in n {
            for j in len {
                benchmark_n_len(i, j);
            }
        }
    }

    fn benchmark_n_len(n: usize, string_len: usize) {
        const RUN: usize = 1;

        let mut json_time = 0u128;
        let mut tran_time = 0u128;
        let mut json_space = 0usize;
        let mut tran_space = 0usize;

        for _ in 0..RUN {
            let data = A::create_batches(n, string_len);

            let start = time::SystemTime::now();
            let size = json_encode(&data).len();
            let end = time::SystemTime::now();

            let diff = end.duration_since(start).unwrap().as_nanos();
            json_time += diff;
            json_space += size;

            let start = time::SystemTime::now();
            let size = tran_encode(&data).len();
            let end = time::SystemTime::now();

            let diff = end.duration_since(start).unwrap().as_nanos();
            tran_time += diff;
            tran_space += size;
        }

        let json_op = json_time as f64 / RUN as f64;
        let tran_op = tran_time as f64 / RUN as f64;
        let json_sp = json_space as f64 / RUN as f64;
        let tran_sp = tran_space as f64 / RUN as f64;

        println!("n={}, sl={}, \n\tjson.time={}ns/op, json.space={}bytes/bench, \n\ttran.time={}ns/op, tran.space={}bytes/bench\n\t[tran/json] time={}%, space={}%", n, string_len, json_op, json_sp, tran_op, tran_sp, (tran_op-json_op)/json_op*100.0, (tran_sp-json_sp)/json_sp *100.0);
    }
}
