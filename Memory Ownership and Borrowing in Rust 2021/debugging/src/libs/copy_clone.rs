
// y = x.copy() since copy trate is implemented i can just do y = x; the x will copy into y.

// if the copy trate is not implemented the owner ship of the value is moved. 

pub fn example() {
    //scale_copy_trait_implemnted();    
    copy_trait_demo();  
}

pub fn scale_copy_trait_implemnted() {
    let x = 42;
    let mut y = x;

    println!("Before, x is {}", x);
    println!("Before, y is {}", y);

    y = 43;

    println!("Before, x is {}", x);
    println!("Before, y is {}", y);        
}

/*
    implmenting copy trate in to own strcuts.
*/

#[derive(Debug)] // removed , Copy becouse i added string(scaler type)
struct Person {
    id: u8,
    age: u8,
    name: String,
}

impl Clone for Person {
    fn clone(&self) -> Self {
        
        println!("Copied from this old value {:?}", self);

        Person {
            id: self.id,
            age: self.age,
            name: self.name.clone(),
        }
    }
}

pub fn copy_trait_demo() {
    let p1 = Person{
        id: 1,
        age: 20,
        name: "person1".to_string()
    };

    // this will just move the ownership did not made a copy or deep copy
    
    // clone trait is always required if we derived copy trait. clone is super trait for copy, it always required.
    
    // fix is that we need to derive the person strcut to have a copy, since clone is the super set we always have to dervice clone. Person struct is made up of Clone trait.

    let mut p2 = p1.clone();
    p2.id = 2;
    p2.age = 30;
    p2.name = "person2".to_string();


    println!("{:?}",p1);
    println!("{:?}",p2);
}

