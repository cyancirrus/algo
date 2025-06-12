use tokio::sync::{ Mutex, Notify };
use tokio::sync::mpsc::{channel, Sender};
use tokio::task;
use tokio::join;
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

    let fizz_handle = task::spawn(async move {
        while let Some(i) = fizz_rx.recv().await {
            println!("fizz {i}");
        }
    });
    let buzz_handle = tokio::spawn( async move {
        while let Some(i) = buzz_rx.recv().await {
            println!("buzz {i}");
        }
    });
    let num_handle = tokio::spawn( async move {
        while let Some(i) = num_rx.recv().await {
            println!("{i}");
        }
    });
    let fzbz_handle = tokio::spawn( async move {
        while let Some(i) = fzbz_rx.recv().await {
            println!("fizzbuzz {i}");
        }
    });

    for i in 1..=n {
        match (i % 3 == 0, i % 5 == 0) {
            (false, false) => num_tx.send(i).await.unwrap(),
            (true, true) => fzbz_tx.send(i).await.unwrap(),
            (true, false) => fizz_tx.send(i).await.unwrap(),
            (false, true) => buzz_tx.send(i).await.unwrap(),
        }
    }
    drop(fizz_tx);
    drop(buzz_tx);
    drop(fzbz_tx);
    drop(num_tx);

    let _ = join!(fizz_handle, buzz_handle, fzbz_handle, num_handle);
}



#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // println!("Result {:?}", fizzbuzz(16).await);
    fizzbuzz(15).await;
}
