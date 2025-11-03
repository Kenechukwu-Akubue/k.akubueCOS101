use std::io;
fn main() {
    println!("Code         Item                          Price(N)");
    println!("P            Poundo Yam/Edinkaiko Soup     3,200");
    println!("F            Fried Rice & Chicken          3,000");
    println!("A            Amala & Ewedu Soup            2,500");
    println!("E            Eba & Egusi Soup              2,000");
    println!("W            White Rice & Stew             2,500");

    println!("");
    let mut code = String::new();
    println!("From the items listed above, enter the desired code: ");
    io::stdin().read_line(&mut code).expect("Failed to read input!");
    let code = code.trim().to_uppercase();

    let mut qty = String::new();
    println!("Enter the quantity: ");
    io::stdin().read_line(&mut qty).expect("Failed to read input!");
    let qty:u32 = qty.trim().parse().expect("Invalid input (must be numerical)!");

    if code == "P" {
        let total = 3200 * qty;
        println!("The total amount is: N{}", total);
            if total > 10_000 {
            let dis = total as f32 * 0.95;
            println!("You are eligible for a 5% discount! Your original price was N{}. Your discounted price is N{}", total, dis);
        }

    } else if code == "F" {
        let total = 3000 * qty;
        println!("The total amount is: N{}", total);
        if total > 10_000 {
            let dis = total as f32 * 0.95;
            println!("You are eligible for a 5% discount! Your original price was N{}. Your discounted price is N{}", total, dis);
        }

    } else if code == "A" {
        let total = 2500 * qty;
        println!("The total amount is: N{}", total);
        if total > 10_000 {
            let dis = total as f32 * 0.95;
            println!("You are eligible for a 5% discount! Your original price was N{}. Your discounted price is N{}", total, dis);
        }

    } else if code == "E" {
        let total = 2000 * qty;
        println!("The total amount is: N{}", total);
        if total > 10_000 {
            let dis = total as f32 * 0.95;
            println!("You are eligible for a 5% discount! Your original price was N{}. Your discounted price is N{}", total, dis);
        }

    } else if code == "W" {
        let total = 2500 * qty;
        println!("The total amount is: N{}", total);
        if total > 10_000 {
            let dis = total as f32 * 0.95;
            println!("You are eligible for a 5% discount! Your original price was N{}. Your discounted price is N{}", total, dis);
        }

    } else {
        println!("Invalid code entered!");
    }
}
