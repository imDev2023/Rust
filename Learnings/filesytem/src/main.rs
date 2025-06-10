pub mod test_filesystem;

fn main() {
    println!("Rust Filesystem APIs");

    test_filesystem::my_filesystem();
    test_filesystem::create_files();
    // test_filesystem::remove_dir();
    // test_filesystem::read_somefile();
    test_filesystem::read_file();

}
