pub fn run() {
    // Int default is i32
    let x = 12;

    // Float default is f64
    let y = 5.3;

    // Add explicit type
    let z: i64 = 565656565656;
    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 5;

    // Characters
    let a1 = 'a';
    let emoji = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, emoji));
}
