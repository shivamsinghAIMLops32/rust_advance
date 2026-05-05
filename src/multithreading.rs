use std::thread;
use std::sync::mpsc;
use std::time::Duration;
pub fn multithreading_example(){
  let handle = thread::spawn(|| {
      for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
      }
  });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}


// moving ownership to the thread so it wont go out of scope before the thread is done with it
pub fn multithreading_example_move(){
    let v = vec![2,4,6,8];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}


// intensive work using multiple threads and mpsc
pub fn multithreading_example_mpsc() {
    let (tx, rx) = mpsc::channel();
    println!("starting main thread");
    
    // FIX 1: Removed `let ans = `. 
    // A `for` loop doesn't return a value in Rust (it returns `()`), 
    // so we don't need to assign it to a variable.
    for i in 0..4 {
        let thread_tx = tx.clone();
        
        thread::spawn(move || {
            thread_tx.send(i).unwrap();
            println!("sent {}", i);
        });
    }
    
    drop(tx);
    
    // FIX 2: Fixed the receiver loop.
    for received_data in rx {
        // You had `let received = recieved_data..unwrap();`
        // When you use a `for` loop on a receiver (`rx`), Rust is smart! 
        // It automatically unwraps the message for you. You don't need 
        // to call `.unwrap()` at all. `received_data` is exactly the `i` you sent.
        println!("received {}", received_data);
    }
}
