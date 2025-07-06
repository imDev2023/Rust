use std::io::{self, Write};

fn main() {
    let mut user_balance: f64 = 0.0;
    let mut flag = false;
    loop {
        println!(
            "Press '1' to Deposit, Press '2' for Withdrawal, '3' for Balance Enquire, '4' for Exit"
        );
        let mut input = String::new();
        print!("> {}", input);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let choice: u8 = match input.trim().parse::<u8>() {
            Ok(choice) => choice,
            Err(_) => {
                println!("Invalid Selection");
                continue;
            }
        };

        execute(choice, &mut user_balance, &mut flag);
        if flag == true {
            println!("Thanks for using the simulator. Goodbye!");
            break;
        };
    };
}

fn execute(choice: u8, user_balance: &mut f64, flag: &mut bool) {
    match choice {
        1 => money_deposit(user_balance),
        2 => withdraw_money(user_balance),
        3 => check_balance(user_balance),
        4 => exit(flag),
        _ => println!("Please choose 1, 2, 3, or 4."),
    }
}

fn money_deposit(user_balance: &mut f64) {
    println!("Enter deposit amount: ");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let balance: f64 = match input.trim().parse::<f64>() {
        Ok(choice) => choice,
        Err(_) => {
            println!("Please enter valid numbers");
            return;
        }
    };
    *user_balance = *user_balance + balance;
    println!("Your Balance After Deposit is : ${:.2}", *user_balance);
}

fn withdraw_money(user_balance: &mut f64) {

    println!("Enter withdrawal amount: ");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let balance: f64 = match input.trim().parse::<f64>() {
        Ok(choice) => choice,
        Err(_) => {
            println!("Please enter valid numbers");
            return;
        }
    };

    if balance > *user_balance{
        println!("Insufficient funds.");
        return;
    }

    *user_balance = *user_balance - balance;
    println!("Your Balance After Withdrawl is : ${:.2}", user_balance);
}

fn check_balance(user_balance: &mut f64) {
    println!("Your current balance is : ${:.2}", user_balance);
}

fn exit(flag: &mut bool) {
    *flag = true;
}
