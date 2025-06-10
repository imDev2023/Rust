use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
// #[serde(rename_all="PascalCase")]
struct Dog {
    name: String,
    year_born: i32,
    owner: DogOwner,
    breed: String,
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all="PascalCase")]
struct DogOwner {
    first_name: String,
    last_name: String,
}

fn main() {
    deserialize();
    // serializedTest();
}

fn serializedTest() {
    let owner01 = DogOwner {
        first_name: "Farhan".to_string(),
        last_name: "Basharat".to_string(),
    };
    let dog01 = Dog {
        breed: "Husky".to_string(),
        name: "Cheyenne".to_string(),
        year_born: 2021,
        owner: owner01,
    };
    let dog_ser = to_string_pretty(&dog01);

    if dog_ser.is_ok() {
        println!("{}", dog_ser.ok().unwrap());
    } else {
        println!("{:#?}", dog_ser.err());
    }
}

fn deserialize() {
    let json_string = r#"
    {
  "name": "Cheyenne",
  "year_born": 2021,
  "owner": {
    "first_name": "Farhan",
    "last_name": "Basharat"
  },
    "breed": "Husky"
}
    "#;

    let dog_deser = from_str::<Dog>(json_string);

    if dog_deser.is_ok() {
        println!("{:#?}", dog_deser.ok().unwrap());
    } else {
        println!("{:#?}", dog_deser.err());
    }
}
