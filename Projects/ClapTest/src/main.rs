use clap::{Arg, Command, command};

fn main() {
    let match_result = command!()
        .subcommand(
            Command::new("register-person")
                .arg(
                    Arg::new("firstname")
                        .short('f')
                        .long("first-name")
                        .aliases(["fname", "firstname"])
                        .required(true)
                        .help("The person's first name"), // .conflicts_with("lastname")
                )
                .arg(
                    Arg::new("lastname")
                        .short('l')
                        .long("last-name")
                        .aliases(["lname", "lastname"])
                        .required(true)
                        .help("This argument takes the person's last name"),
                ),
        )
        .subcommand(
            Command::new("register-pet")
                .arg(
                    Arg::new("pet-name")
                        .long("pet-name")
                        .short('n')
                        .required(true),
                )
                .about("This application registers people with their doctors office"),
        )
        .arg(
            Arg::new("fluffy")
                .long("fluffy")
                .help("Is the person wearing fluffy coat 🧥 or not?"),
        )
        .get_matches();

    // println!("Fluffy: {}", match_result.get_one::<String>("pet-name").unwrap_or(&"No Pet Name".to_string()))

    // let pet_args = match_result.subcommand_matches("register-pet");
    // println!("Pet name {}",  pet_args.unwrap().get_one::<String>("pet-name").unwrap());

    let person_args = match_result.subcommand_matches("register-person").unwrap();
    println!("First name {} second name {}", person_args.get_one::<String>("firstname").unwrap(), person_args.get_one::<String>("lastname").unwrap())

}
