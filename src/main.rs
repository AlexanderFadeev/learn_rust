fn fib(n : u8) -> u64 {
    if n <= 1 {
        return 1
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    for i in 0..10 {
        println!("{}", fib(i));   
    }
}
