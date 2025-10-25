fn main() {
    for i in 1..6 {
        say_hello();
    }

    let mut name = "Alice";
    say_hi(name);
    println!("{}", name);

    name_mutation(&mut name);
    println!("{}", name);

    let greeting = return_greeting(&mut name);
    println!("{}", greeting);
    //output directly
    println!("{}", return_greeting(&mut name));
}

fn say_hello() {
    println!("Hello from say_hello function!");
}

fn say_hi(name: &str) {
    println!("Hi, {}!", name);
}

// This function attempts to mutate the name by changing the reference
fn name_mutation(name: &mut &str) {
    println!("Before mutation: {}", name);
    *name = "Bob";
}

// This function returns a greeting string
fn return_greeting(name: &mut &str) -> String {
    let greeting = format!("Hello, {}!", name);
    return greeting;
}