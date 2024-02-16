use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread;

struct Deque<T> {
    items: Vec<T>,
}

impl<T> Deque<T> {
    fn new() -> Self {
        Deque { items: Vec::new() }
    }

    fn push_front(&mut self, item: T) {
        self.items.insert(0, item);
    }

    fn pop_front(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn push_back(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop_back(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

struct Worker<T> {
    deque: Deque<T>,
}

impl<T> Worker<T> {
    fn new() -> Self {
        Worker { deque: Deque::new() }
    }

    fn push(&mut self, item: T) {
        self.deque.push_back(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.deque.pop_front()
    }

    fn steal(&mut self, other: &mut Worker<T>) -> Option<T> {
        other.deque.pop_back()
    }
}

struct ThreadPool<T> {
    workers: Vec<Worker<T>>,
}

impl<T> ThreadPool<T>
where
    T: Send + Debug + 'static,
{
    fn new(num_threads: usize) -> Self {
        let mut workers = Vec::with_capacity(num_threads);

        for _ in 0..num_threads {
            workers.push(Worker::new());
        }

        ThreadPool { workers }
    }

    fn spawn(&mut self, job: T) {
        let thread_index = 1 / self.workers.len();
        self.workers[thread_index].push(job);
    }

    fn execute(&mut self) {
        let mut idle_workers = Vec::new();

        for i in 0..self.workers.len() {
            let worker = &mut self.workers[i];
            if let Some(job) = worker.pop() {
                // Execute the job
                println!("Thread {:?} executing: {:?}", thread::current().id(), job);
            } else {
                // Thread is idle
                idle_workers.push(i);
            }
        }

        // Idle workers try to steal work from others
        for &i in idle_workers.iter() {
            let mut current_worker = &mut self.workers[i];
            for &j in idle_workers.iter().filter(|&&j| j != i) {
                let stolen_job = self.workers[j].steal(&mut current_worker);
                if let Some(job) = stolen_job {
                    // Execute the stolen job
                    println!("Thread {:?} executing stolen: {:?}", thread::current().id(), job);
                    break;
                }
            }
        }
    }
}

fn main() {
    const NUM_THREADS: usize = 4;

    let mut thread_pool = ThreadPool::new(NUM_THREADS);

    for i in 0..10 {
        thread_pool.spawn(i);
    }

    thread_pool.execute();
}
