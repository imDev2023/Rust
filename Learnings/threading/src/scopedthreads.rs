struct Person {
    first_name : String,
}

pub fn test_thread_variables(){
    let age = 39;
    let person01 = Person {first_name: String::from("Farhan")};

    let print_age = || {
        println!("your age is {age}");
        println!("You name is {}", &person01.first_name);
    };
    
    std::thread::scope(|scope| {
        scope.spawn(print_age);
    });
    
    // let _result = std::thread::spawn(print_age).join();
    println!("your age is {age}");
    println!("You name is {}", person01.first_name);
    println!("Finished printing age");
}