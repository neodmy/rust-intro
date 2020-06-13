pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Integers can be printed directly
    println!("Number: {}", 1);

    // Basic formatting
    println!("{} is from {}", "Dayus", "Madrid");

    // Positional Arguments: with indexes
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Dayus", "Madrid", "code"
    );

    // Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    );

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
