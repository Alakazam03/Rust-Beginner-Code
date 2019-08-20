pub fn run() {
    // print to console
    println!("hello from print.rs file");

    // placeholders {} always needed for variable
    println!("{} is number", 1);

    println!("{} is from {}", "Vaibhav", "Karnal");

    // advanced positional arguements
    println!("{0} is from {1} and {0} likes to {2}", "Vaibhav", "Karnal", "code");

    // named args
    println!(
        "{name} likes to play {sports}",
         name = "vaibhav", 
         sports = "skaitng"
    );

    // placeholder trauits
    println!("binary: {:b} Hex: {:x} octal: {:o}", 10, 10, 10);

    // ebug traits
    println!("{:?}", (12, 10, "trial"));
}