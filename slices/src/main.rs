fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4]; // This creates a slice containing elements 2, 3, and 4
    println!("Slice: {:?}", slice);

    let mut colors = ["red", "green", "blue", "yellow"];
    println!("{:?}", colors);

    update_colors(&mut colors[2..4]); // This will update "blue" and "yellow"
    println!("{:?}", colors);
}

fn update_colors(colors_slice: &mut [&str]) {
    colors_slice[0] = "cyan";
    colors_slice[1] = "magenta";
}