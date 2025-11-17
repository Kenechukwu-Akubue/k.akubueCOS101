fn main() {
    let v = vec!['C', 'O', 'M', 'P', 'U', 'T', 'E', 'R'];

    let mut index = String::new();
    println!("Enter an index value between 0 and 7: ");
    std::io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Please type a number!");
    //index is non negative and smaller than the length of the vector

    let ch: char = v[index];

    print!("{} is the character for index [{}].\n", ch, index);
}
