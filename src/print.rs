pub fn run() {
    // Print to console
    println!("Hello from the print.rs file!");

    // Interpolation
    println!("{} was born on {}", "Rust", 2010);

    // Positonal arguments
    println!(
        "{2} was born on {1} and is a compiled language just like {0} that was born on {3}",
        "Go", 2010, "Rust", 2009
    );

    // Named arguments
    println!(
        "{name} likes to {activity}",
        name = "Renatto",
        activity = "code"
    );

    // Placeholder traits
    println!("Binary: {:b} - Hex: {:x} - Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("Debug: {:?}", (12, true, "Rust"));

    // Basic math
    println!("10 + 20 = {}", 10 + 20,);
}
