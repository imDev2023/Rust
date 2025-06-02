pub fn test_for() {
    let ages = [ 19, 25, 37, 81, 14];
    let age_to_drive:i32 = 16i32;

    for value in ages {
        println!("The current age is {0}", value);
        if value >= age_to_drive {
            println!("You are old enough to drive!");
        }
        else {
            println!("You need to wait a little bit more...");
        }
    }

}

pub fn test_loop () {
    let mut x = 1;
    loop {
        println!("Hello from Rust! 🦀");
        if x > 5 {
            break;
        }
        x += 1;
    }
}

pub fn test_while(){
    let age_to_drive: u8 = 16u8;

    let mut current_age:u8 = 0u8;
    while current_age < age_to_drive {
        println!("Waiting...");
        current_age +=1;
        if current_age == 6 {
            break;
        }
    }
}

pub fn test_if(){

    let age_to_drive: u8 = 16u8;

    println!("Enter the person's age: ");
    let my_input = &mut String::from("");
    std::io::stdin().read_line(my_input).unwrap();

    let age = my_input.replace("\n", "").parse::<u8>().unwrap();
    if age >= age_to_drive {
        println!("Issuing driver's license, because they are old enough")
    }
    else if age == 16 || age > 14{
        println!("You are just being old enough! Wait one more year.");
    }
    else {
        println!("Not old enough for a driver's license!");
    }

    let drivers_license: bool = if age >= 16 { true } else { false };
}