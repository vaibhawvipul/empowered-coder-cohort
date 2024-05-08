use std::time::Duration;
use tokio::time::sleep;

async fn greet(name: &str) {
    println!("Hello, {}!", name);
    sleep(Duration::from_secs(1)).await;
    println!("Nice to meet you, {}!", name);
}

#[tokio::main]
async fn main() {
    greet("Alice").await;
    greet("Bob").await;
}

// use futures::stream;
// use futures::stream::StreamExt;

// #[tokio::main]
// async fn main() {
//     let stream = stream::iter(vec![1, 2, 3]);
    
//     let sum = stream.fold(0, |acc, x| async move { acc + x }).await;
    
//     println!("Sum: {}", sum);
// }

// use std::time::Duration;
// use tokio::time::sleep;
// use tokio::task;

// async fn task1() {
//     println!("Task 1 started");
//     sleep(Duration::from_secs(2)).await;
//     println!("Task 1 completed");
// }

// async fn task2() {
//     println!("Task 2 started");
//     sleep(Duration::from_secs(1)).await;
//     println!("Task 2 completed");
// }

// #[tokio::main]
// async fn main() {
//     let join_handle1 = task::spawn(task1());
//     let join_handle2 = task::spawn(task2());
    
//     join_handle1.await.unwrap();
//     join_handle2.await.unwrap();
// }
