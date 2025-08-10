#![allow(dead_code)]
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Reverse;
use std::ops::Add;
use std::hash::Hash;

//TODO: Implement a heuristic to unite dijsktras with a* as best_first


trait Frontier<T> {
    fn push(&mut self, item:T);
    fn pop(&mut self) -> Option<T>;
    fn is_empty(&self) -> bool;
    fn new() -> Self;
}

trait Tracker<S, D> {
    fn retrieve(&self, s:&S) -> Option<&D>;
    fn update(&mut self, s:&S, d:D);
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

struct EdgeDistance<S, D> (HashMap<S, D>); 
struct GridDistance<D> (Vec<Vec<Option<D>>>);
type Entry = (usize, usize);

impl <S, D> Tracker<S,D> for EdgeDistance<S, D>
where
    S: Hash + Eq + Copy,
    D: Copy,
{
    fn retrieve(&self, s:&S) -> Option<&D> {
        self.0.get(k)
    }
    fn update(&mut self, s:&S, d:D) {
        self.0.insert(*s, d);
    }
}

impl <D> GridDistance<D>
where D: Clone
{
    fn new(m:usize, n:usize) -> Self {
        Self(
            vec![vec![None; n];m]
        )
    }
}

impl <D:Clone> Tracker <Entry, D> for GridDistance<D>
{
    fn retrieve(&self, &s:&Entry) -> Option<&D> {
        let (x, y) = s;
        self.0.get(x)?.get(y)?.as_ref()
    }
    fn update(&mut self, &s:&Entry, d:D) {
        let (x, y) = s;
        self.0[x][y] = Some(d);
    }
}


fn best_first<S, I, Tr, D, Next, Weight, Goal, Heuristic>(
    mut next:Next,
    mut is_goal:Goal,
    mut weight:Weight,
    heuristic: Heuristic,
    tracker: &mut Tr,
    frontier:&mut PQueue<(D, S)>,
) -> Option<D>
where
    S: Hash + Eq + Ord,
    I: IntoIterator<Item=S>,
    D: PartialOrd + Add<Output=D> + Ord + Eq + Copy,
    Tr:Tracker<S, D>,
    Heuristic: Fn(&S) -> D,
    Next: FnMut(&S) -> I,
    Weight: FnMut(&S, &S) -> Option<D>,
    Goal: FnMut(&S) -> bool,
{
    while let Some((g_u, u)) = frontier.pop() {
        if let Some(best) = &tracker.retrieve(&u) {
            if g_u > **best { continue };
        }
        if is_goal(&u) { return Some(g_u); }
        for v in next(&u) {
            if let Some(w_uv) = weight(&u, &v) {
                let g_v = g_u + w_uv;
                match tracker.retrieve(&v) {
                    None => {
                        tracker.update(&v, g_v);
                        let f_v = g_v + heuristic(&v);
                        frontier.push((f_v, v));
                    },
                    Some(&best_g) if g_v < best_g => {
                          {
                            tracker.update(&v, g_v);
                            let f_v = g_v + heuristic(&v);
                            frontier.push((f_v, v));
                        }
                    },
                    _ => {}
                }
            }
        }
    }
    None
}

fn djs_on_edge<T>(edges:&[(T, T, usize)], start:T, goal:T) -> Option<usize>
where
    T: Hash + Eq + Copy + Ord,
{
    let n = edges.len();
    let mut diredges: HashMap<(T, T), usize> = HashMap::with_capacity(n);
    let mut distance: EdgeDistance<T, usize> = EdgeDistance(HashMap::with_capacity(n));
    let mut neighbors: HashMap<T, Vec<T>> = HashMap::new();
    let mut frontier = PQueue::new();
    for &(src, tgt, d) in edges {
        diredges.insert((src, tgt), d);
        neighbors.entry(src).or_default().push(tgt);
        neighbors.entry(tgt).or_default();
    }
    let weight = |s:&T, t:&T  | { diredges.get(&(*s,*t)).copied() };
    let next = |u:&T | { neighbors.get(u).into_iter().flatten().copied() };
    frontier.push((0, start));
    distance.update(&start, 0);

    best_first(
        next,
        |u| *u == goal,
        weight,
        |&_| 0,
        &mut distance,
        &mut frontier, 
    )
}


fn astar_grid(grid:&[Vec<Option<isize>>], start:&Entry, goal:&Entry) -> Option<isize>
{
    let m = grid.len();
    let n = grid[0].len();
    let mut dist: GridDistance<isize> = GridDistance::new(m, n);
    let mut pqueue = PQueue::new();
    pqueue.push((0_isize, start.clone()));
    dist.update(&start, 0);
    let mut sum = 0;
    let mut count:usize = 0;
    for i in 0..m { for j in 0..n {
        if let Some(d) = grid[i][j] { sum += d; count +=1; }
    }}
    let mean = sum / count as isize;

    let next = |&(x, y): &Entry| {
        let mut neighs = vec![];
        for (dx,dy) in [(0, 1), (1,0), (!0, 0), (0, !0)] {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            if let Some(_) = grid[nx][ny] {
                neighs.push((nx, ny));
            }
        }
        neighs.into_iter()
    };
    let weight = |&(bx, by): &Entry, &(tx, ty):&Entry | {
        if let Some(d) = grid[tx][ty] {
            let dx = (bx as isize - tx as isize).abs();
            let dy = (by as isize - ty as isize).abs();
            if dx + dy == 1 {
                return Some(d)
            }
        };
        None
    };
    let heuristic = |&(tx, ty): &Entry | {
        (count - tx - ty) as isize  * mean 

    };
    best_first(
        next,
        |&u| u == *goal,
        weight,
        heuristic,
        &mut dist,
        &mut pqueue,
    )
}

fn main() {
    let mut v = [3.0f32, 1.0, 2.0];
    v.sort_by(|a, b| a.total_cmp(b));
    println!("test {:?}", v);
}
