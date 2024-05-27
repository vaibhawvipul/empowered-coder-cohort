use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn main() {
    // Create shared data structures
    let shared_data = Arc::new(Mutex::new(0));
    let condvar = Arc::new(Condvar::new());
    let semaphore = Arc::new(Mutex::new(0));

    // Number of threads
    let num_threads = 8;
    
    // Spawn multiple threads
    let mut handles = vec![];

    for i in 0..num_threads {
        let shared_data_clone = Arc::clone(&shared_data);
        let condvar_clone = Arc::clone(&condvar);
        let semaphore_clone = Arc::clone(&semaphore);

        let handle = thread::spawn(move || {
            // Simulate some work
            let work_duration = std::time::Duration::from_secs(i as u64);
            thread::sleep(work_duration);
            // Lock the mutex and access the shared data
            let mut data = shared_data_clone.lock().unwrap();
            *data += 1;

            println!("Thread {} has updated the shared data.", i);

            // Check if all threads have updated the shared data
            if *data == num_threads {
                println!("All threads have updated the shared data. Signaling others.");
                condvar_clone.notify_all();
            } else {
                println!("Thread {} is waiting for others to finish.", i);
                condvar_clone.wait(data).unwrap();
                println!("Thread {} has been notified and can proceed.", i);
            }

            // Release the semaphore to indicate completion
            let mut sem = semaphore_clone.lock().unwrap();
            *sem += 1;
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
