// Keep in mind that Rust is statically typed language
// which means it must know the type of variables at compile time

// Scalar Types :
// Primary four : integers, floating-point, numbers, Booleans, and characters

// Integer types
// 8 bit signed integer : i8
// 8 bit unsigned integer : u8
// 32 bit signed integer : i32
// 32 bit unsigned integer : u32 
// upto i128 exist in rust.
// Additionally isize and usize types depend on the architecture of the computer
// your program is running on, which is denoted as "arch": 64 bits if you're using
// on a 64-bit architecture and 32 if you are using a 32-bit architecture.

/* You can write integer literals in any of the forms :  */
const A_CONSTANT: i32 = 57i32;

// Floating point types
// f32: single precision float
// f64: double precision float

// Numeric Operations
// All sum , difference , division works here.
// Integer division truncates toward zero to the nearest integer.

// Boolean type

// Character type

fn main() {
    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32

    // let t = true;
    // let f: bool = false; // with explicit type annotation

    // let c = 'x';
    // let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("The value of A_CONSTANT is: {A_CONSTANT}");
    println!("Hi There! I am a cat{heart_eyed_cat}");

}

