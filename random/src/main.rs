use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

fn main() {
    let mut rng = rand::thread_rng();
    let i: i32 = rng.gen();
    println!("Random: {}", i);

    println!("bounded int: {}", rng.gen_range(0, 100));
    println!("bounded float: {}", rng.gen_range(0.0, 100.0));
    
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(256)
        .collect();
    println!("Random string: {}", rand_string);
}
