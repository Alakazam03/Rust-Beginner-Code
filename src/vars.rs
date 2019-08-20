pub fn run(){

    // variables hold primitive data or reference to data
    // immutable by default
    // block-scoped language

    let name = "Vaibhav";
    let age = 23;
    // age = 28; immutable variable
    let mut salary = 27000;
    println!("may name is {} and i earn {}", name, salary);
    salary = 28000;
    
    println!("may name is {} and i am {}", name, age);
    println!("may name is {} and i earn {}", name, salary);

    // define const
    const ID : i32 = 001;
    println!("ID: {}", ID);

    // assign multiple variables
    let (my_name, my_age) = ("Brad", 37);
    println!("my name {} and my age {}", my_name, my_age);
}