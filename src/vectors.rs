// Vectors - Resizable arrays

use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single value
    println!("Sigle value: {}", numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Add on to vector
    numbers.push(5);
    numbers.push(6);
    println!("{:?}", numbers);

    // Pop off last value
    numbers.pop();

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers vec {:?}", numbers);
}