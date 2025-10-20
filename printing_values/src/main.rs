fn main() {
    println!("Hello, world!");
    println!("My name is {} and I am {} years old.", "Alice", 30);
    print!("a + b = {}\n", 5 + 10);
    println!("{0} has a {2} and {0} has a {1}", "Bob", "car", "bike");
    println!("{name} {surname} is learning Rust!", name = "Charlie", surname = "Brown");
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 50, 50, 50);
    println!("array: {:?}", [1, 2, 3, 4, 5]);
    println!("float: {:.2}, scientific: {:e}", 3.14159, 3.14159);
    println!("right aligned: {:>10}, left aligned: {:<10}", "foo", "bar");
    println!("zero padded: {:0>5}", 423);
    println!("percentage: {:.2}%", 99.4567);
}
