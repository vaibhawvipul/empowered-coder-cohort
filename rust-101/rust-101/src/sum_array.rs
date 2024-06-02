use std::sync::Arc;
use std::sync::Mutex;

pub fn sum_arrays() {
    let mut array = vec![];
    for i in 0..10000 {
        array.push(i);
    }
    let sum = Arc::new(Mutex::new(0));

    let threads: Vec<_> = array
        .into_iter()
        .map(|num| {
            let sum = Arc::clone(&sum);
            std::thread::spawn(move || {
                *sum.lock().unwrap() += num;
            })
        })
        .collect();

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Sum: {}", *sum.lock().unwrap());
}                                                          