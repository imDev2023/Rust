pub fn test_match_string() {
    let car_manufacturer = "Porshe";

    match car_manufacturer {
        "Hyundai" => 30000,
        "Porshe" => 90000,
        _ => 0,
    };
}

pub fn test_match_array() {
    let prices: [u32; 4] = [30000, 50000, 90000, 120000];

    match prices[0..=3] {
        [30000, 50000] => println!("you have some reasonably priced cars"),
        [30000, 50000, ..] => println!("you have variety of cars!"),
        _ => println!("You don't have any reasonably priced cars"),
    }
}

pub fn test_match_int() {
    let mut my_input = String::from("");
    std::io::stdin().read_line(&mut my_input).unwrap();
    let age = my_input.replace("\n", "").parse::<u16>().unwrap();

    let y: u8 = 5;

    match age {
        0 => println!("You are a new born"),
        1..=35 if y == 5 => println!("Your age is up to 35, y is 5"),
        1..=35 if y != 5 => println!("Your age is up to 35, y is 5, y is not 5"),
        1..=35 => println!("You age is 1 to 35, y is not defined"),
        36..=149 => println!("You age is between 36 and 149"),
        150.. => println!("Your is over 150"),
    }
}
