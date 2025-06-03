use std::collections::HashMap;
use std::collections::HashSet;

// so essentially we have like
//
// 1 -> 2, 1-> 3 => !(2 -> 3)
//
// 1: (2, 3) <- could look to make sure that it's not in these
// which means we should just be able to store this in a set
//
// for n-ary we just change to check for is superset
//
// there are no requirements about it not being n-ary
// a hash set sounds nice but it doesn't fit well
//
// nodes are 1..=n
//
// i should just be able to follow and if i hit the same node i remove the last visisted

fn find_redundant_edge(edges:&[(usize, usize)]) -> (usize, usize) {
    let mut nodes:HashMap<usize, Vec<usize>> = HashMap::new();
    let mut visited:HashSet<usize> = HashSet::new();
    for &(src, tgt) in edges {
        visited.clear();
        if dfs(src, tgt, &mut nodes, &mut visited) {
            return (src, tgt)
        }
        nodes.entry(src).or_default().push(tgt);
        nodes.entry(tgt).or_default().push(src);
    }
    (usize::MAX, usize::MAX)
    
}

fn dfs(src:usize, target:usize, nodes:&HashMap<usize,Vec<usize>>, visited:&mut HashSet<usize>) -> bool {
    println!("src target ({}, {})", src,target);
    if src == target { return true };
    if !visited.insert(src) { return false };
    if let Some(neighbors) = nodes.get(&src) {
        for &n in neighbors {
            if dfs(n, target, nodes, visited) {
                return true;
            }
        }
    }
    visited.remove(&src);
    false
}

fn main() {
    assert_eq!((2,3), find_redundant_edge(&[(1,2),(1,3),(2,3)]));
    assert_eq!((1,4), find_redundant_edge(&[(1,2),(2,3),(3,4),(1,4),(1,5)]));
}
// fn find_redundant_edge(edges:&[(usize, usize)]) -> (usize, usize) {
//     let mut nodes:HashMap<usize, Vec<usize>> = HashMap::new();

//     for &(src, tgt) in edges {
//         nodes.entry(src).or_default().push(tgt);
//         nodes.entry(tgt).or_default().push(src);
//     }
//     let n = nodes.len();
//     let (prev, curr) = edges[0];
//     let mut visited = HashSet::with_capacity(n);
//     visited.insert(prev);
    
//     dfs(prev, curr, &nodes, &mut visited).unwrap()
// }

// fn dfs(prev:usize, curr:usize, nodes:&HashMap<usize,Vec<usize>>, visited:&mut HashSet<usize>) -> Option<(usize, usize)> {
//     if visited.contains(&curr) { return Some((prev, curr)) }
//     visited.insert(curr); 
//     for connect in &nodes[&curr] {
//         if let Some(found) = dfs(curr, *connect, nodes, visited) {
//             return Some(found);
//         }
//     }
//     visited.remove(&curr);
//     None
// }
