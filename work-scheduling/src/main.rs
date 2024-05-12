// how not to do it
use rayon::join;

fn fib_recursive(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }

    let (fib_n_minus_1, fib_n_minus_2) = join(
        || fib_recursive(n - 1),
        || fib_recursive(n - 2),
    );

    fib_n_minus_1 + fib_n_minus_2
}

fn fib_rec(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }
    let fib_n_minus_1 = fib_rec(n - 1);
    let fib_n_minus_2 = fib_rec(n - 2);    
    fib_n_minus_1 + fib_n_minus_2
}

fn main() {
    const FIBONACCI_N: usize = 45;

    let start = std::time::Instant::now();
    let result = fib_rec(FIBONACCI_N);
    println!("Fibonacci Recursive({}) = {}", FIBONACCI_N, result);
    println!("Elapsed time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let result = fib_recursive(FIBONACCI_N);
    println!("Fibonacci Parallel ({}) = {}", FIBONACCI_N, result);
    println!("Elapsed time: {:?}", start.elapsed());
}
