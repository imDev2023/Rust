use std::default::Default;
#[derive(Debug)]
struct FirstName(String);

impl Default for FirstName{
    fn default() -> Self {
        return FirstName("FarhanBhutta".to_string());
    }
}

#[derive(Debug)]
struct Person {
    first_name: FirstName,
    last_name: String,
    age: u8,
    location: String,
}

impl Default for Person {
    fn default() -> Self {
        return Person {
            first_name: FirstName::default(),
            last_name: "Basharat".to_string(),
            age: 39,
            location: "UK".to_string(),
        };
    }
}

// fn new_person() -> Person {
//     return Person{
//         first_name: "Farhan".to_string(),
//         last_name: "Basharat".to_string(),
//         age: 39,
//         location: "UK".to_string(),
//     };
// }

fn main() {
    println!("Default String: {}", String::default());
    let p1 = Person::default();
    println!("Default Person is : {:#?}", p1);
}
