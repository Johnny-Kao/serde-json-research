use std::hint::black_box;
use std::time::Instant;

fn measure<T: serde::de::DeserializeOwned>(case: &str, source: &str) {
    const N: usize = 2_000_000;
    for _ in 0..100_000 {
        black_box(serde_json::from_str::<T>(black_box(source)).unwrap());
    }
    let start = Instant::now();
    for _ in 0..N {
        black_box(serde_json::from_str::<T>(black_box(source)).unwrap());
    }
    println!("{case},ns_per_op={:.3}", start.elapsed().as_nanos() as f64 / N as f64);
}

fn main() {
    measure::<i128>("i128_123", "123");
    measure::<i128>("i128_0", "0");
    measure::<i128>("i128_max", "170141183460469231731687303715884105727");
    measure::<u64>("control_u64", "123");
}
