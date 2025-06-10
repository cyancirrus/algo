use tokio::sync::{ Mutex, Notify };
use tokio::task::JoinSet;
use std::sync::Arc;
use std::sync::atomic::{ Ordering, AtomicUsize };
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::mem;


async fn foo(i:usize) -> (usize, String) {
    (i, format!("foo({})", i))
}

async fn bar(i:usize) -> (usize, String) {
    (i, format!("bar({})", i))
}


async fn nfoobar(n:usize) {
    let mut futs = JoinSet::new();
    for i in (0..2*n).step_by(2) {
        futs.spawn(async move {foo(i).await});
        futs.spawn(async move {bar(i+1).await});
    }
    let mut output: BTreeMap<usize, String>  = BTreeMap::new();
    while let Some(Ok((i,s))) = futs.join_next().await {
        output.insert(i, s);
    }
    let mut foobar = String::new();
    while let Some((_, mut word)) = output.pop_last() {
        mem::swap(&mut word, &mut foobar);
        foobar += &word;
    }
    println!("{:?}", foobar);
}

#[tokio::main]
async fn main() {
    nfoobar(10).await;
    // process(&mut [5, 6, 3, 5, 4, 5, 2], numwords).await;
}
