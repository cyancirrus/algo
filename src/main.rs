use rand::Rng;
const SIZE: usize = 127;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn random(size: usize) -> usize {
    assert!(size > 0);
    rand::thread_rng().gen_range(0..size)
}

#[derive(Clone, Copy, Debug)]
struct Item {
    datum: usize,
}

impl Item {
    fn binary(&self) -> usize {
        self.datum
    }
}

struct Cache {
    storage: [Option<Item>; SIZE],
}


fn primes(size: usize) -> Vec<usize> {
    let mut primes: Vec<usize> = vec![2, 3];
    let mut candidate = 5;

    while primes.len() < size {
        let mut is_prime = true;
        for &p in &primes {
            if candidate % p == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(candidate);
        }
        candidate += 2;
    }
    primes
}


impl Cache {
    fn new() -> Self {
        Self {
            storage: [None; SIZE],
        }
    }

    fn store(&mut self, hash_args: usize, result: Item) {
        self.storage[hash_args % SIZE] = Some(result);
    }

    fn get(&self, hash_args: usize) -> Option<Item> {
        self.storage[hash_args % SIZE]
    }
}

fn hash(input: &Item, size: usize) -> usize {
    let prime_table = primes(SIZE);
    let prime_index = 31;
    let prime = prime_table[prime_index];
    prime * input.binary() % size
}

fn heavy_compute_function(x: usize, cache: &mut Cache) -> Item {
    let item_key = Item { datum: x };
    let hash_key = hash(&item_key, SIZE);
    if let Some(cached_item) =  cache.get(hash_key) {
        println!("This should save some time?");
        cached_item
    } else {
        let item = Item { datum: x * x };
        sleep(Duration::from_secs(1));
        cache.store(hash_key, item);
        item
    }
}

fn main() {
    let mut cache = Cache::new();

    let input = 5;
    let result = heavy_compute_function(input, &mut cache);

    println!("Computed: {:?}", result);

    // Try to retrieve it
    let key = hash(&result, SIZE);
    if let Some(cached) = cache.get(key) {
        println!("From cache: {:?}", cached);
    }
    let start = Instant::now();
    for i in 0..5 {
        heavy_compute_function(i, &mut cache);
    }
    let duration = start.elapsed();
    println!("The non-cached version took {:?}", duration);
    println!("The cache appears as {:?}", cache.storage);

    let start = Instant::now();
    for i in 0..5 {
        heavy_compute_function(i, &mut cache);
    }
    let duration = start.elapsed();
    println!("The Cached version took {:?}", duration);
    println!("The cache appears as {:?}", cache.storage);
}
