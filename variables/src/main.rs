#[allow(unused_variables)] // To suppress warnings for unused variables
#[allow(unused_assignments)] // To suppress warnings for unused mutable variables

fn main() {
    let name = "Alice";
    let age = 30;
    println!("Name: {}, Age: {}", name, age);

    //let number = 123456789123456789; causes an error
    let number: i64 = 123456789123456789; // Specify type to avoid error
    println!("{}", number);

    // age = 31; // This will cause an error because `age` is immutable by default
    
    let mut age_new = 40; // Make `age` mutable
    age_new = 41; // Now this is allowed
    println!("Updated Age: {}", age_new);

    // Alow shadowing
    let age = "forty"; // Shadowing `age` with a new type
    println!("Shadowed Age: {}", age);

    let (x, y, z) = (1, 2, "red"); // Destructuring assignment
    println!("x: {}, y: {}, z: {}", x, y, z);
}
