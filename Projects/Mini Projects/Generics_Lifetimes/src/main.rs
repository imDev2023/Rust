use std::ops::Add;

#[derive(Debug)]
struct Student<'a> {
    name: &'a str,
    math: u32,
    science: u32,
}

trait Grader {
    fn average(&self) -> f32;
}

impl<'a> Grader for Student<'a> {
    fn average(&self) -> f32 {
        (self.math + self.science) as f32/2.0
    }
}

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
fn main() {
    let student1 = Student {
        name: "Alice",
        math: 90,
        science: 85,
    };
    
    println!("{:?}", student1);
    println!("Average score: {}", student1.average());

    let sum = add(5, 10);
    println!("Generic add result: {}", sum);

}
