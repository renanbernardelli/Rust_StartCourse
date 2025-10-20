#[allow(unused_variables)] // To suppress warnings for unused variables
#[allow(unused_assignments)] // To suppress warnings for unused mutable variables

fn main() {
    let cat = "cat";
    let name = "Alice";

    // Explicit type annotations - string slices
    let dog: &str = "dog";
    let animal: &'static str = "animal"; // 'static lifetime

    println!("cat: {}, dog: {}, animal: {}", cat, dog, animal);

    // String objects
    let mut s = String::new(); // Create an empty String
    println!("Empty String: '{}'", s);
    s = String::from("hello"); // Create a String from a string literal
    println!("String from literal: '{}'", s);
    s = "hello".to_string(); // Convert a string literal to a String
    println!("Converted String: '{}'", s);

    let bear = String::new();
    let bear = String::from("bear");
    println!("bear: '{}'", bear);

    // Format macro to create a String
    let hello = format!("{}-{}, my name is {}", "hello", "world", name);
    println!("Formatted String: '{}'", hello);

    // Get the length of a String
    let len = hello.len();
    println!("Length of '{}': {}", hello, len);

    //Push and push_str methods
    let mut string1 = String::new();
    string1 = String::from("Renan");

    let mut string2 = String::new();
    string2 = String::from("Bernardelli");

    println!("Before push and push str: '{} {}", string1, string2);

    string1.push('Z'); // Push a single character
    string2.push_str(" Zauzar"); // Push a string slice
    println!("After push and push_str: '{} {}'", string1, string2);

    // Replace method
    let string3 = string2.replace("Zauzar", "Z"); // Replace substring
    println!("After replace: '{}'", string3);

    // Split method
    let string4 = String::from("apple,banana,cherry");
    let parts: Vec<&str> = string4.split(',').collect(); // Split by comma
    println!("After split: {:?}", parts);

    // Trim method
    let string5 = String::from("   Hello, World!   ");
    let trimmed = string5.trim(); // Trim whitespace
    println!("After trim: '{}'", trimmed);

    // split_whitespace method
    let string6 = String::from("Hello   World  from Rust");
    let words: Vec<&str> = string6.split_whitespace().collect(); // Split by whitespace
    println!("After split_whitespace: {:?}", words);

    // chars method
    let string7 = String::from("Rust");
    let chars: Vec<char> = string7.chars().collect(); // Collect characters
    println!("After chars: {:?}", chars);
}
