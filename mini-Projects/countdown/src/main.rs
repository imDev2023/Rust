use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    loop {
        print!("Please enter your countdown number: ");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let timer: u16 = match input.trim().parse() {
            Ok(timer) => timer,
            Err(_) => {
                println!("Invalid Number, Please try again.");
                continue;
            }
        };
        start_timer(timer);
        break;
    }
}

fn start_timer(timer: u16) {
    for i in (1..=timer).rev() {
        println!("countdown timer: {}", i);
        sleep(Duration::from_secs(1));
    }
}
