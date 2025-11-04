fn main() {
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    println!("{:?}", vector);

    let second_vector = vec![4, 5, 6];
    println!("{:?}", second_vector);

    vector.remove(1);
    println!("{:?}", vector);

    let mut numbers = vec![2; 5];
    println!("{:?}", numbers);

    const DEFAULT: bool = false;
    let default_vector = vec![DEFAULT; 3];
    println!("{:?}", default_vector);

    numbers[0] = 10;
    println!("{:?}", numbers);

    for number in numbers.iter() {
        println!("{}", number);
    }
}
