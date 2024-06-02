// Example 1: Generic Function
fn print_twice<T>(value: T) {
    println!("Example 1: {} {}", value, value);
}

// Example 2: Generic Struct
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Example 3: Generic Enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub fn generics_example() {
    // Example 1: Generic Function
    print_twice("Hello");
    print_twice(42);

    // Example 2: Generic Struct
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 2.5 };
    println!(
        "Example 2: Integer Point: {:?}, Float Point: {:?}",
        integer_point, float_point
    );

    // Example 3: Generic Enum
    let success: Result<i32, &str> = Result::Ok(42);
    let failure: Result<i32, &str> = Result::Err("Something went wrong");
    match success {
        Result::Ok(value) => println!("Example 3: Success: {}", value),
        Result::Err(err) => println!("Example 3: Failure: {}", err),
    }
}
