
// & accepts a string slice.
fn print_length(s: &str) {
    println!("The length of '{}' is {}",s, s.len());
}

fn update_length(s: &mut String) {
    s.push_str(" World!");
}

pub fn example() {
    // immutable_references();
    // mutable_references();
    //increments();
    // references();
    // dangling_references();
}

// rules: x will be dropped before it got returned back, we drop the x value.
// pub fn dangling_references() -> &String {
//     let x = "hello world!".to_string();
//     &x
// }

pub fn references() {
    let mut x = 42;
    let y = &mut x;

    *y += 1;

    // rule: we can not have mutable and immutable reference to the same value at the sometimes.
    // fix: not a good one but we can scop the y in to a scope.
    /*
        {
            *y += 1;
            println!("y = {}", y);
        }
     */
    // println!("x = {}", x); 
    println!("y = {}", y);
}

pub fn increments() {
    let mut x = 42;
    // let mut z = 55;
    let y = &mut x;

    *y+=1;
    // y = &mut z;

    // println!("The value of y is: {}", x);
    println!("The value of y is: {}", y);
}

// allows you to modify the data you are referring too.
pub fn mutable_references() {
    let mut x = "Hello".to_string();
    // let y = &mut x;
    //y.push_str(" World!");
    update_length(&mut x);
    
    println!("The value of x is:{}", x);
}

// very performative and easy on memory.
pub fn immutable_references() {
    let x = String::from("Hello World!");
    let y = &x;
    let z = &x[..]; // changed it to use string slice. or [0..5] if i went first 5 bits.

    println!("x is {}", x);
    println!("y is {}", y);
    println!("*y is {}", *y); // explicit referencing to memory.
    println!("z is {}", z);

    print_length(&x);
    print_length(y);
    print_length(&y);
}



 