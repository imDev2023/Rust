pub mod optiontest;

fn main () {

let results = optiontest::test_option_type();
println!("{0}", results.unwrap());

let str_results = optiontest::test_option_string();
println!("name is: {}", str_results.unwrap());

let char_results = optiontest::test_option_chartype();

if char_results.is_some() {
    println!("User has selected a character type")
} else {
    println!("charater type is None");
}
println!("Character type selected is : {}", char_results.unwrap().to_string());

}