// group of values
// max 12 elements


pub fn run(){

    let person: (&str, &str, i8) = ("vaibhav", "karnal", 23);
    println!("{} is from {} and is of age {}", person.0, person.1, person.2);
}