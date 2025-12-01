fn main() {
    let v = vec![20, 40, 60, 80];

    let v2 = v;
    let v2_return = display(v2); //v2 will be invalidated if & not used
    println!("In main: {:?}", v); //cannot use v
}

fn display(v:Vec<i32>) {
    println!("Inside display: {:?}", v);
    return v
}
//code will flag errors (I did this on purpose)