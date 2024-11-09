use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

const LOCKED : bool = true;
const UNLOCKED : bool = false;

struct Mutex<T> { // usually has a data field
    data: UnsafeCell<T>, // shared resource, this is not thread safe by default
    locked: AtomicBool, // by locks
}

unsafe impl<T> Sync for Mutex<T> where T: Send {} // this is a demo of what rust people claim to be fearlessly concurrency.
// Send is for ownership transfer between threads
// Sync is for shared references between threads

impl<T> Mutex<T> { // traits (behaviour) for Mutex
    fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
            locked: AtomicBool::new(UNLOCKED),
        }
    }

    fn tec_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // spin lock
        // while self.locked.load(Ordering::Acquire) == LOCKED {
        //     // keep spinning until lock is acquired

        //     // this is a busy wait loop
        //     // os will interrupt this thread and give time to other threads
        // }
        // not preemtively switch threads here.
        // 1. out of order execution acquire/release only solves out of order execution
        // 2. os can switch threads - still an open problem
        // self.locked.store(LOCKED, Ordering::Release);
        // cannot preemtively switch threads
        //  // preemtively switch threads
        // std::thread::yield_now(); // this is a hint to the os to switch threads
        loop { // still a spin lock
            // CAS - compare and swap
            if self.locked.compare_exchange(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed) == Ok(UNLOCKED) {
                let res = f(unsafe { &mut *self.data.get() });
                self.locked.store(UNLOCKED, Ordering::Relaxed);  // release lock
                return res;
            } else {
                println!("Waiting for lock to be released");
            }

            // ABA Problem - very frequent with non blocking data structure especially with CAS.
        }
    }
}

fn main() {
    println!("Hello, world! This is a mutex tutorial!");

    let mutex: &'static Mutex<i32> = Box::leak(Box::new(Mutex::new(0)));

    // sp[a]wn threads
    // acquire lock
    let thread_handles = (0..2).map(|_| {
        std::thread::spawn(move || {
            for _ in 0..10000 {
                mutex.tec_lock(|data| {
                    *data += 1;
                })
            }
        })
    }).collect::<Vec<_>>();

    // join threads
    for handle in thread_handles {
        handle.join().unwrap(); // wait for thread to finish
    }

    let data = mutex.tec_lock(|data| *data);
    println!("Mutex data: {}", data);
    assert!(data == 2*10000);
}
