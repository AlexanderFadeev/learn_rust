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

fn fib_linear(n : usize) -> u64 {
    let (mut a, mut b) = (0_u64, 1_u64);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }

    return b;
}

fn bench_fib(f: fn(usize) -> u64, impl_name: &str) {
    println!("Benchmarking {}", impl_name);
    let start = Instant::now();
    let mut iters: u64 = 0;
    loop {
        let mut i = 0_usize;
        loop {
            if i > MAX_FIB_INDEX {
                break;
            }
            let now = Instant::now();
            if (now - start) > Duration::from_secs(1) {
                break;
            }
            
            f(i);
            i += 1;
            iters += 1;
        }
        
        let now = Instant::now();
        if (now - start) > Duration::from_secs(1) {
            break;
        }
    }
    let finish = Instant::now();
    let elapsed = finish - start;
    println!("Elapsed {:?}, {} iterations", elapsed, iters);  
}

fn main() {
    bench_fib(fib_recursive, "fib_recursive");
    bench_fib(fib_dynamic, "fib_dynamic");
    bench_fib(fib_linear, "fib_linear");
}
