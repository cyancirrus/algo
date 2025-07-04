use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

pub struct WeightEdge {
    src:usize,
    tgt:usize,
    weight:isize,
}

// could iterate through the things to get the minimum between nodes
//
    
fn create_adj_matrix(n:usize, alist:&Vec<WeightEdge>) -> Vec<Vec<isize>> {
    let mut data = vec![vec![isize::MAX;n];n];
    for w in alist {
        data[w.src][w.tgt] = data[w.src][w.tgt].min(w.weight);
    }
    data
}

fn floydwarshall(amatrix:&mut Vec<Vec<usize>>) -> &Vec<Vec<usize>> {
    let n = amatrix.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                amatrix[i][j] = amatrix[i][j].min(amatrix[i][k] + amatrix[k][j])                 
            }
        }
    }
    amatrix
}


fn bellmanford(
    n:usize,
    src:usize,
    alist:&Vec<WeightEdge>
) -> Result<Vec<isize>, &'static str> {
    let mut dist = vec![isize::MAX;n];
    dist[src] = 0;

    for _ in 0..n-1 {
        let mut updated = false;
        for w in alist {
            if dist[w.src] < isize::MAX {
                if let Some(sum) = dist[w.src].checked_add(w.weight) {
                    if sum < dist[w.tgt] {
                        dist[w.tgt] = dist[w.src] + w.weight;
                        updated = true;
                    }
                }
            }
        }
        if !updated {
            break;
        }
    }
    for w in alist {
        if dist[w.src] < isize::MAX {
            if let Some(sum) = dist[w.src].checked_add(w.weight) {
                if sum < dist[w.tgt] {
                    return Err("Negative cycle detected");
                }
            }
        }
    }
    return Ok(dist)
}

fn grid() -> Vec<WeightEdge> {
    vec![
    WeightEdge {src:0, tgt:1, weight:1 },
    WeightEdge {src:1, tgt:2, weight:2 },
    WeightEdge {src:1, tgt:4, weight:4 },
    WeightEdge {src:1, tgt:5, weight:3 },
    WeightEdge {src:1, tgt:5, weight:6 },
    WeightEdge {src:2, tgt:4, weight:5 },
    WeightEdge {src:3, tgt:0, weight:2 },
    WeightEdge {src:4, tgt:1, weight:3 },
    WeightEdge {src:5, tgt:3, weight:7 },
    WeightEdge {src:5, tgt:7, weight:1 },
    WeightEdge {src:6, tgt:7, weight:1 },
    WeightEdge {src:7, tgt:5, weight:2 },
    ]
}


fn main() {
    let edges = grid();
}


