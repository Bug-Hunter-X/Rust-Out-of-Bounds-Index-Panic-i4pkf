fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let index = 10;

    // This will panic at runtime because the index is out of bounds
    let number = numbers[index];
    println!("The number at index {} is {}", index, number);
}