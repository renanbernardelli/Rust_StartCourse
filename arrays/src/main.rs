fn main() {
    let primes = [2, 3, 5, 7, 11, 13];
    println!("{:?}", primes);

    let doubles: [f64; 4] = [1.1, 2.2, 3.3, 4.4];
    println!("{:?}", doubles);

    let numbers = [0; 10];
    println!("{:?}", numbers);

    const DEFAULT: i32 = 3;
    let numbers = [DEFAULT; 5];
    println!("{:?}", numbers);

    println!("First prime: {}", primes[0]);
    // primes[0] = 17; // This line will cause a compile-time error because `primes` is immutable

    let mut primes = [2, 3, 5, 7, 11, 13];
    primes[0] = 17; // Now this is valid because `primes` is mutable
    println!("Updated first prime: {}", primes[0]);

    for number in primes.iter() {
        println!("Prime: {}", number);
    }
}