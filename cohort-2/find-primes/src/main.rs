use std::thread;

fn find_primes(end:u64) {
    let mut primes = Vec::new();
    for num in 2..end {
        let mut is_prime = true;
        for div in 2..num {
            if num % div == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(num);
        }
    }
    println!("Found {} primes", primes.len());
}

fn run_in_batches() {
    let mut handles = Vec::new();
    for _ in 0..10 {
        let handle = thread::spawn(|| {
            find_primes(100000);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    println!("Hello, world!");
    run_in_batches();
}
