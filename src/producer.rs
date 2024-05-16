use std::{sync::mpsc::Sender, thread, time::Instant};

pub fn produce(tx: Sender<i32>) {
    let start_time = Instant::now();
    let join_handle = thread::spawn(move || {
        for index in 1..10 {
            tx.send(index).unwrap();

        }
    });

    let _ = join_handle.join();

    let end_time = Instant::now();
    let execution_time = end_time - start_time;

    println!("Execution time Sync: {:?}", execution_time);
}

#[tokio::main]
pub async fn produce_async(tx: Sender<i32>) {
    let start_time = Instant::now();

    let handle = tokio::spawn(async move {
        for i in 1..10 {
            // send an item to channel
            tx.send(i).unwrap();
        }
    });

    handle.await.unwrap();

    let end_time = Instant::now();
    let execution_time = end_time - start_time;
    println!("Execution time Async: {:?}", execution_time);
}
