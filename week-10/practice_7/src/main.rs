fn main() {
    let emp1 = Employee {
        name: String::from("Tony Stark"),
        company: String::from("Stark Industries"),
        age: 50
    };

        println!("Name: {}\n", emp1.name);
        println!("Company: {}\n", emp1.company);
        println!("Age: {}", emp1.age);
}

struct Employee {
    name: String,
    company: String,
    age: u32
}
