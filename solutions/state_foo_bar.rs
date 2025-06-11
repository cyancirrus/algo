use tokio::sync::{ Mutex, Notify };
use tokio::task::JoinSet;
use std::sync::Arc;
use std::sync::atomic::{ Ordering, AtomicUsize };
use std::collections::HashMap;

// States
// unstarted
// locked (alternating foo/bar)
// finished (up to n)

#[derive(Debug, Clone, PartialEq)]
enum State {
    Unstarted,
    Foo,
    Bar,
    Finished,
}

struct StateTracker {
    state: Mutex<State>,
    notify: Notify,
    n: AtomicUsize,
}

impl StateTracker {
    fn new(n:usize) -> Arc<Self> {
        Arc::new(Self { 
            state: Mutex::new(State::Unstarted), 
            notify: Notify::new(),
            n: AtomicUsize::new(n),
        })
    }
    async fn foo(&self) {
        loop {
            let mut state = self.state.lock().await;
            if *state == State::Unstarted || *state == State::Foo {
                println!("foo");
                *state = State::Bar;
                self.notify.notify_waiters();
                break;
            }
            self.notify.notified().await;
        }
    }

    async fn bar(&self) {
        loop {
            let mut state = self.state.lock().await;
            if *state == State::Bar {
                println!("bar");
                let rem = self.n.fetch_sub(1, Ordering::SeqCst);
                if rem == 0 {
                    *state = State::Finished;
                } else {
                    *state = State::Foo;
                }
                self.notify.notify_waiters();
                break;
            }
            self.notify.notified().await;
        }
    }

    async fn process(&self) {
        loop {
            self.foo().await;
            self.bar().await;
            let state = self.state.lock().await;
            if *state == State::Finished {
                break
            }
        }
    }
}



async fn nfoobar(n:usize) {
    let tracker = StateTracker::new(n);
    tracker.process().await 

}


#[tokio::main]
async fn main() {
    nfoobar(2).await;
    // process(&mut [5, 6, 3, 5, 4, 5, 2], numwords).await;
}
