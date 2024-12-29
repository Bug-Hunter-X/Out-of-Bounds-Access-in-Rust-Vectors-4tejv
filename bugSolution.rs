fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    //Safe way to access vector elements
    if let Some(&value) = vec.get(2) {
        println!("The value at index 2 is: {}", value);
    } else {
        println!("Index out of bounds");
    }
    
    //Using a match expression for better error handling
    match vec.get(10) {
        Some(&value) => println!("The value at index 10 is: {}", value),
        None => println!("Index out of bounds"),
    }
}