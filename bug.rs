fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // This will cause a runtime error because we try to access an index
    // that is out of bounds.
    println!("The value at index 2 is: {}", vec[10]);
}