// As you know variables are immutable (constant) by default.

fn main() {
    let x = 4; // a constant

    println!("Hello, world!");

    // x = 6; // this will give an error as x is immutable(constant)
    println!("The value of x is: {x}");

    // But when we make a variable mut, then its value can be changed
    let mut a = 5;
    println!("The value of a is: {a}");

    a = 56;

    println!("The value of a is now changed: {a}");
}
