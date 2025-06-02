use control_flow::test_for;
use control_flow::test_loop;
use control_flow::test_while;

fn main() {
    println!("Control Flow");
    test_if();
    test_while();
    test_loop();
    test_for();
}

#[allow(dead_code)]
fn test_if(){

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