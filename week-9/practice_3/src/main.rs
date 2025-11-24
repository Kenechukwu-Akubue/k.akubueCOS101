use std::fs;
fn main() {
    fs::remove_file("data.txt").expect("FAILED TO REMOVE FILE");
    println!("File removed successfully");
}
