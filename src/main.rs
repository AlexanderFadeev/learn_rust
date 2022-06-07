use std::time::{Duration, Instant};

const MAX_FIB_INDEX : usize = 92;

fn fib_recursive(n : usize) -> u64 {
    if n <= 1 {
        return 1
    }
    fib_recursive(n - 1) + fib_recursive(n - 2)
}

fn fib_dynamic(n : usize) -> u64 {
    if n <= 1 {
        return 1
    }

    let mut a: [u64; MAX_FIB_INDEX + 1] = [0; MAX_FIB_INDEX + 1];
    a[0] = 1;
    a[1] = 1;
    for i in 2..=n {
        a[i] = a[i-1] + a[i-2];
    }

    return a[n];
}

fn bench_fib(f: fn(usize) -> u64, impl_name: &str) {
    println!("Benchmarking {}", impl_name);
    let start = Instant::now();
    let mut i = 0_usize;
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
    bench_fib(fib_dynamic, "fib_dynamic");
}
