use csv::{Reader, ReaderBuilder};

#[derive(serde::Deserialize, Debug)]
struct Vehicle {
    #[serde(rename(deserialize = "Manufacturer"))]
    manufacturer : String,
    #[serde(rename(deserialize = "Model"))]
    model: String,
    #[serde(rename(deserialize = "VIN"))]
    vin: String,
}

fn main() {
    let file_name = "data.csv";
    let mut builder = ReaderBuilder::new();
    // builder.double_quote(false).comment(Some(b'-')).has_headers(false);
    builder.double_quote(false).comment(Some(b'-'));
    let result = builder.from_path(file_name);
    
    // let result = Reader::from_path(file_name);

    if result.is_err(){
        println!("Failed to read CSV");
        std::process::exit(9);
    }

    let mut my_reader = result.unwrap();

    // for record in my_reader.records(){
    for record in my_reader.deserialize(){
        let car: Vehicle = record.unwrap();
        // println!("You car manufacturer is {}", car.get(0).unwrap());
        // println!("You car model is {}", car.get(1).unwrap());
        // println!("You car vin is {}", car.get(2).unwrap());
        println!("You car manufacturer is {}", car.manufacturer);
        println!("You car model is {}", car.model);
        println!("You car vin is {}", car.vin);
    }
}
