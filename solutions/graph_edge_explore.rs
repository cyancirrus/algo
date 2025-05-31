use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::Instant;

type Adjacency = HashMap<usize, HashSet<usize>>;

fn find_order_bit(n:usize, edges:&[(usize,usize)]) -> Vec<usize> {
    assert!(n < 64);
    // dependencies k, # of prereqs
    let mut in_degree:Vec<u64> = vec![0;n];
    // enables n, others
    let mut out_degree:Vec<u64> = vec![0;n];
    let mut available: VecDeque<usize> = VecDeque::new();
    let mut ordering:Vec<usize> = Vec::new();
    
    for &(o, e) in edges {
        out_degree[e] |= 1<<o;
        in_degree[o] += 1;
    }
    for (n, e) in in_degree.iter().enumerate() {
        if *e == 0 { available.push_back(n);}
    } 
    while let Some(node) = available.pop_front() {
        ordering.push(node);
        let mut bitmask = out_degree[node];
        while bitmask != 0 {
            let lsb = bitmask.trailing_zeros() as usize;
            in_degree[lsb] -= 1;
            in_degree[lsb] &= !(1<<node);
            if in_degree[lsb] == 0 {
                available.push_back(lsb);
            }
            bitmask &= !(1<<lsb as u64);
        }
    }
    if ordering.len() != n {
        ordering.clear();
    }
    ordering
}

fn find_order(adj:&[(usize,usize)]) -> Vec<usize> {
    // dependencies
    let mut edges:Adjacency = HashMap::new();
    let mut ordering:Vec<usize> = Vec::new();
    let mut resolve:HashSet<usize> = HashSet::new();
    let mut seen:HashSet<usize> = HashSet::new();
    
    for &(e, o) in adj {
        edges.entry(e).or_default().insert(o);
        edges.entry(o).or_default();
    }
    for &n in edges.keys() {
        if !resolve.contains(&n) {
            collect(n, &edges, &mut ordering, &mut seen, &mut resolve);
        }
    }
    if ordering.len() != edges.len() {
        ordering.clear();
    }
    ordering
}

fn collect(
    node:usize,
    edges:&Adjacency,
    ordering:&mut Vec<usize>,
    seen:&mut HashSet<usize>,
    resolve:&mut HashSet<usize>,
) {
    if seen.contains(&node) {
        // already processed dependency
        return
    }
    seen.insert(node);
    for &n in &edges[&node] {
        collect(n, edges, ordering, seen, resolve);
    }
    if resolve.is_superset(&edges[&node]) {
        resolve.insert(node);
        ordering.push(node);
    }
}


fn main() {
    // let test = &[(1,0),(2,0),(3,1),(3,2)];
    // println!("Result {:?}", find_order(test));
    let test = &[(1,0),(2,0),(3,1),(3,2)];
    let start = Instant::now();
    println!("Result {:?}", find_order_bit(4,test));
    println!("Duration {:?}", start.elapsed());

}

// fn find_order(edges:&[(usize,usize)]) -> Vec<usize> {
//     // dependencies k, # of prereqs
//     let mut in_degree:HashMap<usize, usize> = HashMap::new();
//     // enables n, others
//     let mut out_degree:HashMap<usize, Vec<usize>> = HashMap::new();
//     let mut available: VecDeque<usize> = VecDeque::new();
//     let mut ordering:Vec<usize> = Vec::new();
    
//     for &(o, e) in edges {
//         out_degree.entry(e).or_default().push(o);
//         out_degree.entry(o).or_default();
//         *in_degree.entry(o).or_insert(0) += 1;
//         in_degree.entry(e).or_insert(0);
//     }
//     for (n, e) in &in_degree {
//         if *e == 0 { available.push_back(*n);}
//     } 

//     while !available.is_empty() {
//         let c = available.pop_front().unwrap();
//         ordering.push(c);
//         for e in &out_degree[&c] {
//             if let Some(d) = in_degree.get_mut(&e) {
//                 *d -= 1;
//                 if *d == 0 {
//                     available.push_back(*e);
//                 }
//             }
//         }
//     }
//     if ordering.len() != in_degree.len() {
//         ordering.clear();
//     }
//     ordering
// }

// fn find_order(edges:&[(usize,usize)]) -> Vec<usize> {
//     // dependencies
//     let mut out_edge:Adjacency = HashMap::new();
//     // enables
//     let mut in_edge:Adjacency = HashMap::new();
    
//     for &(e, o) in edges {
//         out_edge.entry(e).or_default().insert(o);
//         out_edge.entry(o).or_default();
//         in_edge.entry(o).or_default().insert(e);
//         in_edge.entry(e);
//     }
//     collect(&mut out_edge, &mut in_edge)
// }

// fn collect(out_edge:&mut Adjacency, in_edge:&mut Adjacency) -> Vec<usize> {
//     let n = out_edge.len();
//     let mut ordering:Vec<usize> = Vec::with_capacity(n);
//     let mut available = out_edge.iter().
//         filter(|(_, v)| v.is_empty())
//         .map(|(&k, _ )| k)
//         .collect::<Vec<usize>>();
    
//     while !available.is_empty() {
//         let c = available.pop().unwrap();
//         ordering.push(c);
//         for e in &in_edge[&c] {
//             if let Some(d) = out_edge.get_mut(&e) {
//                 d.remove(&c);
//                 if d.is_empty() {
//                     available.push(*e);
//                 }
//             }
//         }
//     }
//     if n == ordering.len() {
//         ordering
//     } else {
//         vec![]
//     }
// }
// iterative
// fn find_order(adj:&[(usize,usize)]) -> Vec<usize> {
//     // dependencies
//     if adj.len() == 0 { return vec![] };
//     let mut edges:Adjacency = HashMap::new();
//     let mut ordering:Vec<usize> = Vec::new();
//     let mut resolve:HashSet<usize> = HashSet::new();
//     let mut nodes:VecDeque<usize> = VecDeque::new();
    
//     for &(e, o) in adj {
//         edges.entry(e).or_default().insert(o);
//         edges.entry(o).or_default();
//         if !nodes.contains(&e) { nodes.push_back(e); }
//         if !nodes.contains(&o) { nodes.push_back(o); }

//     }
//     while nodes.len() > 0 {
//         if let Some(n) = nodes.pop_back() {
//             if resolve.is_superset(&edges[&n]) {
//                 resolve.insert(n);
//                 ordering.push(n);
//             } else {
//                 nodes.push_front(n);
//             }
//         }
//     }
//     ordering
// }
