pub fn run() {
    // let mut count = 0;

    // Infinite loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // While loop (FizzBuzz)
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("fizzBuzz");
    //     } else if count % 3 == 0 {
    //         println!("fizz");
    //     } else if count % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", count);
    //     }

    //     // Inc
    //     count += 1;
    // }

    // For Range loop
    for x in 0..21 {
        if x % 15 == 0 {
            println!("fizzBuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }
}
