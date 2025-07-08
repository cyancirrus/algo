use tokio::sync::{Mutex, Notify, RwLock};
use std::sync::Arc;

struct Water {
    hydrogens:RwLock<usize>,
    oxygens:RwLock<usize>,
    del_hydrogens:RwLock<usize>,
    del_oxygens:RwLock<usize>,
    available:Notify,
}

impl Water {
    fn new() -> Self {
        Self {
            hydrogens:RwLock::new(0),
            oxygens:RwLock::new(0),
            del_hydrogens:RwLock::new(0),
            del_oxygens:RwLock::new(0),
            available:Notify::new(),
        }
    }
    async fn factory(self: &Arc<Self>) {
        loop {
            self.available.notified().await;
            self.create_water().await;
            self.update().await;
        }
    }

    async fn create_water(self: &Arc<Self>) -> bool {
        let (h_free, o_free) = (
            *self.hydrogens.read().await - *self.del_hydrogens.read().await, 
            *self.oxygens.read().await - *self.del_oxygens.read().await
        );
        if h_free >= 2  && o_free >= 1 {
            println!("created water");
            let (mut h, mut o)  = (
                self.del_hydrogens.write().await,
                self.del_oxygens.write().await,
            );
            *h += 2;
            *o += 1;
            true
        } else {
            false
        }
    }

    async fn update(self: &Arc<Self>) {
        {
            let (h_free, o_free) = {(
                *self.hydrogens.read().await - *self.del_hydrogens.read().await, 
                *self.oxygens.read().await - *self.del_oxygens.read().await
            )};
            if h_free >= 2  && o_free >= 1 {
                self.available.notify_one();
            }
        }
    }
    async fn new_hydrogen(self: &Arc<Self>) {
        println!("new hydrogen");
        let mut h = self.hydrogens.write().await;
        *h += 1;
        self.available.notify_one();
    }
    async fn new_oxygen(self: &Arc<Self>) {
        println!("new oxygen");
        let mut o = self.oxygens.write().await;
        *o += 1;
        self.available.notify_one();
    }

}

#[tokio::main]
async fn main() {
    let water = Arc::new(Water::new());

    let w1 = water.clone();
    tokio::spawn(async move {
        for _ in 0..10 {
            w1.new_hydrogen().await;
        }
    });
    let w2 = water.clone();
    tokio::spawn(async move {
        for _ in 0..6 {
            w2.new_oxygen().await;
        }
    });

    water.factory().await;
}

