// Functions
// Entry point
// any functions/variables should be written in snake case
fn main() {
    hello_world();
    tell_age(23);
    human_id("Dylan", 23, 183.0);
}

// hoisting (function defined below but used above)
fn hello_world() {
    println!("Hello, world!");
}

// you can insert input values into functions
fn tell_age(age: i32) {
    println!("I am {age} years old.");
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {name}, I am {age} years old and my height is {height} cm.");
}
