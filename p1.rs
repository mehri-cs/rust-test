use std::mem;

fn main() {
    struct ComplexData {
        a: u32,
        b: u32,
        c: u32,
    }

    let mut data = ComplexData { a: 100, b: 200, c: 300 };

    println!("Initial values: a={}, b={}, c={}", data.a, data.b, data.c);

    fn perform_overflow_calculation(data: &mut ComplexData) {
        data.a *= 10;  // Potential overflow if data.a is close to its maximum value
        data.b += 1000;  // Potential overflow if data.b is close to its maximum value
        data.c -= 2000;  // Potential underflow if data.c is close to its minimum value
    }

    perform_overflow_calculation(&mut data);

    println!("Updated values after potential overflow: a={}, b={}, c={}", data.a, data.b, data.c);

    println!("Size of ComplexData struct: {} bytes", mem::size_of::<ComplexData>());
}