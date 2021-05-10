pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("{:?} - {:?}", arr1, arr2);
    // With non-primitives, if you assign another variable to the first variable.
    // The first variable will lost his value. You'll need to use a reference (&) to point
    // to the resource.

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("Values: {:?}", (&vec1, vec2));
}
