use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

static ALLOCS: AtomicU64 = AtomicU64::new(0);
static BYTES: AtomicU64 = AtomicU64::new(0);

struct Counting;
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
    unsafe fn realloc(&self, ptr: *mut u8, old: Layout, new: usize) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(new as u64, Ordering::Relaxed);
        System.realloc(ptr, old, new)
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

fn bench<T: serde::de::DeserializeOwned>(label: &str, source: &str) {
    const ITERS: u64 = 300_000;
    for _ in 0..2000 {
        black_box(serde_json::from_str::<T>(black_box(source)).unwrap());
    }
    ALLOCS.store(0, Ordering::Relaxed);
    BYTES.store(0, Ordering::Relaxed);
    let start = Instant::now();
    for _ in 0..ITERS {
        black_box(serde_json::from_str::<T>(black_box(source)).unwrap());
    }
    let elapsed = start.elapsed();
    println!("{label},ns_per_op={:.2},alloc_per_op={:.4},bytes_per_op={:.2}",
        elapsed.as_nanos() as f64 / ITERS as f64,
        ALLOCS.load(Ordering::Relaxed) as f64 / ITERS as f64,
        BYTES.load(Ordering::Relaxed) as f64 / ITERS as f64);
}

fn main() {
    bench::<i128>("i128_0", "0");
    bench::<i128>("i128_123", "123");
    bench::<i128>("i128_i64max", "9223372036854775807");
    bench::<i128>("i128_max", "170141183460469231731687303715884105727");
    bench::<i128>("i128_min", "-170141183460469231731687303715884105728");
    bench::<u128>("u128_0", "0");
    bench::<u128>("u128_123", "123");
    bench::<u128>("u128_u64max", "18446744073709551615");
    bench::<u128>("u128_max", "340282366920938463463374607431768211455");
}
