fn main() {
    let mut nums = vec![1; 10]; // Initialize a vector with 10 elements, each containing the value 1

    for i in 1..=15 {
        for num in &mut nums {
            *num *= i;
        }
    }

    println!("Resulting vector: {:?}", nums);
}
