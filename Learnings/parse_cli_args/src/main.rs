pub mod myargs;

fn main() {
    println!("Parse CLI Arguments With Rust Standard Crate");
    myargs::process_args();
}
