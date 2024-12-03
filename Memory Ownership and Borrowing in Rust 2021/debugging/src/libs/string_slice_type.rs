pub fn example() {

    create_string();

}


// examples
fn create_string() {
    let mut x: String = String::new();
    x.push_str("This is a string using the new constructor");

    let y = "This is a string using the to_string function".to_string();
    let z = String::from("This is a string using the from function");

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("The value of z is {}", z);
}




