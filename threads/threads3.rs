// threads3.rs
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a hint.

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc); //Cloning 2 sending ends
    let qc2 = Arc::clone(&qc); //Cloning 2 sending ends
    let tx1 = tx.clone(); //tx can be used once only so it needs to be clone for each use

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx.send(*val).unwrap(); //sending val
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap(); //sending val
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel(); //tx(transmitter) sending end, rx(receiver) receiving end
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    //unwarpping data in receiving end
    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
