// Structs - Used to create custom data types

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(), // cast to String
            last_name: last.to_string(),
        }
    }

    // Self is this struct
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

// Tuple struct
struct ColorTuple(u8, u8, u8);

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c_tuple = ColorTuple(255, 0, 0);

    c_tuple.0 = 200;

    println!("Color: {} {} {}", c_tuple.0, c_tuple.1, c_tuple.2);

    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    
    println!("Person {}", p.full_name());

    p.set_last_name("Williams");
    println!("Person {}", p.full_name());

    println!("Person {:?}", p.to_tuple());
}