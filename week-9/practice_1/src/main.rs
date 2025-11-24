use std::io::Write;
fn main() {
    let announce = "Week 9 - Rust File I/O\n";
    let dept = "Department of Computer Science";

    let mut file = std::fs::File::create("data.txt").expect("CREATE FAILED");
    file.write_all("welcome to Rust programming\n"
        .as_bytes())
        .expect("WRITE FAILED");
    file.write_all(announce.as_bytes()).expect("WRITE FAILED");
    file.write_all(dept.as_bytes()).expect("WRITE FAILED");
    println!("\nData written to file successfully.");
}
