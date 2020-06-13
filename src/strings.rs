// Primitive str = Immutable fixed-length string in memory
// String = Growable, heap-allocated data structure - Use when you need to modify your own string data

pub fn run() {
    // Primitive string
    // let hello = "Hello";

    let mut hello = String::from("Hello ");

    println!("{}", hello);

    // Get length
    println!("Length: {}", hello.len());

    // Push for characters. String must be mutable
    hello.push('W');

    // Push for strings. String must be mutable
    hello.push_str("orld!");

    // Capacity: name of bits it can store
    println!("Capacity: {}", hello.capacity());

    // Is empty
    println!("Is empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing. Matches left with right. Only shows error if fails
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}