fn main() {
    let mut person = ("Alice", 30, "Engineer");
    println!("Person: {:?}", person);
    let (name, age, profession) = person;
    println!("Name: {}, Age: {}, Profession: {}", name, age, profession);

    let person2: (&str, i32, bool) = ("Bob", 25, true);
    println!("Person 2: {:?}", person2);
    let (name2, age2, is_employed) = person2;
    println!("Name: {}, Age: {}, Employed: {}", name2, age2, is_employed);

    println!("Person 0: {}", person.0);

    person.0 = "Charlie";
    println!("Person 0: {}", person.0);

}
