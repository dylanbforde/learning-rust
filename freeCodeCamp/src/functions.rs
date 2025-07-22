// Functions
// Entry point
// any functions/variables should be written in snake case
fn main() {
    hello_world();
    tell_age(23);
    human_id("Dylan", 23, 183.0);
    let x = {
        let price = 5;
        let qty = 10;
        price * qty
    };

    println!("Result is: {x}");

    let y = add(4, 6);
    println!("Value of y is {y}");
    println!("value from function add is {}.", add(4, 6));

    let weight: f64 = 70.0;
    let height: f64 = 1.83;
    let bmi: f64 = calculate_bmi(weight, height);
    println!("Your BMI is {bmi:.2}");
}

// hoisting (function defined below but used above)
fn hello_world() {
    println!("Hello, world!");
}

fn tell_age(age: u32) {
    println!("My age is {age}");
}

fn human_id(name: &str, age: u32, height: f32) {
    print!("My name is {name}, I am {age} years old, and my height is {height} cm");
}

// you can insert input values into functions fn tell_age(age: i32) { println!("I am {age} years old."); } fn human_id(name: &str, age: u32, height: f32) { println!("My name is {name}, I am {age} years old and my height is {height} cm."); } // Expressions and Statements Expression is anything that returns a value
// Statement is anything that does not return a value

// Expression
// 5
// true & false
// add(3,4)
// if condition {value} else {value_2}
// ({code})

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Statements
// Almsot all Statements in rust end with ;
// Variable declarations
// Function definitions
// Control flow Statements: if condition {code} else {code}
// BMI = height(kg)/height(m) ^ 2
fn calculate_bmi(weight_kilograms: f64, height_m: f64) -> f64 {
    weight_kilograms / (height_m * height_m)
}
