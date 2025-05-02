// Primitive data types
// int, float, bool, char

// Integer
// Rust has signed (+ and -) and unsigned integer (only +)
// types of different sizes.
// i8, i16, i32, i64, i128: Signed integers
// u8, u16, u32, u64, u128: Unsigned integers

fn main(){
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // diff between i32 (32 bits) and i64 (64 bits)
    // range :
    // i32 - 2^31
    // i32 - 2^63

    let e: i32 = 2147483647; // if 1 is added this fails to compile
    let i: i64 = 9223372036854775807; // if 1 is added this fails to compile
    println!("Maximum value of i32: {}", e);
    println!("Maximum vlaue of i64: {}", i);


    // Floats
    // f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi {}", pi);

    // Boolean Values
    let is_bool: bool = true;
    println!("is this true {}", is_bool);

    // Character type
    let letter: char = 'a';
    println!("first letter of the alphabet: {}", letter);


}