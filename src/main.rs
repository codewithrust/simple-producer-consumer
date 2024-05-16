use std::sync::mpsc;

mod producer;
mod consumer;

fn main() {
    proccess_sync();
    proccess_async();
}


fn proccess_sync() {
    // create a channel
    let (tx, rx) = mpsc::channel();

    // start the producer and consumer
    producer::produce(tx);
    consumer::consume(rx, "Sync");
}

fn proccess_async() {
    // create a channel
    let (tx, rx) = mpsc::channel();

    // start the producer and consumer
    producer::produce_async(tx);
    consumer::consume(rx, "Async");
}