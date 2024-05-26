
pub fn control_flow() {

    let number = 2;

    if number < 5 {
        println!("conditional was true");
    } else {
        println!("condition was false");
    }

    // let number = 7;
    let number = 3;

    if number != 0{
        println!("number was three");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    // using if in the let statments.
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");


    // repetition with looops.
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    // return values from loops.
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    

    // loop labels to disambiguate between multiple loops.
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");


    // conditional loops with while loops
    let mut number = 2;

    while number != 0 {
        println!("{number}");

        number -= 1;        
    }

    println!("LIFTOFF!!!");

    // looping through a collection with for
    let a = [10, 20, 30, 40 , 50];

    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");

}

