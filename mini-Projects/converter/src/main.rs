use std::io::{self, Write};

fn main() {
    loop {
        print!("Enter a number to convert to Hex and Binary: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let num = match input.trim().parse::<i64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number, try again.\n");
                continue;
            }
        };

        println!("Binary: {}", binary_converter(num));
        println!("Hexadecimal: {}", hex_converter(num));
        break;
    }
}

fn binary_converter(mut num: i64) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut binary = String::new();
    while num > 0 {
        binary.insert(0, char::from_digit((num % 2) as u32, 10).unwrap());
        num /= 2;
    }
    binary
}

fn hex_converter(mut num: i64) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut hex = String::new();
    while num > 0 {
        let digit = (num % 16) as u32;
        hex.insert(0, char::from_digit(digit, 16).unwrap().to_ascii_uppercase());
        num /= 16;
    }
    hex
}
