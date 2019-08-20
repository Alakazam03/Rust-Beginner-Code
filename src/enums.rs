#![allow(dead_code)]

enum TransactionEvent{
    SUBMIT,
    PROCESS(char),
    COMPLETED(String),
}

enum Number{
    ZERO,
    ONE,
    TWO,
}

pub fn transaction_flow(){
    let complete_message = String::from("evaluation has been completed");
    let status_complete = TransactionEvent::COMPLETED(complete_message);
    let status_submit = TransactionEvent::SUBMIT;
    let _status_processing = TransactionEvent::PROCESS('s');

    println!("{}", Number::ZERO as i32);
    print_cycle(status_submit);
    print_cycle(status_complete);
}

fn print_cycle(event : TransactionEvent) {
    match event {
        TransactionEvent::SUBMIT => println!("Transaction has been submitted"),
        TransactionEvent::COMPLETED(s) => println!("completed '{}'", s),
        TransactionEvent::PROCESS(c) => println!("processing {}", c),
    }
}