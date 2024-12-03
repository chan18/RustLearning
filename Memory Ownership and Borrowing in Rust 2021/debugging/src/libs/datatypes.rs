#![allow(unused_variables)]

pub fn datatype() {
    let x:i32 = 5;

    let y = 5;

    let c = 'z';

    // mutability 
    let mut a: i32 = 5;
    println!("Value of varable [a] is [{}]", a);

    a = 6;
    println!("New value of varable [a] is [{}]", a);

    // scope
    let c = 5;
    {
        let n = 5;

        // n is valid in this scope
        println!("{}",n);
        println!("{}",c);
    }
    // n is not valid in this scope
    println!("{}",c);

    function(x,y);
}

fn function(a: i32, b: i32) -> i32 {
    a + b
}

