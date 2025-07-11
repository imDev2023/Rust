fn main() {
    println!("Hello, world!");
    test_func(); 
}
// Unit Type
fn test_func() {
    let x: () = ();
    println!("Unit Type = {:?}", x);
    
    // integers
    let y: f32 = 255.0;
    let z: u8 = y as u8 - 5;
    println!("inter = {:?}", z);

    // bool
    let mut iamold: bool = true;
    iamold = false;
    println!("Bool = {}", iamold);

    // Char
    let my_char: char = '👍';
    println!("Char = {}", my_char);

    // &str
    let mut first_name: &str = "Farhan";
    println!("&str = {}", first_name);

    first_name = "Basharat";
    println!("mut &str = {}", first_name);

    //Tuple
    let name = ("Farhan", "Basharat", 39 as u8);
    println!("{:?}", name);

    // Array/List
    let age: [u16; 3] = [40, 45, 50];
    println!("{:?}", age);

    //Slices
    let ages: [u16; 6] = [40, 45, 50, 55, 60, 65];
    let new_ages = &ages[1..=4];
    println!("{:?}", new_ages);

}
