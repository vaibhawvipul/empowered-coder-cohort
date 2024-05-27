use std::{sync::{Arc, Mutex}, thread};
use rayon::prelude::*;

fn sum_sequential(nums: Vec<usize>) -> usize {
    let mut sum = 0;

    for num in nums {
        sum += num;
    }

    sum
}

fn cpu_count() -> usize {
    1024
}

fn sum_parallel(nums: Vec<usize>) -> usize {
    let sum: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let chunk_size = nums.len() / cpu_count();

    for chunk in nums.chunks(chunk_size) {
        let sum_clone = Arc::clone(&sum);
        let chunk = chunk.to_vec();
        let handle = thread::spawn(move || {
            // Calculate the sum of the chunk
            let chunk_sum: usize = chunk.iter().sum();

            // Lock the mutex and update the shared sum
            let mut i_sum = sum_clone.lock().unwrap();
            *i_sum += chunk_sum;
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    let final_sum = sum.lock().unwrap();
    *final_sum
}

fn sum_parallel_rayon (nums: Vec<usize>) -> usize {
    nums.par_iter().sum()
}     


fn main() {
    println!("Hello, world!");

    let nums: Vec<usize> = (0..1000000000).collect(); //10^8

    // // time it
    // let start = std::time::Instant::now();
    // let sum_seq = sum_sequential(nums.clone());
    // let duration = start.elapsed();

    // println!("Sequential sum: {}, Duration: {} secs", sum_seq, duration.as_secs_f32());

    //sleep for 1 sec
    // std::thread::sleep(std::time::Duration::from_secs(1));

    // // time it
    // let start = std::time::Instant::now();
    // let sum_par = sum_parallel(nums.clone());
    // let duration = start.elapsed();

    // println!("Parallel sum: {}, Duration: {} secs", sum_par, duration.as_secs_f32());

    //sleep for 1 sec
    // std::thread::sleep(std::time::Duration::from_secs(1));

    // time it
    let start = std::time::Instant::now();
    let sum_par_rayon = sum_parallel_rayon(nums);
    let duration = start.elapsed();

    println!("Parallel sum rayon: {}, Duration: {} secs", sum_par_rayon, duration.as_secs_f32());

}
