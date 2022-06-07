use std::time::{Duration, Instant};

const MAX_FIB_INDEX : u8 = 93;

fn fib_recursive(n : u8) -> u64 {
    if n <= 1 {
        return 1
    }
    fib_recursive(n - 1) + fib_recursive(n - 2)
}

fn bench_fib(f: fn(u8) -> u64, impl_name: &str) {
    println!("Benchmarking {}", impl_name);
    let start = Instant::now();
    let mut i = 0_u8;
    let mut x : u64 = 0;
    loop {
        if i > MAX_FIB_INDEX {
            break;
        }
        let now = Instant::now();
        if (now - start) > Duration::from_secs(1) {
            break;
        }
        
        x = f(i);
        i += 1;
    }
    let finish = Instant::now();
    let elapsed = finish - start;
    println!("Elapsed {:?}, {} iterations, {} max value", elapsed, i, x);  
}

fn main() {
    bench_fib(fib_recursive, "fib_recursive");
}
