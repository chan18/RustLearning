#![allow(unused_variables)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// life time annotation.
// annotate lifetiems using generics.
fn get_oldest<'a>(person1: &'a Person, person2: &'a Person) -> &'a Person {
    if person1.age > person2.age {
        person1
    } else {
        person2
    }
}

// fn get_oldest(person1: &Person, person2: &Person) -> &Person {
//     if person1.age > person2.age {
//         person1
//     } else {
//         person2
//     }
// }


pub fn examples() {
    //lifetime();
    //lifetime1();

    function_lifetime();
}

// lifetiems in functions

pub fn function_lifetime() {
    let p1 = Person {name: "John".to_string(), age: 20};
    let p2 = Person {name: "John".to_string(), age: 30};
    let p3 = get_oldest(&p1, &p2);

    // println!("p1: {:?}", p1);
    // println!("p2: {:?}", p2);

    println!("p3: {:?}", p3);
}

//---



fn lifetime1() {

    let x: &String;
    {
        let y = "Hello World".to_string();
        // x = &y; // moving ownership value from y to x.
        //x = &y; // dangling reference.
        // owener ship of value which already been droped.
    }

    // println!("{}", x);
}


fn lifetime() {

    let x;
    {
        let y = "Hello World".to_string();
        x = y; // moving ownership value from y to x.
        //x = &y; // dangling reference.
        // owener ship of value which already been droped.
    }

    println!("{}", x);
}