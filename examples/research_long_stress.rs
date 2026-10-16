use std::hint::black_box;
use std::time::Instant;

fn main() {
    for len in [39usize, 1_000, 100_000, 1_000_000] {
        let digits = "9".repeat(len);
        for mode in ["string", "slice", "reader"] {
            let start = Instant::now();
            let result = match mode {
                "string" => serde_json::from_str::<u128>(black_box(&digits)),
                "slice" => serde_json::from_slice::<u128>(black_box(digits.as_bytes())),
                _ => serde_json::from_reader::<_, u128>(std::io::Cursor::new(black_box(digits.as_bytes()))),
            };
            println!("len={len} mode={mode} elapsed_us={} result={:?}",start.elapsed().as_micros(),result.err().map(|e| (e.classify(),e.line(),e.column())));
        }
    }
}
