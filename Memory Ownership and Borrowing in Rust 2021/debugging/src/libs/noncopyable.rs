
/*
// non copy
pub fn noncopyable_data_types() {
    let x = String::from("42");
    let y = x;

    println!("{:?}",x);
    println!("{:?}",y);
}
*/

/*
// solution : clone
pub fn noncopyable_data_types() {
    let x = String::from("42");
    
    print_string(x.clone());
    print_string(x);
}

// function scops.
fn print_string(a: String) {
    println!("This is the value of my string {:?}",a);
}
 */

// better solution: use immutable value of x
pub fn noncopyable_data_types() {
    let x = String::from("42");
    
    print_string(&x);
    print_string(&x);
}

// function scops.
fn print_string(a: &String) {
    println!("This is the value of my string {:?}",a);
}


