#![deny(clippy::too_many_arguments)]
fn main() {
    println!("Hello, world!");
    test_lint(1,1,1,1);
}

fn test_lint(arg1: u8, arg2: u8, arg3: u8, arg4: u8) {

}
#[allow(unused_variables)]
fn test_string(arg1: u16) {
    let x = "Hello from Rust, Farhan";

    println!("{}", x.to_string());
}