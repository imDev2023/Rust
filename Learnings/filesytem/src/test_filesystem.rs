use std::fs;

pub fn read_file() {
    let file_path = "./data/file01.txt";
    let read_file = std::fs::read(file_path);

    let converted_from_bytes_to_strings = |mut a: String, v: &u8| {
        let new_char = char::from(*v);
        a.push(new_char);
        return a;
    };

    if read_file.is_ok() {
        println!(
            "content of file01 is: {}",
            read_file
                .ok()
                .unwrap()
                .iter()
                .fold(String::from(""), converted_from_bytes_to_strings)
        );
    }
}

pub fn remove_dir() {
    let path = "./data";
    _ = std::fs::remove_dir_all(path);
}

pub fn create_files() {
    let path = "./data/file01.txt";
    let text = "Farhan Basharat";
    let path2 = "./data/file02.txt";
    let text2 = "Subhan Farhan";
    let path3 = "./data/file03.txt";
    let text3 = "Farhan Basharat 03";
    _ = std::fs::write(path, text);
    _ = std::fs::write(path2, text2);
    _ = std::fs::write(path3, text3);

    // _= std::fs::remove_file(path3);
}

pub fn my_filesystem() {
    let path = "./data";
    let my_path = std::path::Path::new(path);
    if my_path.exists() {
        println!("Directory Already exists!");
        return;
    }

    let create_dir_result = fs::create_dir(path);
    if create_dir_result.is_ok() {
        println!("Created new data directory");
    } else {
        println!("Problem occurred : {:?}", create_dir_result.err());
    }
}
