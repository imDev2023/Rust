// mpsc (multiple producer, single consumer)
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;

pub fn test_channels() {
    let (transmitter, receiver) = mpsc::channel::<u8>();

    let processor_code = move || {
        println!("Starting processor thread...");
        let mut failed_count = 0u8;
        loop {
            println!("Attempting to receive message from channel...");
            let receive_result = receiver.recv_timeout(Duration::from_millis(200));
            if receive_result.is_ok() {
                println!("Received message: {}", receive_result.unwrap());
            } else {
                failed_count += 1;
            }
            if failed_count > 10 {
                println!("Aborting processor thread... no work available");
                break;
            }
        }
    };

    for x in 1..=6 {
        let send_result = transmitter.send(x);
        println!("Send Status: {}", send_result.is_ok());
        std::thread::sleep(Duration::from_millis(300));
    }

    std::thread::spawn(processor_code).join();
}
