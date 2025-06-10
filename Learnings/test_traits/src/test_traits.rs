struct Person<PetType, PetType2: Animal + Dangerous> where PetType: Animal + NotDangerous{
    first_name: String,
    pet: PetType,
    pet2:PetType2,
}

trait Animal {
    fn make_sound(&self) -> ();
}
trait NotDangerous {}
trait Dangerous {}
 
#[allow(dead_code)]
struct Dog {}
impl NotDangerous for Dog {}
impl Animal for Dog { 
    fn make_sound(&self) -> () {
        println!("Dog Barked");
    }
}

#[allow(dead_code)]
struct Cat {}
impl NotDangerous for Cat {}
impl Animal for Cat {
    fn make_sound(&self) -> () {
        println!("Cat Meowed");
    }
 }

#[allow(dead_code)]
struct Bear {}
impl Dangerous for Bear {}
impl Animal for Bear {
    fn make_sound(&self) -> () {
        println!("Bear Roared");
    }
 }

#[allow(dead_code)]
struct Tiger {}
impl Dangerous for Tiger {}
impl Animal for Tiger {
    fn make_sound(&self) -> () {
        println!("Tiger Roared");
    }
 }

pub fn create_person() {
    let pet1 = Dog{};
    let pet2 = Cat{};
    let pet3 = Bear{};
    let pet4 = Tiger{};
    let p1 = Person {first_name: "asdfsafdsaf".to_string(), pet:pet2, pet2: pet3};
    p1.pet.make_sound();
    p1.pet2.make_sound();
}