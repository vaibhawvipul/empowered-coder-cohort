// Write a code which demonstrates sender and recv function of mpsc

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // mpsc stands for multiple producer single consumer
    // tx stands for transmitter
    // rx stands for receiver
    let (tx, rx) = mpsc::channel();

    // clone the transmitter
    let tx1 = mpsc::Sender::clone(&tx);

    // spawn a thread
    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // send the value to the transmitter
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // spawn another thread
    thread::spawn(move || {
        let vals = vec![
            String::from("More"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            // send the value to the transmitter
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // receive the values from the transmitter
    for received in rx {
        println!("Got: {}", received);
    }
}