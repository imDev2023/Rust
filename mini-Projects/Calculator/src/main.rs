//Calculator - add, sub, div, square root

use std::io;
mod calc_fn;
use calc_fn::calculator::*;

fn get_number(prompt: &str) -> u32 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Not Valid Input");

        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Try again"),
        }
    }
}

fn main() {
    let num_one = get_number("Enter Your First Number: ");
    let num_two = get_number("Enter Your Second Number: ");

    println!("The Addition is {}", add(num_one, num_two));
    println!("The Subtract is {}", sub(num_one, num_two));
    println!("The Multiplication is {}", mul(num_one, num_two));
    // println!("The Divide is {}", div(num_one, num_two));
    match div(num_one, num_two) {
        Some(result) => println!("The Division result is {}", result),
        None => println!("The denominator can not be Zero"),
    }
    println!("The Square Root of Number one : {}", square_root(num_one));
    println!("The Square Root second No : {}", square_root(num_two));
}
