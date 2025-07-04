#![allow(dead_code)]
use std::collections::HashMap;
use std::collections::VecDeque;
use std::mem::MaybeUninit;

struct RingBuffer<T> {
    data:Vec<MaybeUninit<T>>,
    head:usize,
    tail:usize,
    len:usize,
    capacity:usize,

}

impl<T> RingBuffer<T> 
{
    fn new(k:usize) -> Self {
        let mut data = Vec::with_capacity(k);
        data.resize_with(k, MaybeUninit::uninit);
        Self {
            data,
            head:0,
            tail:0,
            len:0,
            capacity:k,
        }
    }
    fn push_front(&mut self, value:T) {
        if self.len == self.capacity {
            self.head = (self.head + self.capacity - 1) % self.capacity;
            unsafe {
                self.data[self.head].assume_init_drop();
            }
            self.data[self.head] = MaybeUninit::new(value);
            self.tail = self.head;
        } else {
            self.head = (self.head + self.capacity - 1) % self.capacity;
            self.data[self.head] = MaybeUninit::new(value);
            self.len += 1;
        }
    }
    fn push_back(&mut self, value:T) {
        if self.len == self.capacity {
            unsafe {
                self.data[self.tail].assume_init_drop();
            }
            self.data[ self.tail ] = MaybeUninit::new(value);
            self.tail = (self.tail + 1) % self.capacity;
            self.head = self.tail;
        } else {
            self.data[ self.tail ] = MaybeUninit::new(value);
            self.tail = (self.tail + 1) % self.capacity;
            self.len += 1;
        }
    }
    fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let idx = self.head;
        self.head = (self.head + 1) % self.capacity;
        self.len -= 1; 
        Some( unsafe {
            self.data[idx].assume_init_read()
        })
    }
    fn pop_tail(&mut self) -> Option<T>{
        if self.len == 0 {
            return None;
        }
        self.tail = (self.tail + self.capacity - 1) % self.capacity;
        self.len -= 1; 
        Some( unsafe {
            self.data[self.tail].assume_init_read()
        })
    }
}

impl <T> Drop for RingBuffer<T> {
    fn drop(&mut self) {
        let mut idx = self.head;
        for _ in 0..self.len {
            unsafe {
                self.data[idx].assume_init_drop();
            }
            idx = (idx + 1) % self.capacity;
        }
    }
}


fn find_k_closest_binary(
    k:usize,
    x:usize,
    data:&[usize]
) -> VecDeque<usize> {
    let mut closest:VecDeque<usize>=VecDeque::with_capacity(k);
    let n = data.len();
    let mut l = 0;
    let mut r = n;
    
    while l < r {
        let c = (l + r)/ 2;
        if data[c] < x {
            l = c + 1;
        } else {
            r = c;
        }
    }
    let c = l;
    // overloading l to use count 
    l = 1;
    r = 1;
    for _ in 0..k {
        let l_dist = if c > l {
            data[c-l].abs_diff(x)
        } else {
            usize::MAX
        };
        let r_dist = if c + r < n {
            data[c+r].abs_diff(x)
        } else {
            usize::MAX
        };
        if l_dist < r_dist {
            closest.push_front(data[c-l]);
            l += 1;
        } else {
            closest.push_back(data[c+r]);
            r += 1;
        }
    }
    closest
}


fn find_k_closest(
    k:usize,
    x:usize,
    data:&[usize]
) -> VecDeque<usize> {
    // 1 < k < n;
    let mut l_dist = data[0].abs_diff(x);
    let mut closest:VecDeque<usize> = data[0..k].iter().cloned().collect::<_>();
    for i in k..data.len() {
        let dist = data[i].abs_diff(x);
        if dist < l_dist {
            closest.pop_front();
            closest.push_back(data[i]);
            l_dist = data[i].abs_diff(x);
        }
    }
    closest
}


// 1..n friends n+1 -> 1 ring(of n)
fn circle_game(n:usize, k:usize) -> usize {
    if n == 0 { return 0 }
    let mut players=(1..=n).collect::<Vec<usize>>();
    let mut curr = 0;
    let mut len = n;

    while len > 1 {
        curr=(curr + k)%len;
        players.remove(curr);
        len-=1;
    }
    players.pop().unwrap()
}

fn circle_game_josephus(k:usize, n:usize) -> usize {
    // this one feels like trivia
    let mut ans = 0;
    for i in 2..n {
        ans=(ans + k) % i;
    }
    ans+1
}

// player 0 requires ball
// player 1 requires 2 balls same
// player n requires n+1 balls same
// colors unordered unbounded <- no vecs

fn winning_players(n:usize, pick:&Vec<(usize,usize)>) -> Vec<usize> {
    let mut state: HashMap<usize, HashMap<usize, usize>> = HashMap::with_capacity(n);
    let mut winners: Vec<usize> = Vec::with_capacity(n);
    let mut included: Vec<bool> = vec![true;n];


    for (player, colour) in pick {
        if included[*player] {
            let colour_count = state
                .entry(*player)
                .or_insert(HashMap::new())
                .entry(*colour)
                .or_insert(0);
            *colour_count += 1;
            if *colour_count > *player {
                winners.push(*player);
                included[*player] = false;
            }
        }

    }
    winners    
}


fn main() {
    println!("main");
}
