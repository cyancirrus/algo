use tokio::sync::{Mutex, Notify, RwLock};
use std::sync::Arc;

// const THRESHOLD:usize = usize::MAX-1<<6;
const THRESHOLD:usize = 4;

struct Water {
    hydrogens:RwLock<usize>,
    oxygens:RwLock<usize>,
    del_hydrogens:RwLock<usize>,
    del_oxygens:RwLock<usize>,
    rebase_oxygen:RwLock<bool>,
    rebase_hydrogen:RwLock<bool>,
    available:Notify,
}

impl Water {
    fn new() -> Self {
        Self {
            hydrogens:RwLock::new(0),
            oxygens:RwLock::new(0),
            del_hydrogens:RwLock::new(0),
            del_oxygens:RwLock::new(0),
            rebase_oxygen:RwLock::new(false),
            rebase_hydrogen:RwLock::new(false),
            available:Notify::new(),
        }
    }
    async fn factory(self: &Arc<Self>) {
        loop {
            
            let (rb_h, rb_o) = (*self.rebase_hydrogen.read().await, *self.rebase_oxygen.read().await);
            match (rb_h, rb_o) { 
                (false, false) => {
                    self.available.notified().await;
                    self.create_water().await;
                    self.update().await;
                },
                (true, false) => {
                    self.rebase_hydrogen().await;
                },
                (false, true) => {
                    self.rebase_oxygen().await;
                },
                (true, true) => {
                    self.rebase().await;
                },

            }
        }
    }

    async fn rebase_hydrogen(&self) {
        {
            println!("REBASING HYDROGEN");
            let mut gross_h = self.hydrogens.write().await;
            let mut delta_h = self.del_hydrogens.write().await;
            *gross_h -= *delta_h;
            *delta_h = 0;
        }
        {
            *self.rebase_hydrogen.write().await = false;
        }
    }

    async fn rebase_oxygen(&self) {
        {
            println!("REBASING OXYGEN");
            let mut gross_o = self.oxygens.write().await;
            let mut delta_o = self.del_oxygens.write().await;
            *gross_o -= *delta_o;
            *delta_o = 0;
        }
        {
            *self.rebase_oxygen.write().await = false;
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

    async fn rebase(self: &Arc<Self>) {
        {
            self.rebase_hydrogen().await;
            *self.rebase_hydrogen.write().await = false;
        }
        {
            self.rebase_oxygen().await;
            *self.rebase_oxygen.write().await = false;
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
        if *h > THRESHOLD {
            *self.rebase_hydrogen.write().await = true;
        }
        self.available.notify_one();
    }
    async fn new_oxygen(self: &Arc<Self>) {
        println!("new oxygen");
        let mut o = self.oxygens.write().await;
        *o += 1;
        if *o > THRESHOLD {
            *self.rebase_oxygen.write().await = true;
        }
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
    let rb = water.clone();


    water.factory().await;
}

