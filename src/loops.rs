pub fn run() {
    let mut count = 0;

    // Infitite lopp
    loop {
        count += 1;
        println!("Number: {}", count);

        // break it
        if count == 20 {
            break;
        }
    }

    let mut count_loop = 0;
    // While loop (FizzBuzz)
    while count_loop <= 100 {
        if count_loop % 15 == 0 {
            println!("fizzbuzz");
        } else if count_loop % 3 == 0 {
            println!("fizz");
        } else if count_loop % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count_loop);
        }

        count_loop += 1;
    }

    // For Range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }
}
