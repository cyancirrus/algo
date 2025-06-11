use tokio::sync::{ Mutex, Notify };
use tokio::sync::mpsc::{channel, Sender};
use tokio::task;
use std::sync::Arc;
use std::sync::atomic::{ Ordering, AtomicUsize };

// #[derive(Eq, PartialEq, Debug)]
// enum Task {
//     Fizz(usize),
//     Buzz(usize),
//     FizzBuzz(usize),
//     Number(usize),
// }

async fn fizzbuzz(n:usize) {
    let (fizz_tx, mut fizz_rx) = channel::<usize>(10);
    let (buzz_tx, mut buzz_rx) = channel::<usize>(10);
    let (fzbz_tx, mut fzbz_rx) = channel::<usize>(10);
    let (num_tx, mut num_rx) = channel::<usize>(10);

    task::spawn(async move {
        while let Some(_) = fizz_rx.recv().await {
            println!("fizz");
        }
    });
    tokio::spawn( async move {
        while let Some(_) = buzz_rx.recv().await {
            println!("buzz");
        }
    });
    tokio::spawn( async move {
        while let Some(i) = num_rx.recv().await {
            println!("{i}");
        }
    });
    tokio::spawn( async move {
        while let Some(_) = fzbz_rx.recv().await {
            println!("fizzbuzz");
        }
    });

    for i in 0..n {
        match (i % 3 == 0, i % 5 == 0) {
            (true, true) => fzbz_tx.send(i).await.unwrap(),
            (true, false) => fizz_tx.send(i).await.unwrap(),
            (false, true) => buzz_tx.send(i).await.unwrap(),
            (false, false) => num_tx.send(i).await.unwrap(),
        }
    }
}



#[tokio::main]
async fn main() {
    // println!("Result {:?}", fizzbuzz(16).await);
    fizzbuzz(10_000).await;
}
