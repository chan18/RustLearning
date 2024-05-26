
pub fn functions() {
    // println!("Hello world");
    another_function();

    print_labeled_measurement(5, 'h');

    // statements and expressions
    let _y  = 6;

    // let y = 6 does not return a value.
    // let x = (let y = 6);
    

    // expression.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // functions and return values.
    let x = five();

    println!("The value of x is: {x}");

    let x1 = plus_one(5);

    println!("The value of x1 is: {x1}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five()-> i32 {
    5
}

fn another_function() {
    println!("Another function.");    
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


