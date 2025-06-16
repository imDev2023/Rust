//! This is some sample content for the crate-level docs.
//! 
//! ## Detailed Introduction
//! 
//! This is a detailed introduction.
//! 
//! ## Features
//! 
//!  - [x] This crate allows you to create Person objects
//!  - [ ] This crate allows you to create grocery bills
//! 
//! ## Example
//! 
//! ```
//! let p1 = Person{first_name: "Farhan".to_string()};
//! println!("{}", p1.first_name);
//! ```
//! 
//! ~~This is outdated so do NOT use this.~~
//! 
//! This is a really **important** concept! 🦀 👍

pub mod animals;
pub mod people;
/// This is the entry-point to our People application
fn main() {
    println!("Hello, world!");
}

