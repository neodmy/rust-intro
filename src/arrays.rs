// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run(){
    // Type and length. Must have the same number of values and types than declaration
    // let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // mut. Can't add but can change values
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single value
    println!("Sigle value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

}