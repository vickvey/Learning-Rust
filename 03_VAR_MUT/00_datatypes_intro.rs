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

    // constants in RUST are always immutable
    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;
    println!("3 hours in seconds is : {THREE_HOURS_IN_SECONDS}");

    // Shadowing a variable
    // You can declare a new variable with the same name as a previous variable.
    // Rustaceans say first variable is shadowed by the second, which means that 
    // the second variable is what the compiler will see when you use the name of 
    // the variable.

    let a = a + 1;
    println!("The value of a is: {a}");

    // Differences between mut and shadowing
    let spaces = "    "; // spaces is string type
    let spaces = spaces.len(); // spaces is number type
    // Hence we can change the type of the value but use the same name

    // But this will give us an error:
    // let mut spaces = "   ";
    // spaces = spaces.len();
    println!("Spaces are taken input value is: {spaces}");
    
}
