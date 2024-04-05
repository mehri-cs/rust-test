fn main() {
    let mut nums = vec![1; 10]; // Initialize a vector with 10 elements, each containing the value 1
    
    // Perform a series of additions that can potentially lead to integer overflow
    for i in 1..=15 {
        // Multiply each element in the vector by the iteration counter
        for num in &mut nums {
            *num *= i;
        }
    }
    
    // Print the resulting vector
    println!("Resulting vector: {:?}", nums);
}
