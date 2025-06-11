use tokio::sync::{ Mutex, Notify };
use std::sync::Arc;
use std::sync::atomic::{ Ordering, AtomicUsize };

#[derive(Eq, PartialEq, Debug)]
enum State {
    Fizz,
    Buzz,
    FizzBuzz,
    Number,
    Complete,
}

struct StateModel {
    i:AtomicUsize,
    n:AtomicUsize,
    notify_done:Notify,
    notify_num:Notify,
    notify_fizz:Notify,
    notify_buzz:Notify,
    notify_fzbz:Notify,
    state:Mutex<State>,
    output:Mutex<Vec<String>>,
}
// Contract based from leetcode
fn printfizz() {
    println!("fizz")
}

fn printbuzz() {
    println!("buzz")
}

fn printfizzbuzz() {
    println!("fizzbuzz")
}

fn printnumber(n:usize) {
    println!("{n}")
}

impl StateModel {
    async fn new(n:usize) -> Arc<Self> {
        // 1 is in state number
        Arc::new( Self {
            i:AtomicUsize::new(1),
            n:AtomicUsize::new(n),
            notify_done:Notify::new(),
            notify_num:Notify::new(),
            notify_fizz:Notify::new(),
            notify_buzz:Notify::new(),
            notify_fzbz:Notify::new(),
            state:Mutex::new(State::Number),
            output:Mutex::new(Vec::with_capacity(n)),
        })
    }
    async fn notify_state(&self) {
        let state = self.state.lock().await;
        match *state {
            State::Number => self.notify_num.notify_one(),
            State::Fizz => self.notify_fizz.notify_one(),
            State::Buzz => self.notify_buzz.notify_one(),
            State::FizzBuzz => self.notify_fzbz.notify_one(),
            State::Complete => self.notify_done.notify_one(),
        }
        drop(state);
    }
    fn match_state(k:usize, n:usize) -> State {
        let div3 = (k % 3) == 0;
        let div5 = (k % 5) == 0;
        let done = k > n;
        match (div3, div5, done) {
            (_, _, true) => State::Complete,
            (true, false, false) => State::Fizz,
            (false, true, false) => State::Buzz,
            (true, true, false) => State::FizzBuzz,
            (false, false, false) => State::Number,

        }
    }
    async fn template<ConsoleLog, OutputFmt>(&self, c_log:ConsoleLog, o_fmt:OutputFmt, req_state:State, listener:&Notify) 
    // async fn template<ConsoleLog>(&self, c_log:ConsoleLog, req_state:State, listener:&Notify) 
    where
        ConsoleLog: Fn(usize) + Send + 'static,
        OutputFmt: Fn(usize) -> String + Send + 'static,

    {
        loop {
            let mut state = self.state.lock().await;
            let mut output = self.output.lock().await;
            if *state == req_state {
                let i = self.i.fetch_add(1, Ordering::SeqCst);
                c_log(i);
                let n = self.n.load(Ordering::SeqCst);
                *state = StateModel::match_state(i+1, n);
                output.push(o_fmt(i));
            }
            drop(output);
            drop(state);
            self.notify_state().await;
            listener.notified().await;
        }
    }
    async fn buzz(&self) {
        self.template(
            |_| printbuzz(),
            |_| format!("buzz"),
            State::Buzz,
            &self.notify_buzz
        ).await
    }
    async fn fizzbuzz(&self) {
        self.template(
            |_| printfizzbuzz(),
            |_| format!("fizzbuzz"),
            State::FizzBuzz,
            &self.notify_fzbz
        ).await
    }
    async fn fizz(&self) {
        self.template(
            |_| printfizz(),
            |_| format!("fizz"),
            State::Fizz,
            &self.notify_fizz
        ).await
    }
    
    async fn number(&self) {
        self.template(
            |i| printnumber(i),
            |i| format!("{i}"),
            State::Number,
            &self.notify_num
        ).await
    }

    async fn process(self:Arc<Self>) -> Vec<String> {
        let s = Arc::clone(&self);
        tokio::spawn(async move {
            s.number().await;
        });

        let s = Arc::clone(&self);
        tokio::spawn(async move {
            s.fizz().await;
        });
        
        let s = Arc::clone(&self);
        tokio::spawn(async move {
            s.buzz().await;
        });
        
        let s = Arc::clone(&self);
        tokio::spawn(async move {
            s.fizzbuzz().await;
        });
        loop {
            let state = self.state.lock().await;
            let output = self.output.lock().await;
            if *state == State::Complete {
                return output.to_vec()
            }
            drop(state);
            drop(output);
            self.notify_done.notified().await;
        }
    }
}

async fn fizzbuzz(n:usize) -> Vec<String> {
    let state = StateModel::new(n).await;
    state.process().await
}


#[tokio::main]
async fn main() {
    println!("Result {:?}", fizzbuzz(16).await);
}
