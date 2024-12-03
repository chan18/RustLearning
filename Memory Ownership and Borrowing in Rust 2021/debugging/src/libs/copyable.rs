

pub fn copyable_data_types() {
    let s1 = 1; // copy to stack
    let s2 = s1;
    println!("{:?}",s1);
    println!("{:?}",s2);
}

