pub fn run() {
    // Define variables
    let name = "Renatto";

    // The only way to make mutable a variable is with "mut" keyword
    let mut age = 25;

    println!("My name is {} and I am {}", name, age);
    age = 26;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 002;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Renatto", 25);
    println!("{} is {}", my_name, my_age);
}
