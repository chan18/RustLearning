use std::io; // io module
use rand::Rng; // Rng trait

use std::cmp::Ordering; // gring ordering into scoping from standard lib.

/*
    Note: after adding rand crate do a cargo build, to download and compile the dependencies.
 */
fn main() {
    println!("Guess the number!");

    // thread_rng() local to current thread execution & seeded by os.
    // gen_range() method is defined by the Rng trait method.
    //  range expressoin. start..=end
    // cargo doc --open
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");


    loop {
        println!("Please input your guess."); 

        // by default immutable 
        // once given a value the value own't change.
        // String:new a functio returns new instance of a string.
        // :: ::new - associated function of the String.
        // associated function is implemented on a type.
        // create a mutable variable that is curenttly bound to a new, empty instance of a String.
        let mut guess = String::new(); // mutable

        // std::io::stdin
        // & argument reference.
        // like variables references are mutable 
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // shadowing.
        // : annotation of guess with u32.
        // input validation swtich from expect call to match expression
        // result will have enum that has variants Ok and Err. 
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
        };

        println!("You guessed: {guess}");

        /*
            ordering type is another enum.
        */    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too bing!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }

    }


}



