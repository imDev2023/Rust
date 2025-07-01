
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn wait_for_next_player(player: &str) {
    println!("\nPass the device to {}. Press ENTER when ready...", player);
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).unwrap();
    clear_screen();
}

fn get_players() -> Vec<String> {
    let mut players = Vec::new();
    println!("👥 Enter player names (type 'done' when finished):");
    loop {
        print!("Player {} name: ", players.len() + 1);
        io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();
        if name.to_lowercase() == "done" {
            break;
        }
        if !name.is_empty() {
            players.push(name);
        }
    }
    players
}

fn play_game(players: &[String]) {
    let secret = rand::rng().random_range(1..=100);
    let mut turn = 0;
    let mut attempts: HashMap<String, u32> = players.iter().map(|p| (p.clone(), 0)).collect();

    loop {
        let current_player = &players[turn % players.len()];
        wait_for_next_player(current_player);

        println!("{}: It's your turn to guess!", current_player);
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid number. Try again.");
                sleep(Duration::from_secs(2));
                clear_screen();
                continue;
            }
        };

        let entry = attempts.entry(current_player.clone()).or_insert(0);
        *entry += 1;

        if guess == secret {
            println!("🎉 {} guessed the number correctly! It was {}!", current_player, secret);
            println!("\n📊 Final Scoreboard:");
            for (player, tries) in &attempts {
                println!("{}: {} attempts", player, tries);
            }
            break;
        } else if guess < secret {
            println!("🔼 Too low!");
        } else {
            println!("🔽 Too high!");
        }

        sleep(Duration::from_secs(2));
        clear_screen();
        turn += 1;
    }
}

fn ask_replay() -> bool {
    println!("\n🔁 Do you want to play again? (yes/no)");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    matches!(answer.trim().to_lowercase().as_str(), "yes" | "y")
}

fn main() {
    loop {
        clear_screen();
        println!("🎯 Welcome to the Multiplayer Guessing Game!");

        let players = get_players();
        if players.len() < 2 {
            println!("❗ At least 2 players are required.");
            continue;
        }

        clear_screen();
        println!("Starting game with players: {:?}", players);
        sleep(Duration::from_secs(2));
        clear_screen();

        play_game(&players);

        if !ask_replay() {
            println!("👋 Thanks for playing! Goodbye.");
            break;
        }
    }
}
