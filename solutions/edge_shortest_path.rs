#![allow(dead_code)]
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::mem;

#[derive(Eq, PartialEq)]
struct Node {
    d:usize,
    p:(usize, usize),
}

impl Ord for Node {
    fn cmp(&self, other:&Self) -> Ordering { self.d.cmp(&other.d) }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> { Some(self.cmp(&other)) }
}

// solution space
// solutions look like 010203, 10203
fn shortest_path(edges:&[Vec<usize>]) -> i32 {
    let n = edges.len();
    // implementation assumes n less than bitmask card
    debug_assert!(n <= 32);
    // bitmask completion
    let full = (1usize<<n)-1;
    let mut seen = vec![vec![false;1<<n];n];
    let mut state = VecDeque::with_capacity(n);
    // initialize state 
    for i in 0..n {
        let m = 1<<i;
        state.push_back((i, m, 0));
        seen[i][m]=true;
    }
    while let Some((u, mask, d)) = state.pop_front() {
        if mask == full {
            return d as i32;
        }
        for &n in &edges[u] {
            let nmask = mask | 1 << n;
            if !seen[u][nmask ] {
                state.push_back((n, nmask, d + 1));
            }
        }
    }
    -1
}


fn main() {
    println!("shortest path {:?}", shortest_path(&[vec![1,2,3],vec![0],vec![0],vec![0]]));
}
