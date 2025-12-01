fn main() {
    let v = vec![15, 20, 35, 45, 55];
    print_vector(v);
    println!("{}",v[0])
}

fn print_vector(v: Vec<i32>) {
    println!("Inside Print vector function: {:?}", v);
}