// Variables hold primitive data or references to data
// Variables are inmutable by default (can't reassign them)
// Rust is a block-scoped language

pub fn run() {
    let name = "Dayus";

    let mut age = 32;
    println!("My name is {} and I am {}", name, age);

    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Define constant. Uppercase and Type must be provided
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Dayus", 32);
    println!("{} is {}", my_name, my_age);

    
}