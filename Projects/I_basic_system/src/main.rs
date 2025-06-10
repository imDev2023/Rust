use std::io::{self, Write};
struct User {
    username: String,
    active: bool,
}
enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Say(String),
    Invalid,
}

fn parse_input(input: &str) -> Command {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    match parts.as_slice() {
        ["quit"] => Command::Quit,
        ["move", x, y] => {
            let x = x.parse::<i32>();
            let y = y.parse::<i32>();
            if let (Ok(x), Ok(y)) = (x, y) {
                Command::Move { x, y }
            } else {
                Command::Invalid
            }
        }
        ["say", msg @ ..] => Command::Say(msg.join(" ")),
        _ => Command::Invalid,
    }
}

fn handle_command(user: &User, cmd: Command) -> bool {
    match cmd {
        Command::Quit => {
            println!("{} has quit.", user.username);
            return false;
        }
        Command::Move { x, y } => {
            println!("{} moved the position to ({}, {}).", user.username, x, y);
            return true;
        }
        Command::Say(msg) => {
            println!("{} says: {}", user.username, msg);
            return true;
        }
        Command::Invalid => {
            println!("Invalid Command. Try: move x y, say <message>, or quit.");
            return true;
        }
    }
}
fn main() {
    let user = User {
        username: String::from("Farhan Basharat"),
        active: true,
    };

    println!("Welcome, {}!", user.username);
    println!("Commands: move x y, say <message>, quit");

    loop {
        print!(">>>");
        io::stdout().flush().unwrap(); //Flush to ensure prompt shows

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input");
            continue;
        }

        let command = parse_input(&input);
        let continue_running = handle_command(&user, command);
        if !continue_running {
            break;
        }
    }
}