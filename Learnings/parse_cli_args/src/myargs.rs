use std::env::args;

pub fn process_args() {
    let myargs: Vec<String> = args().collect();
    println!("{:?}", args());

    if myargs.len() != 3 {
        println!("Hey, you didn't specify two arguments");
        return;
    }
    let name: String = myargs.get(1).unwrap().into();
    let parsed_year = myargs.get(2).unwrap().parse::<i32>();
    if !parsed_year.is_ok() {
        println!("The specified dog year was invalid.");
        return;
    }
    let year_born = parsed_year.ok().unwrap();

    println!("{name} {year_born}");
    let dog01 = new_dog(name, year_born);
    dog01.get_details();
}
fn new_dog(name: String, year_born: i32) -> Dog {
    return Dog { name, year_born };
}
struct Dog {
    name: String,
    year_born: i32,
}
impl Dog {
    fn get_details(&self) {
        println!(
            "Dog name is {}, and was born in year {}",
            self.name, self.year_born
        );
    }
}
