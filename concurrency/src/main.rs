use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    simple_demo();
    ownership_in_threads();
    channel();
    mutex();
    atomic_type();
}

fn simple_demo() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn ownership_in_threads() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn mutex() {
    use std::sync::{Arc, Mutex};
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn atomic_type() {
    use std::sync::atomic::{AtomicI32, Ordering};
    use std::sync::Arc;
    for ord in [
        Ordering::Relaxed,
        Ordering::Release,
        Ordering::Acquire,
        Ordering::AcqRel,
        Ordering::SeqCst,
    ] {
        let start_time = std::time::Instant::now();
        for _ in 0..10000 {
            let atomic_num = Arc::new(AtomicI32::new(5));
            let mut handles = vec![];

            // let clone = atomic_num;
            for i in 0..10 {
                let atomic_num = Arc::clone(&atomic_num);
                let h = thread::spawn(move || {
                    atomic_num.fetch_add(i, ord);
                });
                handles.push(h);
            }
            for h in handles {
                h.join().unwrap();
            }
        }
        let end_time = std::time::Instant::now();
        println!("Time for {:?} is {:?}", ord, end_time - start_time);
    }
}
