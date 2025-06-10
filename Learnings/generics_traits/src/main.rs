// use generics::Test;

// use crate::generics::Exam;


// pub mod generics;

fn main() {
    println!("Learning Generics and Traits!");

    println!("-------------");

    // let sum = generics::add(19, 1);
    // let sum_float = generics::add(1.2 as f32,1.9 as f32);
    // let sum_float64 = generics::add(19.1 as f64, 9.2 as f64);
    // println!("result of generics function is: {}, sum_float: {}, sum_f64: {}", sum, sum_float, sum_float64);

    // let exam: Exam = Exam::new("Farhan Basharat", 5, 12);
    // println!("Exam : {0}, {1}", exam.name(), exam.score());

    // let number_list = vec![49, 59, 89,98];

    // let mut largest = &number_list[0];

    // for num in &number_list {
    //     if num > largest {
    //         largest = num;
    //     }
    // }
    // println!("{}", largest);
    // let vehicle = Sedan {trunk : 5.4};
    // let vehicle = SUV {off_road: String::from("Good")};
    // let person = Person {name: String::from("Farhan"), car: vehicle};

    // person.print_info();
    let rectangle = Rectangle {
        width: 8,
        height: 4,
    };
    println!("{}", rectangle.get_area());
}

// struct Person<T : Car> {
//     name: String,
//     car: T,
// }
// impl <T:Car> Person<T> {
//     fn print_info(&self){
//         println!("Name = {}", self.name);
//         self.car.print_spec();
//     }
// }
// trait Car {
//     fn print_spec(&self);
// }

// struct Sedan{
//     trunk:f32,
// }
// impl Car for Sedan {
//     fn print_spec(&self) {   
//         println!("Sedan!");
//         println!("Trunk Capacity: {}", self.trunk);
//     }
// }
// struct SUV{
//     off_road: String
// }
// impl Car for SUV{
//     fn print_spec(&self) {   
//         println!("SUV");
//         println!("Suv :{}", self.off_road);
//     }
// }
// struct Minivan{
//     sliding : String,
// }
// impl Car for Minivan {
//     fn print_spec(&self) {
//        println!("Minivan");
//         println!("Minivan: {}", self.sliding);
//     }
    
// }

struct Rectangle {
    width: u32,
    height: u32,
}

trait IShape {
    fn get_area (&self)-> u32;
}

impl IShape for Rectangle {
    fn get_area (&self)-> u32 {
        self.width * self.height
    }
}