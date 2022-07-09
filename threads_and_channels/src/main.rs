// RUN WITH: RUST_LOG=info cargo run
// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam::channel;
use std::thread;
use std::time::Duration;
use log::{info,error};
fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn sleep(seconds: f32) {
    thread::sleep(Duration::from_secs_f32(seconds));
}

pub mod work_1 {
    use super::{info,sleep};
    pub fn expensive_sum(v: Vec<i32>) -> i32 {
        sleep(1.0);
        info!("Child thread: just about finished");
        v.iter().filter(|&x| x % 2 == 0).map(|x| x * x).sum()
    }
}

fn main() {
    // init logger
    env_logger::init();

    let my_vector = vec![2, 5, 1, 0, 4, 3];


    let handle = thread::spawn(move || { // child thread
        work_1::expensive_sum(my_vector)
    });

    // While the child thread is running, the main thread will also do some work
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        info!("Main thread: Processing the letter '{}'", letter);
        sleep_ms(200);
    }

    ////////////
    let result = handle.join();
    let sum = result.unwrap();
    info!("The child thread's expensive sum is {}", sum);

    
    let (tx,rx) = channel::unbounded();
    // Cloning a channel makes another variable connected to that end of the channel so that you can
    // send it to another thread. We want another variable that can be used for sending...
    let tx2 = tx.clone();
   
    // Thread A
    let handle_a = thread::spawn(move || {
        sleep_ms(900);
        tx2.send("Thread A: 1").unwrap();
        sleep_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });
    sleep_ms(100);
    // Thread B
    let handle_b = thread::spawn(move || {
        sleep_ms(0);
        tx.send("Thread B: 1").unwrap();
        sleep_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    // Using a Receiver channel as an iterator is a convenient way to get values until the channel
    // gets closed.  A Receiver channel is automatically closed once all Sender channels have been
    // closed.  Both our threads automatically close their Sender channels when they exit and the
    // destructors for the channels get automatically called.
    for msg in rx {
        info!("Main thread: Received {}", msg);
    }
    
    //joining
    handle_a.join().unwrap();
    let _not_important = handle_b.join();



    ////////////////
    let (txa,rxa) = channel::unbounded::<i32>();
    let rxb = rxa.clone();
    let rxc =  rxa.clone();
    let handle_c = thread::spawn(move || {
        // let n: Vec<_> = rxb.try_iter().collect();
        // let n = rxb.recv().unwrap();
        for msg in rxa {
            //let n =  rxb.recv_timeout(Duration::from_millis(10000));
            info!("Child 1 receiving {:?}", msg);
        }
    });

    let handle_d =  thread::spawn(move || {
        for msg in rxc {
            //let n =  rxb.recv_timeout(Duration::from_millis(10000));
            info!("Child 2 receiving {:?}", msg);
        }
    });

    for num in vec![1,3,4,5,6,89]  {
        sleep_ms(500);
        // println!("sending {}", &num);
        txa.send(num).unwrap();
    }

    drop(txa);
    let _r = handle_c.join().unwrap();
    handle_d.join().unwrap();
    info!("Main thread: Exiting.")
}