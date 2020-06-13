// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    // Types must be declared
    let person: (&str, &str, i8) = ("Dayus", "Madrid", 32);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}