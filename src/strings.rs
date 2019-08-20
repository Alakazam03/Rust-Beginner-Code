// // str : immutable fixed-length string somewhere in memory
// //  String : when you want to own the data and modify it

// pub fn run(){
//     let _hello = "Hello ";
//     let  hello_string = String::from("Hello ");
//     println!("hello_string : {}", hello_string.len());

//     hello_string.push('W');
//     println!("hello_string : {}", hello_string);
//     hello_string.push_str("orld!");
//     println!("hello_string : {}", hello_string);

//     // capactiy in bytes
//     println!("capacity: {}", hello_string.capacity());

//     println!("{} is empty", hello_string.is_empty());

//     // contains
//     println!("contains world: {}", hello_string.contains("World"));
    
//     // contains
//     println!("Replace {}", hello_string.replace("World", "There"));

//     // loop through whitespaces
//     for word in hello_string.split_whitespace() {
//         println!("{}", word);
//     }

//     // create string with capacity
//     let mut s = String::with_capacity(10);
//     s.push('a');
//     s.push('b');
//     // Assertion testing
//     assert_eq!(2, s.len());
//     assert_eq!(10, s.capacity());
// }

pub fn run() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    println!("{}", s);


    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} 