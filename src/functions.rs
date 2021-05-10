pub fn run() {
    gretting("Hello", "Jane");

    // Bind function values to variables
    let sum: i32 = add(20, 20);
    println!("Sum: {}", sum);

    // Closure
    const N3: i32 = 10;
    const N4: i32 = 5;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + N3 - N4;
    println!("Closure Sum: {}", add_nums(3, 3));
}

fn gretting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
