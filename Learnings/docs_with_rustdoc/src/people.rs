/// This module deals with People and Person objects.
/// 
/// ## Detailed Intro
/// 
/// - This mod has Person struct to represent users
/// - This mod can be expanded in the future to deal with other user-related stuff.
pub mod people {
    /// This Person struct represents a user of the system
    pub struct Person {
        /// This `f_name` field represents the first name of the person
        f_name: String,
        /// This is the last name of the Person who subscribed to my Cloud SaaS service.
        last_name: String,
        /// This field points to all the [crate::animals::Dog] owned by this person.
        animals: Vec<crate::animals::Dog>,

    }
}