// Functions
// Entry point
// any functions/variables should be written in snake case
fn main() {
    hello_world();
    tell_age(23);
}

// hoisting (function defined below but used above)
fn hello_world() {
    println!("Hello, world!");
}

// you can insert input values into functions
fn tell_age(age: i32) {
    println!("I am {} years old.", age);
}
