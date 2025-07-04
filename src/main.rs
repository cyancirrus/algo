mod ring;
use tokio;
use tokio::sync::Notify;
use std::sync::Arc;
use ring::RingBuffer;

#[async_trait::async_trait]
pub trait Handler<T>: Send + Sync {
    async fn process(&self, task:T);
}

struct Calculator;

#[async_trait::async_trait]
impl Handler<(i32, Opps)> for Calculator {
    async fn process(&self, task:(i32, Opps)) {
        let (val, op) = task;
        match op {
            Opps::Add => println!("Adding {}", val),
            Opps::Mul => println!("Multiplying {}", val),
            Opps::Sub => println!("Subtracting {}", val),
            Opps::Div => println!("Dividing {}", val),
        }

    }
}
#[derive(Clone)]
enum Opps{
    Add,
    Mul,
    Sub,
    Div
}

struct AsyncQueue<T,H> 
where
    T: Send + 'static,
    H: Handler<T>,

{
    notify:Arc<Notify>,
    mem_queue:usize,
    mem_overflow:usize,
    queue:RingBuffer<T>,
    overflow:RingBuffer<T>,
    handler:Arc<H>,
}

impl<T, H> AsyncQueue<T, H> 
where
    T: Send + 'static,
    H: Handler<T>,

{
    async fn new(queue_size:usize, overflow_size:usize, handler:Arc<H>) -> Self {
        Self {
            notify:Arc::new(Notify::new()),
            mem_queue:queue_size,
            mem_overflow:overflow_size,
            queue:RingBuffer::new(queue_size),
            overflow:RingBuffer::new(overflow_size),
            handler,
        }
    }
    pub async fn execute(&mut self)  {
        if let Some(task) = self.queue.pop_front() {
            self.handler.process(task).await;
            self.notify.notify_one();
        }
    }

    pub async fn load(&mut self, task:T) {
        loop {
            if self.queue.len < self.mem_queue {
                while self.overflow.len > 0 && self.queue.len < self.mem_queue {
                    if let Some(t) = self.overflow.pop_front() {
                        self.queue.push_back(t);
                    }
                }
                if self.queue.len < self.mem_queue {
                    self.queue.push_back(task);
                    return;
                }
            }

            if self.overflow.len < self.mem_overflow {
                self.overflow.push_back(task);
                return;
            }
            self.notify.notified().await;
        }
    }
}


#[tokio::main]
async fn main() {
    let mut test = RingBuffer::new(5);
    for i in 0..5 {
        test.push_back(i);
    }
    test.push_back(5);
    test.pop_front();
    test.push_front(11);
    test.pop_back();
    test.push_back(100);
    test.push_back(100);
    test.push_front(42);
    let c = test.iter();

    for c in c {
        println!("Value {c:?}");
    }
}
