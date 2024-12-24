fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let index = 10;

    // Use `get()` for safe index access. It returns an `Option`.
    match numbers.get(index) {
        Some(number) => println!("The number at index {} is {}", index, number),
        None => println!("Index {} is out of bounds", index),
    }
} 
