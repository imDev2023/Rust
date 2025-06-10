static GLOBAL_NUMBER: i32 = 42;

fn static_str() -> &'static str {
    "I have a static lifetime"
}
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
fn main() {

    let s: &'static str = static_str();
    println!("{}", s);


    // let excerpt: ImportantExcerpt;
    // let novel = String::from("Call me Ishmael. Some years ago...");
    // {
    //     let first_sentence = novel.split('.').next().expect("Could not find a '.' ");
    //     excerpt = ImportantExcerpt {
    //         part: first_sentence,
    //     };
    // }
    // println!("Excerpt: {}", excerpt.part);

  
    // let x = String::from("Hello");
    // let y = String::from("World");
    // let result = longest(&x, &y);
    // println!("{:?}", result);

}

// fn longest<'a>(x : &'a str, y: &'a str)-> &'a str{
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
