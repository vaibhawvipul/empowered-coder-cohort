// Why?
// 1. You will understan how mutex work internally
// 2. Where exactly the tools like compare_exchange or Atomics can be used to improve this.
use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::UnsafeCell;

const LOCKED:bool = true;
const UNLOCKED:bool = false;

struct Mutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T> // the unsafe data which multiple threads need to modify
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            data: UnsafeCell::new(t),
        }
    }

    fn tec_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // the whole load and store for locks can be an compare_exchange operation
        while self.locked.load(Ordering::Acquire) != UNLOCKED {
            // keep spinning until the lock is released
        }
        // something can happen here -> a new thread can be yielded here.
        self.locked.store(LOCKED, Ordering::Release); //acquire lock
        let res = f(unsafe { &mut *self.data.get() });
        self.locked.store(UNLOCKED, Ordering::Relaxed); //release lock
        res
    }
}

fn main() {
    println!("Hello, world! This is a Mutex implementation");
    let mutex: &'static _ = Box::leak(Box::new(Mutex::new(0)));

    let handles = (0..2).map(|_| {
        std::thread::spawn(move || {
            for _ in 0..100 {
                mutex.tec_lock(|data| {
                    *data += 1;
                });
            }
        })
    }).collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap(); // wait for the threads to finish
    }

    assert!(mutex.tec_lock(|data| *data) == 2*100);
}

mod tests {
    use std::{sync::atomic::AtomicUsize, thread::spawn};

    use  super::*;

    #[test]
    fn test_of_relaxation() {
        let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
        let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

        let t1 = spawn(move ||{
            let r1 = y.load(Ordering::Acquire);
            x.store(r1, Ordering::Release);
            r1
        });

        let t2 = spawn(move ||{
            let r2 = x.load(Ordering::Relaxed);
            y.store(42, Ordering::Relaxed);
            r2
        });

        // output for x = 0, 42
        // output for y = 0, 0, 42

        // MO(X) 0 42
        // MO(Y) 0 42

        // Causal relations or happens before relation '->'

        let r1 = t1.join().unwrap();
        let r2 = t2.join().unwrap();
    }
}
