use tokio::sync::{ Notify };
use tokio::task::JoinSet;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Clone)]
struct Foo {
    second_ready:Arc<Notify>,
    third_ready:Arc<Notify>,
}

impl Foo {
    fn new() -> Self {
        Self { second_ready: Arc::new(Notify::new()), third_ready: Arc::new(Notify::new()) }
    }
    async fn first(&self) {
        println!("first");
        self.second_ready.notify_one();
    }
    async fn second(&self)  {
        // self.second_ready.notified().await;
        println!("second");
        self.third_ready.notify_one();
    }

    async fn third(&self)  {
        // self.third_ready.notified().await;
        println!("third");
    }
    async fn process(&self, idx:usize) -> usize {
        match idx {
            1 => self.first().await,
            2 => self.second().await,
            3 => self.third().await,
            _ => {},
        }
        idx
    }
}

async fn printer(order:usize, value:usize, dependency:Arc<Vec<Arc<Notify>>>, numwords:Arc<HashMap<usize, String>>) {
    if order != 0 {
        dependency[order-1].notified().await;
    }
    if let Some(place) = numwords.get(&value) {
        println!("{:?}",place);
    }
    dependency[order].notify_one();
}

// async fn process(ids:&[usize]) {
//     let foo = Arc::new(Foo::new());
//     let mut futs = JoinSet::new();
//     for &i in ids {
//         let _foo = foo.clone();
//         futs.spawn(
//         async move {
//             _foo.process(i).await
//         });
//     }
//     futs.join_all().await;
// }
async fn process(ids:&mut [usize], numwords:Arc<HashMap<usize, String>>) {
    ids.sort();
    let dependency = Arc::new((0..ids.len()).map(|_| Arc::new(Notify::new())).collect::<Vec<_>>());

    let mut futs = JoinSet::new();
    for (order, &value) in ids.iter().enumerate() {
        let numwords = numwords.clone();
        let dependency = dependency.clone();
        futs.spawn(
        async move {
            printer(order, value, dependency, numwords).await
        });
    }
    futs.join_all().await;
}

// async fn process(ids:&mut [usize]) {
//     ids.sort();
//     let foo = Arc::new(Foo::new());
//     let mut futs = JoinSet::new();
//     for &mut i in ids {
//         let _foo = foo.clone();
//         futs.spawn(
//         async move {
//             _foo.process(i).await
//         });
//     }
//     futs.join_all().await;
// }



#[tokio::main]
async fn main() {
    let numwords:Arc<HashMap<usize, String>> = Arc::new(HashMap:: from([
        (1, String::from("first")),
        (2, String::from("second")),
        (3, String::from("third")),
        (4, String::from("fourth")),
        (5, String::from("fifth")),
        (6, String::from("sixth")),
    ]));
    // process(&[1,2,3]).await;
    // process(&mut [3,1,2], numwords).await;
    process(&mut [5, 6, 3, 5, 4, 5, 2], numwords).await;
    // process(&mut [3,1,2], numwords).await;
    // breaks
    // process(&[3,1]).await;
    // breaks
    // process(&[1]);
}
