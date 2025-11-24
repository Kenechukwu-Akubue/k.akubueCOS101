use std::fs;
fn main() {
    fs::File::create("data.txt").expect("CREATE FAILED");
    fs::remove_file("data.txt").expect("FAILED TO REMOVE FILE");
    println!("File removed successfully");
}
