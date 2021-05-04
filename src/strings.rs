pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hello.push('W');

    // Push string
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if str is empty
    println!("Is empty: {}", hello.is_empty());

    // Contains
    println!("Check if contains 'World' {}", hello.contains("World"));

    // Replace
    let new_world = hello.replace("World", "There");
    println!("Replace: {}", new_world);
    hello = new_world;

    // Split by whitespace
    let arr_hello = hello.split_whitespace();
    for word in arr_hello {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    // assert_eq!(3, s.len());
    // assert_eq!(11, s.capacity());
    println!("{}", s);

    println!("{}", hello);
}
