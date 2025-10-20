use std::io;

fn main() {

    let mut input = String::new();
    println!("Enter value for a:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a: f32 = input.trim().parse().expect("Failed to input. Check if your value is numerical and try again.");
    if a == 0.0 {
        panic!("The value of 'a' must not equal zero!");
    }

    input.clear();
    println!("Enter value for b:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let b: f32 = input.trim().parse().expect("Failed to input. Check if your value is numerical and try again.");

    input.clear();
    println!("Enter value for c:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let c: f32 = input.trim().parse().expect("Failed to input. Check if your value is numerical and try again.");

    println!("Solving: {}x^2 + {}x + {}", a, b, c);

    let d: f32 = b.powi(2) - 4.0 * a * c;

    if d < 0.0 {
        println!("There are no real roots for the equation.");
    } else if d == 0.0 {
        let x: f32 = -b / (2.0 * a);
        println!("The root of the equation is: {}", x);
    } else {
        let sqrt_d = d.sqrt();
        let x1: f32 = (-b + sqrt_d) / (2.0 * a);
        let x2: f32 = (-b - sqrt_d) / (2.0 * a);
        println!("The roots of this equation are: {} and {}", x1, x2);
    }
}