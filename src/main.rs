// mod print;
// mod vars;
// mode types;
//mod strings;
// mod tuples;
// mod arrays;
//mod class;
//mod enums;
mod learn_functions;
fn main() {
    println!("Hello, world!");
    // print::run();
//    strings::run();
    // tuples::run();
    // arrays::run();
//    class::students();
//    class::rectangle(3.2, 4.1);
//    class::make_line();
//    enums::transaction_flow();
    learn_functions::line();
    let mut square = learn_functions::Rectangle {
        p1: learn_functions::Point::origin(),
        p2: learn_functions::Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
//    learn_functions::rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line

    // Okay! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);

}
