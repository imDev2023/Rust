pub fn test_iterators() {
    let fruit_list = vec!["Strawberry", "Blueberry", "Mango", "Orange", "Apple"];

    let nut_list = vec!["Walnut", "Almonds", "Pecans", "Pine"];

    let mut fruit_iter = fruit_list.iter();

    fruit_iter.next();
    fruit_iter.next();
    let item01 = fruit_iter.next();
    // println!("{}", item01.unwrap());

    // Test .chain() Method
    let aggregate_foods = fruit_list.iter().chain(&nut_list);

    let all_foods: Vec<&&str> = aggregate_foods.clone().collect();

    for food in aggregate_foods {
        println!("Eating {}", food);
    }

    let fruit_list_string = fruit_list.iter().map(|e: &&str| String::from(*e));
    let new_fruits = fruit_list_string.map(|mut e: String| {
        e.push_str(" fruit");
        return e;
    });

    new_fruits.clone().for_each(|e| println!("{}", e));

    println!("Last fruit is : {}", new_fruits.clone().last().unwrap());

    let mut stepby = new_fruits.clone().step_by(2);
    println!("step : {}", stepby.next().unwrap());
    println!("step : {}", stepby.next().unwrap());
    println!("step : {}", stepby.next().unwrap());

    let first_name = vec!["Farhan", "Subhan", "Dua", "Iqqra"];
    let first_name_strings = first_name.iter().map(|e| String::from(*e));

    let last_name = vec!["Basharat", "Farhan", "Butta", "Khan"];
    let last_name_strings = last_name.iter().map(|e| String::from(*e));

    let full_names = first_name_strings.zip(last_name_strings);
    // full_names.for_each(|e| println!("{} {}",e.0, e.1));

    // for (index, value) in full_names.enumerate(){
    //     println!("Index: {0} value: {1} {2}", index, value.0, value.1);
    // }

    // full_names.skip(2).take(1).for_each(|e|println!("{}", e.0));

    let foods = vec![("potatoes", 10), ("strawberries", 25), ("Burgers", 31)];

    let result = foods.clone().iter().fold(0u32, |mut a: u32, e| a + e.1);
    println!("result : {}", result);

    let food_iter = foods.iter();
    let mut mypeekable = food_iter.peekable();
    mypeekable.next();
    let food = mypeekable.peek();
    println!("peeking at : {}", food.unwrap().0)
}
