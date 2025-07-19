// Compound Data Types
// arrays, tuples, slices, and strings

fn main() {
    // Array Example
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:#?}", numbers);

    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("Fruits Array: {:#?}", fruits);
    println!("First Fruit: {}", fruits[0]);
    println!("Second Fruit: {}", fruits[1]);
    println!("Third Fruit: {}", fruits[2]);

    // Tuple Example
    // .to_string() converts &str to String
    let human: (String, i32, bool) = ("Dylan".to_string(), 23, false);
    println!("Human Tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", "23", true, [1, 2, 3, 4, 5]);
    println!("My mix tuple: {:?}", my_mix_tuple);

    // Slices: [1, 2, 3, 4, 5] (continuous part of an array)
    let number_slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice: {:?}", number_slice);

    let animal_slice: &[&str] = &["cat", "dog", "fish"];
    println!("Animal Slice: {:?}", animal_slice);

    let book_slice: &[&String] = &[
        &"IT".to_string(),
        &"The Shining".to_string(),
        &"Misery".to_string(),
    ];
    println!("Book Slice: {:?}", book_slice);

    // Strings vs String Slices (&str)
    // String [growable, mutable, owned string type]
    // String heap allocated, can change size
    let mut stone_cold: String = String::from("Hell");
    stone_cold.push_str(" Yeah!");
    println!("Stone Cold String: {}", stone_cold);

    // B- &str (string slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("String Slice: {}", slice);
}
