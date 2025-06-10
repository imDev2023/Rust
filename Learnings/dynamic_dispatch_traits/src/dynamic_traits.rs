pub fn testing_dynamic_traits() {
    let dog01: &dyn AnimalSound = &Dog{};
    make_animal_noise(dog01);
    let antelope01:&dyn AnimalSound = &Antelope {};
    make_animal_noise(antelope01);
    let bear01: &dyn AnimalSound = &Bear {};
    make_animal_noise(bear01);
    println!("=== Animal Eats ===");
    let dog01: &dyn AnimalEating = &Dog {};
    animal_food(dog01);
    let antelope01: &dyn AnimalEating = &Antelope {};
    animal_food(antelope01);
    // let bear01:&dyn AnimalEating = &Bear {};
    // animal_food(bear01);

    let animal01 = get_animal();
     animal01.eat_food();
    let animal02 = get_animal();
    animal02.make_sound();
     

// fn get_animal() ->Box<dyn AnimalEating> {
//     let bear = Bear{};
//     return Box::from(bear);
// }

// Super Trait
fn get_animal() -> Box<dyn Animal>{
    let animal = Antelope{};
    return Box::from(animal);
}


// dynamic trait
fn make_animal_noise(a: &dyn AnimalSound){
    a.make_sound();
}

fn animal_food(a: &dyn AnimalEating){
    a.eat_food();
}

// usual trait method
// fn make_animal_noise<Animal: AnimalSound>(a: Animal) { 
//     a.make_sound();
// }
// fn animal_food<Animal: AnimalEating>(a: Animal) {
//     a.eat_food();
// }
}
struct Dog {}
struct Antelope {}
struct Bear {}

trait Animal : AnimalEating + AnimalSound {} // super trait defined

trait AnimalEating {
    fn eat_food(&self);
}

trait AnimalSound {
    fn make_sound(&self);
}

impl AnimalEating for Dog {
    fn eat_food(&self) {
        println!("Dog is eating Dog Food");
    }
}

impl AnimalSound for Dog {
    fn make_sound(&self) {
        println!("Dog is Barking!");
    }
}

impl Animal for Dog {}

impl AnimalEating for Antelope {
    fn eat_food(&self) {
        println!("Antelope is eating natural desert food");
    }
}

impl AnimalSound for Antelope {
    fn make_sound(&self) {
        println!("Antelope is bleating!");
    }
}
impl Animal for Antelope {
    
}
impl AnimalEating for Bear {
    fn eat_food(&self) {
        println!("Bear is Eating Honey");
    }
}

impl AnimalSound for Bear {
    fn make_sound(&self) {
        println!("Bear is Roaring!");
    }
}
impl Animal for Bear {
    
}
