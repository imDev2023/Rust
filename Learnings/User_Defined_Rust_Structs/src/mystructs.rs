use std::cell::Cell;

#[derive(Debug)]
#[allow(dead_code)]
enum VehicleColor {
    Silver,
    Blue,
    Green,
    Red,
    White,
    Black,
}
enum CharacterType {
    Mage,
    Warrior,
    Wizard,
    Archer,
}
#[derive(Debug)]
struct VehicleTuple(String, String, u16); // Tuple Based Structs
pub fn create_vehicle_typle() {
    let myvehicletuple = new_vehicle_tuple();
    println!("Manu: {0}, Model: {1}, Year: {2}", myvehicletuple.0, myvehicletuple.1,myvehicletuple.2);
}
fn new_vehicle_tuple() -> VehicleTuple {
    return VehicleTuple("Benz".to_string(), "GLS".to_string(), 2025);
}

#[derive(Debug)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: VehicleColor,
}

impl Vehicle {
    fn paint(&mut self, new_color: VehicleColor) {
        self.color = new_color;
    }

    fn create_vehicle()-> Vehicle {
        let new_vehicle = Vehicle{manufacturer: "default".to_string(), model: "default".to_string(), year: 1990, color:VehicleColor::Red};
        return new_vehicle;
    }
}

fn new_vehicles() -> Vehicle {
    let mut v1 = Vehicle {
        manufacturer: "Porche".to_string(),
        model: "911".to_string(),
        year: 2025,
        color: VehicleColor::White,
    };
    v1.paint(VehicleColor::Silver);
    return v1;
}

pub fn create_vehicle() {
    // let my_vehicle = new_vehicles();
    let my_vehicle = Vehicle::create_vehicle();
    println!("{:?}", my_vehicle);
}
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub birth_year: u16,
    pub birth_month: u8,
    meters_walked: u32,
}

fn new_person() -> Person {
    let p1 = Person {
        first_name: "Farhan".to_string(),
        last_name: "Basharat".to_string(),
        birth_month: 07,
        birth_year: 1985,
        meters_walked: 0,
    };
    //  p1.first_name.set("Subhan"); // changed first named
    return p1;
}

impl Person {
    fn walked_meters(&mut self, meters: u32) {
        self.meters_walked += meters;
    }
}

pub fn test_create_person() {
    let mut my_person = new_person();
    my_person.walked_meters(8);
    my_person.walked_meters(12);
    println!(
        "First name: {0}, last name: {1}, birth year: {2}, birth month: {3}, meters walked {4}",
        my_person.first_name, my_person.last_name, 
        my_person.birth_year, my_person.birth_month,
        my_person.meters_walked
    );
}
