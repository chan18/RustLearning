

pub fn ownership() {
    /*
        rust to make memeory safe guarantees without needing a garbage collector.

        ownership
        borrowing
        slices

        how rust lays data out in memeory.
     */

    // ownership is set of rules how rust manages memory.

    /*
        gc 
        manaual memeory
        3rd approch with ownership set of rules that compiler checks.

        stack vs heap
        
        allocating on heap.

        the main purpose of ownership is to manage heap data.
     */

    /*
        rules of ownership.

        1. each value in rust has an owner.
        2. there can only be one owner at a time.
        3. when the owner goes out of scop, the value will be dropped. 


     */

    // vairalbe scope

    { // s is not valid here, it's not yet declared
        let s = "hello"; // s is valid from this point forward
        // do stuff with s.
    } // this scop is now over, and s is no longer valid.


    // name convention : method-syntax and modules.
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a string.

    println!("{}", s);


    // memory and allocation
    // rust drop
    // RAII
    // resource acquisation is initialization
    

    // variable and data interaction with move

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

    /* string
        pointer -> memeory
        length
        capacity

        memeory = 0X123

        s1 ->  -> memeory
        s2 -> memeory

        // double free error
        // freeing memeory twice could lead to memeory corruption
        // which can potentially lead to security vulnerabilites. 

        once after the line let s1 = s2
        rust will no longer make s1 as valid. ( to ensure memeory safety )

        shallow copy vs deep copy

        copying the pointer, lenth, capacity without copying the data probably sounds like 
        making a shallow copy.

        but rust also makes the first variable invalid. insted of being called a shallow copy.
        is's known as move.

        s1 was moved into s2

        rust will never automatically create "deep" copies of your data.

        automatic copying expensive interms of runtime performance.

     */

    let s2 = s1.clone(); // could be expensive.
    println!("s1 = {}, s2 = {}", s1, s2);

    // stack only data copy.

    /*
        the value y is not moved. 

        becouse the integer values are known in compile time.
        copies of the actual values are quick to make.


     */
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x,y);

    // ownership and functions

    let s = String::from("hello"); // s comes into scope

    // s's value moves into the fuction  and so is no longer valid here
    takes_ownership(s); 

    // borrow of moved value: 's'.
    // println!("trying to use the value s to understsand ownership concpet here is the s:{}", s);



    // x comes into scope.
    let x = 5;

    // x would move into the function.
    // but i 32 is copy, so it's ok to still
    // use x afterward.
    makes_copy(x);

    println!("x: {}",x);

    // return values and scope

    // gives_onwership moves its return
    // values into s1_data
    let s1_data = gives_ownership(); 

    // s2_data into scope
    let s2_data = String::from("hello");

    // s2_data is moved into takes_and_gives_back.
    // which also moves its return value into s3    
    let s3_data = takes_and_gives_back(s2_data);

    // what if we want function to use the value but not take ownership

    let s1 = String::from("hello");

    let (s2, len) = calculate_lenth(s1);

    println!("The lenth of '{}' is {}.", s2, len);

}
// here, x goes out of scope, then s, but because s's value was into scope
// then s. but becosue s's value was moved, nothing special happens.

// here, s3_data goes out of the scope and dropped.
// s2_data was moved. so nothing happens. so s1_data goes out of scope and is dropped.


// some_string comes into scope
fn takes_ownership(some_string: String) { 
    println!("{}", some_string);
}
// here, some_string goes out of scope and `drop` is called. the backing
// memeory is freed.


fn makes_copy(some_integer: i32) { // some_integer comes into scope 
    println!("{}", some_integer);
} // here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string  = String::from("Your's");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

/*
    ownership of a varabile follows the same pattern every time.

    assigning a value to another variable moves it. 

    when a variable that includes data on the heap goes out of scope.

    than the value of data will be cleaned up by drop unless onwership of the data has been moved to another variable.    
*/

// rust has a feature for using a value without transferring ownership, called references.
fn calculate_lenth(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

