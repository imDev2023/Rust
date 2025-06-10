use std::f32::consts::E;

pub fn test_vec_int() {
    let mut my_ints: Vec<i32> = Vec::new();

    my_ints.push(20);
    my_ints.push(10);
    my_ints.push(30);
    my_ints.push(50);
    my_ints.push(60);

    println!("Size of Vec {:?}", my_ints.len());
    println!("Capacity of Vec {:?}", my_ints.capacity());
    println!("{:?}", my_ints);

    // println!("First item in Vec is: {:?}", &(&my_ints).as_slice()[0..=4]);

    println!("First element is: {:?}", my_ints.get(1));
}

pub fn test_vec_string() {
    let first_names = vec!["Farhan", "Subhan", "Dua", "Zoya"];

    for first_name in first_names.as_slice() {
        println!("Processing {}...", first_name);
    }

    println!("{:?}", first_names);
}
#[derive(Debug)]
struct Car {
    manufacturer: String,
    model: String,
}
pub fn tes_vec_car() {
    let mut car_list: Vec<Car> = vec![];
    let mut car_lot2: Vec<Car> = vec![];

    for _ in 1..=4u8 {
        car_list.push(Car {
            manufacturer: "Porsche".to_string(),
            model: "Panamera".to_string(),
        });
    }

    for _ in 1..=4u8 {
        car_lot2.push(Car {
            manufacturer: "Hyundai".to_string(),
            model: "Tuscon".to_string(),
        });
    }

    car_list.append(&mut car_lot2);
    car_list.insert(
        0,
        Car {
            manufacturer: "Lambo".to_string(),
            model: "SUV".to_string(),
        },
    );
    car_list.remove(0);

    let keep = |e: &Car| {
        if e.manufacturer == "Porsche" {
            return true;
        } else {
            return false;
        }
    };
    car_list.retain(keep);

    car_list.reserve(5000);

    println!("{:?}", car_list);
    println!("{:?}", car_list.len());
    println!("{:?}", car_list.capacity());
    println!("----------");

    println!("{:?}", car_lot2);
    println!("{:?}", car_lot2.len());
    println!("{:?}", car_lot2.capacity());

    println!("{:?}", car_list.get(0).unwrap());
}
