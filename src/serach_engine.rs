#![allow(dead_code)]
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Reverse;
use std::hash::Hash;

trait Frontier<T> {
    fn push(&mut self, item:T);
    fn pop(&mut self) -> Option<T>;
    fn is_empty(&self) -> bool;
    fn new() -> Self;
}

struct Queue<T>(VecDeque<T>);
struct PQueue<T:Ord>(BinaryHeap<Reverse<T>>);
struct Stack<T>(Vec<T>);

impl <T>  Frontier<T> for Queue<T> {
    fn push(&mut self, item:T) { self.0.push_back(item); }
    fn pop(&mut self) -> Option<T> { self.0.pop_front() }
    fn is_empty(&self) -> bool { self.0.is_empty() }
    fn new() -> Self { Self(VecDeque::new()) }
}

impl <T> Frontier<T> for PQueue<T>
where T: Ord
{
    fn push(&mut self, item:T) { self.0.push(Reverse(item)) }
    fn pop(&mut self) -> Option<T> { self.0.pop().map(|Reverse(v)| v) }
    fn is_empty(&self) -> bool { self.0.is_empty() }
    fn new() -> Self { Self(BinaryHeap::new()) }

}

impl <T> Frontier<T> for Stack<T> {
    fn push(&mut self, item:T) { self.0.push(item) }
    fn pop(&mut self) -> Option<T> { self.0.pop() }
    fn is_empty(&self) -> bool { self.0.is_empty() }
    fn new() -> Self { Self(Vec::new()) }
}

fn search_unweighted<S, I, IO, K, Fr, Next, Key, Goal> (
    starts: IO,
    mut next:Next,
    mut key: Key,
    is_goal: Goal,
    frontier: &mut Fr,
    seen: &mut HashSet<K>,
) -> Option<S>
where 
    S: Hash + Eq + Copy,
    K: Eq + Hash,
    I: IntoIterator<Item=S>,
    IO: IntoIterator<Item=S>,
    Fr: Frontier<S>,
    Next: FnMut(&S) -> I,
    Key: FnMut(&S) -> K,
    Goal: Fn(&S) -> bool,
{
    for s in starts {
        let ks = key(&s);
        if seen.insert(ks) {
            frontier.push(s);
        }
    }
    while let Some(u) = frontier.pop() {
        if is_goal(&u) { return Some(u); }
        for v in next(&u) {
            if seen.insert(key(&v)) {
                frontier.push(v);
            }
        }
    }
    None
}

fn bfs_on_edge<T>(edges:&[(T, T)], start:T, goal:T) -> Option<T>
where T: Hash + Eq + Copy
{
    let mut queue = Queue::new();
    let mut seen = HashSet::new();
    let mut diredges:HashMap<T, Vec<T>> = HashMap::new();
    for (src, tgt) in edges {
        diredges.entry(*src).or_default().push(*tgt);
        diredges.entry(*tgt).or_default();
    }
    let next = |u: &T| {
        diredges.get(u).into_iter().flatten().copied()
    };

    search_unweighted(
        [start],
        next,
        |u| *u,
        |u| *u == goal,
        &mut queue,
        &mut seen,
    )
}

fn bfs_adj(adj:&[Vec<usize>], start:usize, goal:usize) -> Option<usize> {
    let mut queue = Queue::new();
    let mut seen = HashSet::new();
    let n = adj.len();
    let mut adjacency = vec![vec![];n];
    for src in 0..n {
        for tgt in 0..n {
            if adj[src][tgt] == 1{
                adjacency[src].push(tgt);
            }
        }
    }
    let next = | u:&usize | { adjacency[*u].iter().copied() };
    search_unweighted(
        [start],
        next,
        |u| *u,
        |u| *u == goal,
        &mut queue,
        &mut seen,
    )
}

fn dijsktras<S, I, IO, K, Key, Next, Weight, Goal>(
    starts:IO,
    key:Key,
    mut next:Next,
    mut is_goal:Goal,
    mut weight:Weight,
    dist:&mut HashMap<K,usize>,
    frontier:&mut PQueue<(usize, S)>,
) -> Option<S>
where

    S: Hash + Eq + Ord,
    K: Hash + Eq + Copy,
    IO: IntoIterator<Item=S>,
    I: IntoIterator<Item=S>,
    Key: Fn(&S) -> K,
    Next: FnMut(&S) -> I,
    Weight: FnMut(&(K, K)) -> Option<usize>,
    Goal: FnMut(&S) -> bool,
{
    for s in starts {
        let ks = key(&s);
        frontier.push((0, s));
        dist.insert(ks, 0);
    }
    while let Some((du, u)) = frontier.pop() {
        let ku = key(&u);

        if let Some(&best) = dist.get(&ku) {
            if du > best { continue };
        }
        if is_goal(&u) { return Some(u); }
        for v in next(&u) {
            let kv = key(&v);
            if let Some(w) = weight(&(ku, kv)) {
                let nd = match du.checked_add(w) {
                    Some(x) =>x,
                    None => continue,
                };
                if nd < *dist.get(&kv).unwrap_or(&usize::MAX) {
                    dist.insert(kv, nd);
                    frontier.push((nd, v));
                }
            }
        }
    }
    None
}

fn djs_on_edge<T>(edges:&[(T, T, usize)], start:T, goal:T) -> Option<T>
where T: Hash + Eq + Copy + Ord 
{
    let n = edges.len();
    let mut diredges: HashMap<(T, T), usize> = HashMap::with_capacity(n);
    let mut distance: HashMap<T, usize> = HashMap::with_capacity(n);
    let mut neighbors: HashMap<T, Vec<T>> = HashMap::new();
    let mut frontier = PQueue::new();
    for (src, tgt, d) in edges {
        diredges.insert((*src, *tgt), *d);
        neighbors.entry(*src).or_default().push(*tgt);
        neighbors.entry(*tgt).or_default();
    }
    let weight = |e: &(T,T) | { diredges.get(e).copied() };
    let next = |u:&T | { neighbors.get(u).into_iter().flatten().copied() };

    dijsktras(
        [start],
        |u| *u,
        next,
        |u| *u == goal,
        weight,
        &mut distance,
        &mut frontier, 
    )
}


fn main() {
}
