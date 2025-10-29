//use crate::archive::arch::archive_file;
use crate::archive::arch::archive_file as arch;
use rand::Rng;

mod archive;

fn main() {
    //archive_file("document.txt");
    arch("document.txt");

    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen();
    println!("Random number: {}", n);
}
