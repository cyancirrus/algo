use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::Reverse;

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


fn shortest_path_binary_djikstras(bmatrix:&[Vec<usize>]) -> isize {
    if bmatrix.is_empty() || bmatrix[0].is_empty() { return -1 }
    let n = bmatrix[0].len();
    if bmatrix[0][0] != 0 || bmatrix[n-1][n-1] != 0 { return -1 }
    let mut dp = vec![vec![usize::MAX;n];n];
    let mut pqueue:BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    dp[0][0] = 1;
    pqueue.push(Reverse(Node { d:1, p:(0,0) }));
    
    while let Some(Reverse(Node {d, p:(x, y)})) = pqueue.pop() {
        if d > dp[x][y] { continue; }
        if x == n-1 && y == n-1 { return d as isize; }
        
        for (dy, dx) in [
            (1, 0), (!0, 0), (0, 1), (0, !0),
            (1, 1), (!0, !0), (!0, 1), (1, !0)
        ] {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            if nx < n && ny < n {
                if d + 1 < dp[nx][ny] && bmatrix[nx][ny] == 0 {
                    dp[nx][ny] = d + 1;
                    pqueue.push(Reverse(Node { d: d + 1, p:(nx, ny) }));
                }
            }
        }

    }
    -1
}

fn shortest_path_binary(bmatrix:&[Vec<usize>]) -> isize {
    if bmatrix.is_empty() || bmatrix[0].len() == 0 { return -1 }
    let n = bmatrix[0].len();
    if bmatrix[0][0] != 0 || bmatrix[n-1][n-1] != 0 { return -1 }
    let mut dp = vec![vec![usize::MAX;n];n];
    let mut queue = VecDeque::from([(0usize,0usize),]);
    dp[0][0] = 1;
    
    while let Some((x, y)) = queue.pop_front() {
        if x == n-1 && y == n-1 {
            return dp[x][y] as isize; 
        }
        for (dy, dx) in [
            (1, 0), (!0, 0), (0, 1), (0, !0),
            (1, 1), (!0, !0), (!0, 1), (1, !0)
        ] {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            if nx < n && ny < n && bmatrix[nx][ny] == 0 && dp[nx][ny] == usize::MAX {
                dp[nx][ny] = dp[x][y] + 1;
                queue.push_back((nx, ny));
            }
        }
    }
    -1
}

fn main() {
    println!("shortest path binary {:?}", shortest_path_binary(&[vec![0,0,0],vec![1,1,0],vec![1,1,0]]));
    println!("shortest path binary {:?}", shortest_path_binary_djikstras(&[vec![0,0,0],vec![1,1,0],vec![1,1,0]]));
}
