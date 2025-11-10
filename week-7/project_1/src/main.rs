use std::io;

fn trapezium(height: f64, base1: f64, base2: f64) {
    let area:f64 = 0.5 * height * (base1 + base2);
    println!("The area of the trapezium = {:.2}", area);
}

fn rhombus(diagonal1: f64, diagonal2: f64) {
    let area:f64 = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus = {:.2}", area);
}

fn parallelogram(base: f64, altitude: f64) {
    let area:f64 = base * altitude;
    println!("The area of the parallelogram = {:.2}", area);
}

fn cube(length:f64) {
        let area:f64 = 6.0 * length.powi(2);
        println!("The area of the cube  = {:.2}", area);
}

fn v_cylinder(radius:f64, height:f64) {
    let pi:f64 = 3.141592654;
    let volume = pi * height * radius.powi(2);
    println!("The volume of the cylinder = {:.2}", volume);
}

fn main() {
    println!("t -> area of trapezium");
    println!("r -> area of rhombus");
    println!("p -> area of parallelogram");
    println!("c -> area of cube");
    println!("C -> volume of cylinder");

    let mut choice = String::new();
    println!("Enter code: ");
    io::stdin().read_line(&mut choice).expect("FAILED TO READ INPUT!");
    let choice = choice.trim();
    

    if choice.to_lowercase() == "t" {
        let mut height = String::new();
        println!("Enter the height: ");
        io::stdin().read_line(&mut height).expect("FAILED TO READ INPUT!");
        let height:f64 = height.trim().parse().expect("VALUE NOT NUMERICAL!");

        let mut base1 = String::new();
        println!("Enter the length of the first base: ");
        io::stdin().read_line(&mut base1).expect("FAILED TO READ INPUT!");
        let base1:f64 = base1.trim().parse().expect("VALUE NOT NUMERICAL!");

        let mut base2 = String::new();
        println!("Enter the length of the second base: ");
        io::stdin().read_line(&mut base2).expect("FAILED TO READ INPUT!");
        let base2:f64 = base2.trim().parse().expect("VALUE NOT NUMERICAL!");

        if height <= 0.0 || base1 <= 0.0 || base2 <= 0.0 {
            println!("VALUES MUST BE POSITIVE AND GREATER THAN ZERO!");
        } else {
            trapezium(height, base1, base2);
        }
    } else if choice.to_lowercase() == "r" {
        let mut diag1 = String::new();
        println!("Enter the length of the first diagonal: ");
        io::stdin().read_line(&mut diag1).expect("FAILED TO READ INPUT!");
        let diag1:f64 = diag1.trim().parse().expect("VALUE NOT NUMERICAL!");

        let mut diag2 = String::new();
        println!("Enter the length of the second diagonal: ");
        io::stdin().read_line(&mut diag2).expect("FAILED TO READ INPUT!");
        let diag2:f64 = diag2.trim().parse().expect("VALUE NOT NUMERICAL!");

        if diag1 <= 0.0 || diag2 <= 0.0 {
            println!("VALUES MUST BE POSITIVE AND GREATER THAN ZERO!");
        } else {
            rhombus(diag1, diag2);
        }
    } else if choice.to_lowercase() == "p" {
        let mut base = String::new();
        println!("Enter the length of the base: ");
        io::stdin().read_line(&mut base).expect("FAILED TO READ INPUT!");
        let base:f64 = base.trim().parse().expect("VALUE NOT NUMERICAL!");

        let mut height = String::new();
        println!("Enter the height: ");
        io::stdin().read_line(&mut height).expect("FAILED TO READ INPUT!");
        let height:f64 = height.trim().parse().expect("VALUE NOT NUMERICAL!");

        if height <= 0.0 || base <= 0.0 {
            println!("VALUES MUST BE POSITIVE AND GREATER THAN ZERO!");
        } else {
            parallelogram(base, height);
        }
    } else if choice == "c" {
        let mut length = String::new();
        println!("Enter the length: ");
        io::stdin().read_line(&mut length).expect("FAILED TO READ INPUT!");
        let length:f64 = length.trim().parse().expect("VALUE NOT NUMERICAL!");

        if length <= 0.0 {
            println!("VALUES MUST BE POSITIVE AND GREATER THAN ZERO!");
        } else {
            cube(length);
        }
    } else if choice == "C" {
        let mut height = String::new();
        println!("Enter the height: ");
        io::stdin().read_line(&mut height).expect("FAILED TO READ INPUT!");
        let height:f64 = height.trim().parse().expect("VALUE NOT NUMERICAL!");

        let mut radius = String::new();
        println!("Enter the radius: ");
        io::stdin().read_line(&mut radius).expect("FAILED TO READ INPUT!");
        let radius:f64 = radius.trim().parse().expect("VALUE NOT NUMERICAL!");

        if height <= 0.0 || radius <= 0.0 {
            println!("VALUES MUST BE POSITIVE AND GREATER THAN ZERO!");
        } else {
            v_cylinder(radius, height);
        } 
    } else {
        println!("INVALID CODE PROVIDED!")
    }
}
