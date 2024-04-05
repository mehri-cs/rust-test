fn main() {
    struct Complex {
        real: f64,
        imaginary: f64,
    }

    fn complex_multiply(a: &Complex, b: &Complex) -> Complex {
        let real_part = a.real * b.real - a.imaginary * b.imaginary;
        let imaginary_part = a.real * b.imaginary + a.imaginary * b.real;
        Complex {
            real: real_part,
            imaginary: imaginary_part,
        }
    }

    let mut complex_vector: Vec<Complex> = Vec::new();

    for i in 0..100 {
        complex_vector.push(Complex {
            real: i as f64,
            imaginary: (i + 1) as f64,
        });
    }

    // Attempt to perform complex multiplication on elements outside the bounds of the vector
    let index_a = 101;
    let index_b = 50;
    let result = complex_multiply(&complex_vector[index_a], &complex_vector[index_b]);

    println!("Result of complex multiplication: {} + {}i", result.real, result.imaginary);
}
