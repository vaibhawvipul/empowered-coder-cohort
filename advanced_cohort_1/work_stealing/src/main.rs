//! Work Stealing Implementation for Coroutines
//! Demonstrates advanced scheduling concepts with work stealing

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::time::Duration;
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Represents a task that can be executed by our coroutines
#[derive(Debug)]
struct Task {
    id: usize,
    priority: usize,
    work_units: usize,
    state: TaskState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TaskState {
    Ready,
    Running,
    Completed,
    Stolen,
}

/// A deque that supports both LIFO and FIFO operations
/// This is crucial for work stealing as workers use it differently:
/// - Owner uses it as a LIFO stack (push/pop from back)
/// - Thieves use it as a FIFO queue (steal from front)
struct WorkStealingDeque {
    tasks: Arc<Mutex<VecDeque<Task>>>,
    size: Arc<AtomicUsize>,
}

impl WorkStealingDeque {
    fn new() -> Self {
        WorkStealingDeque {
            tasks: Arc::new(Mutex::new(VecDeque::new())),
            size: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Push task to the back (used by owner)
    fn push(&self, task: Task) {
        let mut queue = self.tasks.lock().unwrap();
        queue.push_back(task);
        self.size.fetch_add(1, Ordering::SeqCst);
    }

    /// Pop task from the back (used by owner)
    fn pop(&self) -> Option<Task> {
        let mut queue = self.tasks.lock().unwrap();
        let task = queue.pop_back();
        if task.is_some() {
            self.size.fetch_sub(1, Ordering::SeqCst);
        }
        task
    }

    /// Steal task from the front (used by thieves)
    fn steal(&self) -> Option<Task> {
        let mut queue = self.tasks.lock().unwrap();
        let task = queue.pop_front();
        if task.is_some() {
            self.size.fetch_sub(1, Ordering::SeqCst);
        }
        task
    }

    fn size(&self) -> usize {
        self.size.load(Ordering::SeqCst)
    }
}

/// Represents a worker in our work stealing scheduler
#[derive(Clone)]
struct Worker {
    id: usize,
    local_queue: Arc<WorkStealingDeque>,
    other_queues: Vec<Arc<WorkStealingDeque>>,
    tasks_completed: Arc<AtomicUsize>,
    tasks_stolen: Arc<AtomicUsize>,
    total_system_tasks: Arc<AtomicUsize>,
}

impl Worker {
    fn new(
        id: usize,
        other_queues: Vec<Arc<WorkStealingDeque>>,
        tasks_completed: Arc<AtomicUsize>,
        tasks_stolen: Arc<AtomicUsize>,
        total_system_tasks: Arc<AtomicUsize>,
    ) -> Self {
        Worker {
            id,
            local_queue: Arc::new(WorkStealingDeque::new()),
            other_queues,
            tasks_completed,
            tasks_stolen,
            total_system_tasks,
        }
    }
    fn run(&self) {
        println!("\x1b[33mWorker {} starting with {} tasks. Number of other queues: {}\x1b[0m", 
            self.id, self.local_queue.size(), self.other_queues.len());
        
        loop {
            // First check if we should exit
            if self.all_work_complete() {
                let completed = self.tasks_completed.load(Ordering::SeqCst);
                let stolen = self.tasks_stolen.load(Ordering::SeqCst);
                println!("\x1b[33mWorker {} exiting - Completed: {}, Stolen: {}\x1b[0m", 
                    self.id, completed, stolen);
                break;
            }

            // For non-zero workers, try to steal if they have no local work
            if self.id != 0 && self.local_queue.size() == 0 {
                if let Some(mut stolen_task) = self.steal_task() {
                    println!("\x1b[32mWorker {} successfully stole task {}\x1b[0m", 
                        self.id, stolen_task.id);
                    stolen_task.state = TaskState::Stolen;
                    self.tasks_stolen.fetch_add(1, Ordering::SeqCst);
                    self.execute_task(&mut stolen_task);
                    continue;
                }
            }
    
            // Check local queue
            if let Some(mut task) = self.local_queue.pop() {
                self.execute_task(&mut task);
                continue;
            }
    
            // If we get here, we couldn't find work - sleep briefly before trying again
            thread::sleep(Duration::from_millis(50));
        }
    }

    fn steal_task(&self) -> Option<Task> {
        // Worker 0 never steals
        if self.id == 0 {
            return None;
        }
    
        // Calculate minimum tasks to consider stealing
        // Only steal if the source queue has at least this many more tasks
        let min_imbalance = 0; // Only steal if queue has 10+ more tasks than us
    
        // For all other workers, try to steal from worker 0's queue first
        if let Some(worker_0_queue) = self.other_queues.first() {
            let source_size = worker_0_queue.size();
            let our_size = self.local_queue.size();
            
            // Only steal if there's a significant imbalance
            if source_size > our_size + min_imbalance {
                let stolen = worker_0_queue.steal();
                if stolen.is_some() {
                    println!("Worker {} successfully stole from worker 0 (imbalance: {})", 
                        self.id, source_size - our_size);
                    return stolen;
                }
            }
        }
    
        // If we couldn't steal from worker 0, try other queues with same imbalance check
        for (i, queue) in self.other_queues.iter().enumerate().skip(1) {
            let source_size = queue.size();
            let our_size = self.local_queue.size();
            
            if source_size > our_size + min_imbalance {
                println!("Worker {} checking other queue {}, size: {}", self.id, i, source_size);
                let stolen = queue.steal();
                if stolen.is_some() {
                    println!("Worker {} successfully stole from queue {} (imbalance: {})", 
                        self.id, i, source_size - our_size);
                    return stolen;
                }
            }
        }
        None
    }

    fn all_work_complete(&self) -> bool {
        let completed = self.tasks_completed.load(Ordering::SeqCst);
        let stolen = self.tasks_stolen.load(Ordering::SeqCst);
        let total = self.total_system_tasks.load(Ordering::SeqCst);
        
        // Only exit if all tasks are accounted for AND there's no work in any queue
        completed + stolen >= total && !self.work_exists_in_system()
    }

    fn execute_task(&self, task: &mut Task) {
        let status = if task.state == TaskState::Stolen { 
            "\x1b[32m(stolen)\x1b[0m" 
        } else { 
            "\x1b[34m(local)\x1b[0m" 
        };
        
        println!("Worker {} executing task {} {} - work units: {} - Queue size: {}", 
            self.id, 
            task.id,
            status,
            task.work_units,
            self.local_queue.size()
        );

        thread::sleep(Duration::from_millis(task.work_units as u64));
        
        task.state = TaskState::Completed;
        self.tasks_completed.fetch_add(1, Ordering::SeqCst);
    }

    fn work_exists_in_system(&self) -> bool {
        self.other_queues.iter().any(|q| q.size() > 0)
    }
}

/// Work stealing scheduler that manages all workers
struct WorkStealingScheduler {
    workers: Vec<Worker>,
    tasks_completed: Arc<AtomicUsize>,
    total_tasks: Arc<AtomicUsize>,
    tasks_stolen: Arc<AtomicUsize>,
}

impl WorkStealingScheduler {
    fn new(num_workers: usize) -> Self {
        let tasks_completed = Arc::new(AtomicUsize::new(0));
        let tasks_stolen = Arc::new(AtomicUsize::new(0));
        let total_tasks = Arc::new(AtomicUsize::new(0));
        
        // Create all queues that will be shared between workers
        let queues: Vec<Arc<WorkStealingDeque>> = (0..num_workers)
            .map(|_| Arc::new(WorkStealingDeque::new()))
            .collect();

        println!("Created {} shared queues", queues.len());
        
        // Create workers
        let workers = (0..num_workers)
            .map(|worker_id| {
                // Each worker's local queue is a reference to their corresponding shared queue
                let local_queue = Arc::clone(&queues[worker_id]);
                
                // Other queues are all queues except their own
                let other_queues: Vec<Arc<WorkStealingDeque>> = queues
                    .iter()
                    .enumerate()
                    .filter(|&(idx, _)| idx != worker_id)
                    .map(|(_, queue)| Arc::clone(queue))
                    .collect();

                println!("Worker {} sees {} other queues", worker_id, other_queues.len());
                
                Worker {
                    id: worker_id,
                    local_queue,  // This is now a reference to the shared queue
                    other_queues,
                    tasks_completed: Arc::clone(&tasks_completed),
                    tasks_stolen: Arc::clone(&tasks_stolen),
                    total_system_tasks: Arc::clone(&total_tasks),
                }
            })
            .collect();

        WorkStealingScheduler {
            workers,
            total_tasks,
            tasks_completed,
            tasks_stolen,
        }
    }

    fn add_task(&mut self, worker_id: usize, task: Task) {
        println!("Adding task {} to worker {}'s queue", task.id, worker_id);
        self.total_tasks.fetch_add(1, Ordering::SeqCst);
        self.workers[worker_id].local_queue.push(task);
    }

    fn run(&mut self) {
        let mut threads = Vec::new();
        for worker in &self.workers {
            let worker = worker.clone();
            threads.push(thread::spawn(move || {
                worker.run();
            }));
        }

        for thread in threads {
            thread.join().unwrap();
        }

        println!("\n\x1b[35mAll workers have completed their work\x1b[0m");
    }
}

pub fn demo_work_stealing() {
    println!("\n=== Work Stealing Demonstration ===\n");

    let mut scheduler = WorkStealingScheduler::new(4);

    // Create extremely unbalanced workload
    for i in 0..12 {  // 12 total tasks
        let task = Task {
            id: i,
            priority: i % 3,
            work_units: if i < 9 {  // Worker 0 gets 9 long tasks
                3000  // Worker 0's tasks take 3 seconds each
            } else {
                100   // Other workers get quick tasks
            },
            state: TaskState::Ready,
        };

        // Give 9 out of 12 tasks to worker 0
        if i < 9 {
            println!("\x1b[34mAdding long task {} (3s) to worker 0\x1b[0m", i);
            scheduler.add_task(0, task);
        } else {
            let worker_id = (i % 3) + 1;
            println!("\x1b[36mAdding quick task {} (0.2s) to worker {}\x1b[0m", i, worker_id);
            scheduler.add_task(worker_id, task);
        }
    }

    println!("\n\x1b[35mInitial task distribution:\x1b[0m");
    for (i, worker) in scheduler.workers.iter().enumerate() {
        println!("Worker {} has {} tasks", i, worker.local_queue.size());
    }
    println!("\n\x1b[35mStarting execution...\x1b[0m\n");

    scheduler.run();
}

// Example usage in main:
fn main() {
    demo_work_stealing();
}