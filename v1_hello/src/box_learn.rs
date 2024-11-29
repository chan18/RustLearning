

pub fn learn() {
    /*

        Box<T> to point to Data on heap.

        smart pointer is a box.

        whoes type is written Box<T> 

        Box allow you to store data on the heap rather than the stack.

        Box dont have performance overhead.    

        
     */
    let b = Box::new(5);

    println!("b = {}", b);
}

