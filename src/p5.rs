fn main() {
    let mut data = Box::new(42);

    // Creating a raw pointer from Box data
    let raw_ptr = &mut *data as *mut i32;

    // Dereferencing the raw pointer after dropping Box data
    unsafe {
        drop(data);
        let value = *raw_ptr; // Accessing invalidated memory
        println!("Value: {}", value);
    }
}