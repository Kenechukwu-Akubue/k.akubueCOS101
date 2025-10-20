use std::io;

fn main() {
    let mut name = String::new();
    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Not a valid String");

    let mut input1 = String::new();
    println!("Enter your age: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:i32 = input1.trim().parse().expect("Not a valid number");

    let mut input2 = String::new();
    println!("Enter your experience level (i for inexperienced, e  for experienced): ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let xp_lvl = input2.trim();

    if xp_lvl == "i" {
        println!("Your incentive is N100,000 per month");
    }
    else if xp_lvl == "e" {
        if age < 28 {
            println!("Your incentive is N1,300,000 per month");
        }
        else if age <= 30 {
            println!("Your incentive is N1,400,000 per month");
        }
        else if age < 40 {
            println!("Your incentive is N1,480,000 per month");
        }
        else {
            println!("Your incentive is N1,560,000 per month");
        }
    }
    else {
        println!("Invalid input");
    }

}
