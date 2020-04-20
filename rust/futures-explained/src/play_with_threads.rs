// The first time I play with Rust's threads
use std::thread;
// use std::rc::Rc;

#[allow(dead_code)]
pub fn run() {
    println!("So we start the program here, let's see what thread can do");
    // let rc1 = Rc::new(1);
    
    // The core abstraction of Rust threading system centers around 
    // thread::spawn. thread::spawn takes a FnOnce closure as input,
    // launches a new thread (via system calls I guess) and runs the 
    // the closure in that thread. 
    let thread1 = thread::spawn(move || {
        // Sleep for 1s here, pay attention to the sleep API, it is
        // not a associated function just like spawn
        thread::sleep(std::time::Duration::from_secs(1));
        println!("thread1 is waken up");

        // Try print rc in this thread
        // println!("{}", rc1);
    });

    // Then we create thread2, and let thread2 create thread3. 
    // [nested thread creation]
    let thread2 = thread::spawn(move || {        
        // Create the third thread
        let thread3 = thread::spawn(move ||{
            // Sleep for 3s
            thread::sleep(std::time::Duration::from_secs(3));
            println!("thread3 is waken up");
        });

        // Sleep for 2s here
        thread::sleep(std::time::Duration::from_secs(2));
        println!("thread2 is waken up");
        
        thread3.join().unwrap();
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("All threads have quitted");
}