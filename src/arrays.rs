use std::mem;
pub fn run() {
    // Fixed length
    let mut numbers: [i32; 4] = [1, 1, 3, 4];

    // Reassign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Using imports with "use" keyword
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice from array. Get from 0 until 2 value of the array
    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);
}
