fn main() {
    let v = vec![101,250,330,400];
    
    let v2 = v;

    display(&v2);
    //v2 will be invalidated if & not used

    println!("In main: {:?}", v2);

}

fn display(v: &Vec<i32>) {
    println!("Inside display: {:?}", v);
}