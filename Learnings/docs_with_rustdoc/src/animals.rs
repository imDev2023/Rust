#![doc = include_str!("./animal-docs.md")]
#[warn(missing_docs)]

/// This is the Dog type that represents dogs in my game.
pub struct Dog {
    breed: String,
}

/// This represents a cow in my zelda-like game.
pub struct Cow {
    /// This is the name of the cow.
    name: String,
}