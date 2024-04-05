fn main() {
    let data = vec![2,4];

    let result = complex_computation(&data);

    println!("Result of complex computation: {}", result);
}

fn complex_computation(data: &[i32]) -> f64 {
    let intermediate_result = data.iter().map(|&x| (x * x) -10).collect::<Vec<i32>>();

    let divisor = intermediate_result.iter().sum::<i32>();
    let dividend = data.iter().sum::<i32>();

    dividend as f64 / divisor as f64  // Potential division by zero if divisor == 0
}
