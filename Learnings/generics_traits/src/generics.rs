use std::ops::Add;

pub fn add<T: Add<Output = T>>(a: T, b: T)-> T{
    a + b
}

pub trait Test<T> {
    fn name(&self) -> String;
    fn score(&self) -> T;
}



// #[derive(Debug)]
pub struct Exam {
    name: String,
    points: i32,
    max_points: i32,
}

impl Exam {
   pub fn new(name: &str, points: i32, max_points: i32) -> Self {
        Exam { name: name.to_owned(), points, max_points }
    }
}

impl Test<i32> for Exam {
    fn name(&self) -> String {
      self.name.to_owned()
    }
    fn score(&self) -> i32 {
        self.points * 2
    }
}

// impl Test<f32> for Exam {
//     fn name(&self) -> String {
//         format!("{} - Percentage", self.name)
//     }
//     fn score(&self) -> f32 {
//         self.points as f32 / self.max_points as f32 * 100.0
//     }
    
// }