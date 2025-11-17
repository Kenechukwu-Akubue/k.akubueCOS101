fn main() {
    //mutable array
    let mut colors = ["Red", "Green", "Yellow", "White"];

    println!("\nOriginal array: {:?}", colors);

    //mutable slice
    let sliced_colors = &mut colors[1..3];

    println!("First slice: {:?}", sliced_colors);

    sliced_colors[1] = "Blue";

    println!("Changed slice: {:?}", sliced_colors);
}
