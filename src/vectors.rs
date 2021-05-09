// Vectors - resizable arrays

use std::mem;
pub fn run() {
    // Fixed length
    let mut numbers: Vec<i32> = vec![1, 1, 3, 4];

    // Reassign value
    numbers[2] = 20;
    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Vector length: {}", numbers.len());

    // Vector are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Using imports with "use" keyword
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice from array. Get from 0 until 2 value of the array
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
