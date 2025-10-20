#[allow(unused_variables)] // To suppress warnings for unused variables
#[allow(unused_assignments)] // To suppress warnings for unused mutable variables

fn main() {
    //let pi: f32 = 4; // This will cause an error because 4 is an integer literal
    let pi: f32 = 4.0; // Correctly assign a floating-point value
    println!("Value of pi: {}", pi);

    let millions: i32 = 1_000_000; // Using underscores for readability
    println!("Millions: {}", millions);

    let is_day = true; // Boolean variable
    let is_night = false; // Another Boolean variable
    println!("Is it day? {}", is_day);
    println!("Is it night? {}", is_night);

    let char1 = 'a'; // Character variable
    let char2 = '😊'; // Unicode character '\u{1F60A}'
    println!("char1: {}, char2: {}", char1, char2);

}
