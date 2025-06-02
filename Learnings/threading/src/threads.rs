use std::thread::spawn;

pub fn test_threads() {
    let mut x = 0u128;
    for i in 1..500_000_000 {
        x += i;
    }
    println!("Main Thread value of x is: {x}");
    println!("\x1b[38;2;100;100;255mMain thread finished work... Check on worker thread\x1b[0m]");
}

pub fn spawn_thread() {
    let thread_fn = || {
        let mut x = 0u128;
        for i in 1..500 {
            x += i;
        }
        println!("value of x is: {x}");
    };
    println!("Starting new worker thread...");
    let handle = spawn(thread_fn);
    let handle2 = spawn(thread_fn);
    println!("Starting new worker completed...");

    // test_threads();

    loop {
        test_threads();
        if handle.is_finished() && handle2.is_finished() {
            println!("All the workers are done, lets get out of here!");
            break;
        }
    }
}
