struct Person {
    first_name: String,
    last_name: String,
}


fn main() {
    println!("Closures");
    test_closures();
}

fn test_closures() {
    let add = |x, y: i32| {
        println!("x: {} y: {}", x, y);
        x + y
    };
   let result = add(6, 29);

   let print_result = |x: i32| println!("The result is {}", result + x);
   print_result(100);

    let mut p1 = Person{first_name: "Farhan".to_string(), last_name: "Basharat".to_string()};
    let mut change_name = |new_last_name: &str| p1.last_name = new_last_name.to_string();
    change_name("Bhutta Sahib");
    println!("{}", p1.last_name);
}
