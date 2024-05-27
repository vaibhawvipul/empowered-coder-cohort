use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};

// Function using a blocking counter with locks
// not wait-free, we are using locks / sync mechanism at the software
fn blocking_counter() {
    let counter = Arc::new(Mutex::new(0)); // mutex = we want share it across multiple threads, ARC = atomic reference count. 

    // in rust, can you have a mutex without an arc? // rusts claim of fearless concurrency, !race condition issues
    // what is the difference between an RC and ARC?

    // ARC is whenever you will call it , it will trigger cache coherence.
    let mut handles = vec![];

    let start_time = Instant::now();

    for _ in 0..2000 { 
        let counter = Arc::clone(&counter); // data parallelism
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            // some computation or thread's work
            for _ in 0..100_000 { // each thread will increment the counter 100000 times
                let mut data = counter.lock().unwrap(); // lock and contention
                *data += 1;
            }
        });
        handles.push(handle);
    }


    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start_time.elapsed();
    println!("Blocking Counter Execution Time: {:?}", elapsed);
}

// Function using the non-blocking counter
fn non_blocking_counter_ordering_strong() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    let start_time = Instant::now();
    for _ in 0..2000 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100_000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start_time.elapsed();
    println!("Non-blocking Counter SeqCst Execution Time: {:?}", elapsed);
}

// Function using the non-blocking counter
fn non_blocking_counter() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    let start_time = Instant::now();
    for _ in 0..2000 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100_000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start_time.elapsed();
    println!("Non-blocking Counter Relaxed Execution Time: {:?}", elapsed);
}

fn main() {
    blocking_counter(); // check the execution time where 1000 thread, each thread increment the counter 100000 times
    non_blocking_counter_ordering_strong();
    //non_blocking_counter();
}
