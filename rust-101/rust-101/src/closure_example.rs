// Define Closures in rust 
// Closures are anonymous functions that can be saved in a variable or passed as arguments to other functions.

pub fn closure() {
    // Example 1: Basic Closure
    let add = |x, y| x + y; // This closure takes two parameters and returns their sum.
    let result = add(3, 5);
    println!("Example 1: Sum: {}", result);

    // Example 2: Closure with Capture
    let base = 10;
    let add_with_base = |x| x + base; // This closure captures the 'base' variable.
    let result_with_base = add_with_base(5);
    println!("Example 2: Sum with Base: {}", result_with_base);

    // Example 3: Closure with Mutable Capture
    let mut counter = 0;
    let increment = || {
        counter += 1;
        counter
    }; // This closure captures and mutably modifies the 'counter' variable.
    let count1 = increment();
    let count2 = increment();
    println!("Example 3: Counter: {}, {}", count1, count2);

    // Example 4: Closure as an Argument
    let numbers = vec![1, 2, 3, 4, 5];
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Example 4: Sum of Numbers: {}", sum);

    // Example 5: Closure with Move Semantics
    let message = String::from("Hello");
    let printer = move || println!("{}", message); // The closure takes ownership of 'message'.
    // Uncommenting the line below would result in a compile error because 'message' has been moved.
    // println!("Example 5: {}", message);
    printer();

    // Example 6: Returning a Closure
    let multiplier = multiplier_factory(3);
    let result = multiplier(4);
    println!("Example 6: Multiplier Result: {}", result);
}

// Function returning a closure
fn multiplier_factory(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}
