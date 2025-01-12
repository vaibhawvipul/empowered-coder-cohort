//! Educational Coroutine Implementation
//! This implementation demonstrates key concepts of coroutines:
//! 1. Stack Management
//! 2. Context Switching
//! 3. Basic Scheduling
//! 4. State Management
//! 
//! Note: This is a simplified implementation for educational purposes.
//! Production implementations would need additional safety checks and optimizations.

use std::ptr;
use std::cell::UnsafeCell;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use std::marker::PhantomData; // for generator implementation

/// Size of each coroutine's stack
/// In production, this might be configurable or growable
const STACK_SIZE: usize = 1024 * 1024 * 2; // 2MB

/// Represents all possible states a coroutine can be in
/// This is crucial for understanding coroutine lifecycle
#[derive(Debug, Clone)]
pub enum CoroutineState {
    Ready,      // Initial state, ready to run
    Running,    // Currently executing
    Suspended,  // Temporarily paused
    Complete,   // Finished execution
}

/// CPU context that needs to be saved/restored during context switches
/// This is architecture-specific (x86_64 in this case)
#[repr(C)]
struct Context {
    // Register state
    rsp: u64,    // Stack pointer - crucial for resuming execution
    r15: u64,    // Callee-saved registers that must be preserved
    r14: u64,    // across function calls according to ABI
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,    // Frame pointer
}

impl Context {
    fn new() -> Self {
        Context {
            rsp: 0, r15: 0, r14: 0, r13: 0, r12: 0, rbx: 0, rbp: 0
        }
    }
}

/// Manages the actual memory used for coroutine execution
/// Demonstrates stack allocation and safety considerations
struct Stack {
    base: *mut u8,  // Base pointer of our allocated stack
    size: usize,    // Total size of the stack
}

impl Stack {
    /// Allocates a new stack with proper alignment
    /// Important teaching points:
    /// - Stack alignment requirements
    /// - Memory allocation safety
    /// - Resource management
    fn new(size: usize) -> Self {
        // Ensure 16-byte alignment for x86_64 ABI
        let layout = std::alloc::Layout::from_size_align(size, 16)
            .expect("Invalid stack layout");
        
        // Allocate the actual memory
        let base = unsafe { std::alloc::alloc(layout) };
        
        if base.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        Stack { base, size }
    }
}

// Proper cleanup is crucial for safety
impl Drop for Stack {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::from_size_align(self.size, 16)
            .expect("Invalid stack layout");
        unsafe {
            std::alloc::dealloc(self.base, layout);
        }
    }
}

/// The core coroutine structure
/// Type parameter F represents the function/closure to be executed
pub struct Coroutine<F> {
    stack: Stack,
    context: Context,
    state: CoroutineState,
    func: Option<F>,
}

impl<F: FnOnce()> Coroutine<F> {
    /// Creates a new coroutine from a function
    pub fn new(func: F) -> Self {
        let stack = Stack::new(STACK_SIZE);
        let mut coro = Coroutine {
            stack,
            context: Context::new(),
            state: CoroutineState::Ready,
            func: Some(func),
        };
        
        // Set up the initial stack state
        coro.initialize_stack();
        coro
    }

    /// Prepares the stack for first execution
    /// Teaching points:
    /// - Stack growth direction
    /// - Alignment requirements
    /// - Initial stack frame setup
    fn initialize_stack(&mut self) {
        // Calculate the top of the stack (grows downward on x86_64)
        let sp = self.stack.base as usize + self.stack.size;
        
        // Ensure proper stack alignment (16 bytes for x86_64)
        let sp = sp & !15;
        
        self.context.rsp = sp as u64;
    }
}

/// Scheduler for managing multiple coroutines
/// Demonstrates:
/// - Basic scheduling concepts
/// - Queue-based management
/// - Simple round-robin scheduling
pub struct Scheduler {
    ready_queue: VecDeque<Box<dyn AnyCoroutine>>,
    current: Option<Box<dyn AnyCoroutine>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            ready_queue: VecDeque::new(),
            current: None,
        }
    }

    /// Adds a new coroutine to the scheduler
    pub fn spawn<F: FnOnce() + 'static>(&mut self, func: F) {
        let coro = Box::new(Coroutine::new(func));
        self.ready_queue.push_back(coro);
    }

    /// Main scheduling loop
    /// Teaching points:
    /// - Scheduling algorithms
    /// - Coroutine state transitions
    /// - Queue management
    pub fn run(&mut self) {
        while let Some(mut coro) = self.ready_queue.pop_front() {
            self.current = Some(coro);
            
            let mut current = self.current.take();
            if let Some(ref mut coro) = current {
                // This is where actual context switch happens
                unsafe {
                    self.context_switch(&mut **coro);
                }
            }
            self.current = current;
            
            // Handle coroutine after execution
            if let Some(coro) = self.current.take() {
                match coro.state() {
                    CoroutineState::Suspended => {
                        // Coroutine yielded, put it back in queue
                        self.ready_queue.push_back(coro);
                    }
                    CoroutineState::Complete => {
                        // Coroutine finished, let it drop
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    /// Performs the actual context switch
    /// In a real implementation, this would be assembly code
    unsafe fn context_switch(&mut self, coro: &mut dyn AnyCoroutine) {
        coro.set_state(CoroutineState::Running);
        
        // Simulate execution for educational purposes
        if let Some(f) = coro.take_func() {
            f();
            coro.set_state(CoroutineState::Complete);
        }
    }
}

/// Trait for type erasure of coroutines
/// Allows storing different types of coroutines in the scheduler
trait AnyCoroutine {
    fn state(&self) -> CoroutineState;
    fn set_state(&mut self, state: CoroutineState);
    fn take_func(&mut self) -> Option<Box<dyn FnOnce()>>;
}

impl<F: FnOnce() + 'static> AnyCoroutine for Coroutine<F> {
    fn state(&self) -> CoroutineState {
        self.state.clone()
    }

    fn set_state(&mut self, state: CoroutineState) {
        self.state = state;
    }

    fn take_func(&mut self) -> Option<Box<dyn FnOnce()>> {
        self.func.take().map(|f| Box::new(f) as Box<dyn FnOnce()>)
    }
}

// Educational Generator Implementation built on Coroutines
// This implementation shows how generators are a specialized form of coroutines
// that yield values back to their caller.
//
// Key concepts demonstrated:
// 1. Generator State Management
// 2. Value Yielding Mechanism
// 3. Iterator Pattern Integration
// 4. Suspension Points
// 5. Resume with Value

/// Represents the internal state of a generator
/// This extends the coroutine state with generator-specific states
#[derive(Debug, Clone)]
pub enum GeneratorState {
    Ready,          // Initial state, ready to start
    Yielded,        // Suspended after yielding a value
    Running,        // Currently executing
    Complete,       // Finished generating values
}

/// Generator context that includes value management
/// Extends the coroutine context to handle yielded values
#[repr(C)]
struct GeneratorContext<T> {
    // Inherit coroutine context structure
    rsp: u64,       // Stack pointer
    r15: u64,       // Callee-saved registers
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,       // Frame pointer
    
    // Generator-specific fields
    yielded_value: Option<T>,  // Storage for yielded values
}

impl<T> GeneratorContext<T> {
    /// Creates a new generator context
    fn new() -> Self {
        GeneratorContext {
            // Initialize coroutine context fields
            rsp: 0, r15: 0, r14: 0, r13: 0, r12: 0, rbx: 0, rbp: 0,
            // Initialize generator-specific fields
            yielded_value: None,
        }
    }
}

/// The main generator structure
/// Type parameters:
/// - T: Type of values yielded by the generator
/// - F: The generator function type
pub struct Generator<T, F> {
    stack: Stack,                    // Reuse coroutine stack management
    context: GeneratorContext<T>,    // Extended context for generators
    state: GeneratorState,           // Generator-specific state
    func: Option<F>,                 // The generator function
    _marker: PhantomData<T>,         // Type marker for yielded values
}

/// Trait representing generator functions
/// This trait defines how generator functions interact with the generator infrastructure
pub trait GeneratorFunc<T> {
    /// Executes the generator function until the next yield point or completion
    /// Returns Some(T) if a value was yielded, None if generator is complete
    fn resume(&mut self) -> Option<T>;
}

impl<T, F> Generator<T, F>
where
    F: FnMut() -> Option<T>,
{
    /// Creates a new generator from a function
    pub fn new(func: F) -> Self {
        let stack = Stack::new(STACK_SIZE);  // Reuse coroutine stack allocation
        
        Generator {
            stack,
            context: GeneratorContext::new(),
            state: GeneratorState::Ready,
            func: Some(func),
            _marker: PhantomData,
        }
    }

    /// Advances the generator to produce the next value
    /// This is the main method for interacting with the generator
    pub fn next(&mut self) -> Option<T> {
        match self.state {
            GeneratorState::Complete => None,
            _ => {
                // Set state to running
                self.state = GeneratorState::Running;
                
                // Execute generator function until next yield point
                let result = if let Some(ref mut f) = self.func {
                    f()
                } else {
                    None
                };
                
                // Update state based on result
                match result {
                    Some(value) => {
                        self.state = GeneratorState::Yielded;
                        Some(value)
                    }
                    None => {
                        self.state = GeneratorState::Complete;
                        None
                    }
                }
            }
        }
    }
}

/// Implement Iterator for Generator
/// This allows generators to be used in for loops and with Iterator methods
impl<T, F> Iterator for Generator<T, F>
where
    F: FnMut() -> Option<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

/// Example generator creation helper
/// Makes it easier to create common types of generators
pub fn create_range_generator(start: i32, end: i32) -> Generator<i32, impl FnMut() -> Option<i32>> {
    let mut current = start;
    
    Generator::new(move || {
        if current < end {
            let value = current;
            current += 1;
            Some(value)
        } else {
            None
        }
    })
}


/// Educational demonstrations showing various coroutine concepts
pub mod demos {
    use super::*;
    use std::sync::Mutex;
    use std::time::Duration;
    use std::thread;

    /// Demonstrates basic coroutine creation and execution
    pub fn demo_basic_usage() {
        println!("Demo: Basic Coroutine Usage");
        
        let mut scheduler = Scheduler::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        
        scheduler.spawn(move || {
            println!("Coroutine executing...");
            counter_clone.fetch_add(1, Ordering::SeqCst);
            println!("Coroutine complete!");
        });
        
        scheduler.run();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    /// Shows multiple coroutines interacting
    pub fn demo_multiple_coroutines() {
        println!("Demo: Multiple Coroutines");
        
        let mut scheduler = Scheduler::new();
        let shared_data = Arc::new(AtomicUsize::new(0));
        
        // Spawn several coroutines that increment the counter
        for id in 0..3 {
            let data = shared_data.clone();
            scheduler.spawn(move || {
                println!("Coroutine {} starting", id);
                data.fetch_add(1, Ordering::SeqCst);
                // Simulate some work
                thread::sleep(Duration::from_millis(100));
                println!("Coroutine {} complete", id);
            });
        }
        
        scheduler.run();
        assert_eq!(shared_data.load(Ordering::SeqCst), 3);
    }

    /// Demonstrates stack usage patterns
    pub fn demo_stack_usage() {
        println!("Demo: Stack Usage Patterns");
        
        let mut scheduler = Scheduler::new();
        
        scheduler.spawn(|| {
            // Recursive function to demonstrate stack growth
            fn recursive(depth: usize) -> usize {
                if depth == 0 {
                    return 0;
                }
                let buffer = [0u8; 1024]; // Local array to use stack space
                recursive(depth - 1) + buffer[0] as usize
            }
            
            println!("Starting deep recursion...");
            recursive(10);
            println!("Recursion complete!");
        });
        
        scheduler.run();
    }

    /// Demonstrates memory and resource management patterns
    pub fn demo_resource_management() {
        println!("Demo: Resource Management and RAII in Coroutines");
        
        // Simulate a resource that needs cleanup
        struct ManagedResource {
            id: usize,
        }
        
        impl ManagedResource {
            fn new(id: usize) -> Self {
                println!("Resource {} allocated", id);
                ManagedResource { id }
            }
        }
        
        impl Drop for ManagedResource {
            fn drop(&mut self) {
                println!("Resource {} cleaned up", self.id);
            }
        }
        
        let mut scheduler = Scheduler::new();
        scheduler.spawn(|| {
            // Create resources with different lifetimes
            let _resource1 = ManagedResource::new(1);
            {
                let _resource2 = ManagedResource::new(2);
                // resource2 will be cleaned up here
            }
            // resource1 will be cleaned up when coroutine ends
        });
        
        scheduler.run();
    }

    /// Demonstrates suspendable operations and resumption
    pub fn demo_suspension_points() {
        println!("Demo: Suspension Points and Resumption");
        
        struct SuspendableOperation {
            steps: Vec<usize>,
            current_step: usize,
        }
        
        impl SuspendableOperation {
            fn new(steps: Vec<usize>) -> Self {
                SuspendableOperation {
                    steps,
                    current_step: 0,
                }
            }
            
            fn resume(&mut self) -> Option<usize> {
                if self.current_step < self.steps.len() {
                    let step = self.steps[self.current_step];
                    self.current_step += 1;
                    Some(step)
                } else {
                    None
                }
            }
        }
        
        let mut scheduler = Scheduler::new();
        let operation = Arc::new(Mutex::new(SuspendableOperation::new(vec![1, 2, 3, 4, 5])));
        let op_clone = operation.clone();
        
        scheduler.spawn(move || {
            while let Some(step) = op_clone.lock().unwrap().resume() {
                println!("Executing step {}", step);
                thread::sleep(Duration::from_millis(100));
                // This would be a yield point in a real implementation
            }
        });
        
        scheduler.run();
    }

    /// Demonstrates basic generator usage
    pub fn demo_basic_generator() {
        println!("Demo: Basic Generator Usage");
        
        // Create a simple range generator
        let mut gen = create_range_generator(0, 5);
        
        // Manual iteration
        while let Some(value) = gen.next() {
            println!("Generated value: {}", value);
        }
    }

    /// Demonstrates using generators with the Iterator trait
    pub fn demo_generator_iterator() {
        println!("Demo: Generator as Iterator");
        
        let gen = create_range_generator(0, 5);
        
        // Use iterator methods
        let sum: i32 = gen.sum();
        println!("Sum of generated values: {}", sum);
    }

    /// Demonstrates a more complex generator with state
    pub fn demo_stateful_generator() {
        println!("Demo: Stateful Generator");
        
        // Create a Fibonacci generator
        let mut prev = 0;
        let mut curr = 1;
        
        let mut fib = Generator::new(move || {
            let next = prev + curr;
            prev = curr;
            curr = next;
            Some(next)
        });
        
        // Generate first 10 Fibonacci numbers
        for _ in 0..10 {
            if let Some(value) = fib.next() {
                println!("Fibonacci number: {}", value);
            }
        }
    }
}

// Example usage
fn main() {
    println!("Running coroutine demonstrations...\n");
    
    demos::demo_basic_usage();
    println!();
    
    demos::demo_multiple_coroutines();
    println!();
    
    demos::demo_stack_usage();
    println!();

    demos::demo_resource_management();
    println!();

    demos::demo_suspension_points();
    println!();

    demos::demo_basic_generator();
    println!();
    
    demos::demo_generator_iterator();
    println!();
    
    demos::demo_stateful_generator();
    println!();

    println!("\nAll demonstrations complete!");
}