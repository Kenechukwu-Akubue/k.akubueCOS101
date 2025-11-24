use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new().append(true).create(true).open("data.txt").expect("FAILED TO OPEN FILE");
    file.write_all("\nHello Class".as_bytes()).expect("FAILED TO WRITE TO FILE");
    file.write_all("This is the appendage to the document.".as_bytes()).expect("FAILED TO WRITE TO FILE");
    println!("Data appended to file successfully.");
}
