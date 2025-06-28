use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

// TODO: Gaps for tomorrow
// Advanced graph algorithms like shortest paths (Dijkstra, Bellman-Ford), MST (Kruskal, Prim), strongly connected components, topological sort?
// Dynamic Programming: You have backtracking & some DP, but do you have solid exposure to:
// DP with bitmasking, tree DP, interval DP, optimization techniques (knapsack variants, DP on graphs)?
//
// Segment trees / Fenwick trees
// Union-find (Disjoint Set Union)
// Balanced BSTs (AVL, Red-Black trees) or heaps in detail?
// Tries (you have trie.rs, great!) but also suffix trees/arrays?


#[derive(Eq, PartialEq)]
struct MinHeapEntry {
    val:usize,
    pos:(usize, usize),
}

impl Ord for MinHeapEntry {
    fn cmp(&self, other:&Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for MinHeapEntry {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn explore(weights:&[Vec<usize>]) -> usize {
    // A*
    let m = weights.len();
    let n = weights[0].len();
    let mut mem = vec![vec![usize::MAX;n];m];
    let mut queue = BinaryHeap::new();
    // over-estimates by 2 but consistent
    queue.push(MinHeapEntry{ val:weights[0][0] + m + n, pos:(0,0) });
    mem[0][0] = weights[0][0];
    _explore_( m, n, &mut queue, weights, &mut mem);
    mem[m-1][n-1]
}


fn _explore_(
    m:usize, n:usize,
    queue:&mut BinaryHeap<MinHeapEntry>,
    weights:&[Vec<usize>],
    mem:&mut [Vec<usize>]) {
    let directions = [(1,0),(!0,0),(0,1),(0,!0)];
    while let Some(p) = queue.pop() {
        if p.pos == (m-1, n-1) {
            return;
        }
        for (dx,dy) in directions {
            let nx = p.pos.0.wrapping_add(dx);
            let ny = p.pos.1.wrapping_add(dy);
            if nx < m && ny < n {
                let cost = mem[p.pos.0][p.pos.1] + weights[nx][ny];
                if cost < mem[nx][ny] {
                    mem[nx][ny] = cost;
                    queue.push(MinHeapEntry { val:cost + m-nx + n - ny, pos:(nx, ny) });
                }
            }
        }
    }
}

//fn explore(weights:&[Vec<i32>]) -> i32 {
//    //djisktra's
//    let m = weights.len();
//    let n = weights[0].len();
//    let mut mem = vec![vec![i32::MAX;n];m];
//    let mut queue = BinaryHeap::new();
//    queue.push(MinHeapEntry{ val:weights[0][0], pos:(0,0) });
//    mem[0][0] = weights[0][0];
//    _explore_( m, n, &mut queue, weights, &mut mem);
//    mem[m-1][n-1]
//}


//fn _explore_(
//    m:usize, n:usize,
//    queue:&mut BinaryHeap<MinHeapEntry>,
//    weights:&[Vec<i32>],
//    mem:&mut [Vec<i32>]) {
//    let directions = [(1,0),(!0,0),(0,1),(0,!0)];
//    while let Some(p) = queue.pop() {
//        if p.pos == (m-1, n-1) {
//            return;
//        }
//        for (dx,dy) in directions {
//            let nx = p.pos.0.wrapping_add(dx);
//            let ny = p.pos.1.wrapping_add(dy);
//            if nx < m && ny < n {
//                let cost = mem[p.pos.0][p.pos.1] + weights[nx][ny];
//                if cost < mem[nx][ny] {
//                    mem[nx][ny] = cost;
//                    queue.push(MinHeapEntry { val:cost, pos:(nx, ny) });
//                }
//            }
//        }
//    }
//}


// fn explore(weights:&[Vec<i32>]) -> i32 {
//     // dfs 
//     let m = weights.len();
//     let n = weights[0].len();
//     let mut mem = vec![vec![i32::MAX;n];m];
//     mem[0][0] = weights[0][0];
//     _explore_(0, 0, m, n, weights, &mut mem);
//     mem[m-1][n-1]
// }


// fn _explore_(x:usize, y:usize, m:usize, n:usize, weights:&[Vec<i32>], mem:&mut [Vec<i32>]) {
//     let directions = [(1,0),(usize::MAX,0),(0,1),(0,usize::MAX)];
//     for (dx,dy) in directions {
//         let nx = x.wrapping_add(dx);
//         let ny = y.wrapping_add(dy);
//         if nx < m && ny < n {
//             let cost = mem[x][y] + weights[nx][ny];
//             if cost < mem[nx][ny] {
//                 mem[nx][ny] = cost;
//                 _explore_(nx, ny, m, n, weights, mem);
//             }
//         }
//     }
// }

fn grid() -> Vec<Vec<usize>> {
    vec![
    vec![1, 3, 1],
    vec![1, 8, 1],
    vec![4, 2, 1]
    ]
}


fn main() {
    let data = grid();
    let res = explore(&data);

    println!("Result {res}");
}
