use std::io;

/// .
///
/// # Panics
///
/// Panics if .
pub fn data_types() {

    // help: if this is intentional, prefix it with an underscore: so added _ before unused vars.
     /*
        scalar types
        integer types
        floating point types
     */
    let _x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition 
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3;

    // remainder
    let _remainder = 43 % 5;

    let _t = true;

    let _f: bool = false; // with explicit type annotation.

    let _c = 'z';

    let _z: char = 'Z'; // with explicit type annotation.

    let _heart_eyed_cat = '😻';

    // compound types

    // tuple type

    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let _tup = (500, 6.4, 1);

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;

    // the array type
    let _a = [1, 2, 3, 4, 5];

    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // array a with 5 elements with all values set to 3.
    let _a = [3; 5];

    // accessing array elemetns.
    let a = [1,2,3,4,5];

    let _first = a[0];
    let _second = a[1];

    let a = [1,2,3,4,5];

    println!("Please enter an array index.");

    // mutable index value.
    let mut index = String::new();

    // read index from stdin.
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    // validate if index can be parsed into a number if no throw error.
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    //println!("Hello, world!");
}

