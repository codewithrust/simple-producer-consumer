use std::sync::mpsc::Receiver;

pub fn consume(rcv: Receiver<i32>, type_execution: &str) {
    for received in rcv {
        if received == 10 {
            println!("Done: {}", type_execution);
        }
    }
}