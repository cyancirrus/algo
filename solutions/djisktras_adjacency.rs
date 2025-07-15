use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone)]
pub struct WeightEdge {
    src:usize,
    tgt:usize,
    weight:usize,
}

#[derive(Eq, PartialEq)]
pub struct MinHeapNode {
    node:usize,
    cost:usize,
}

impl MinHeapNode {
    fn new(node:usize, cost:usize) -> Self {
        Self { node, cost }
    }
}

impl Ord for MinHeapNode {
    fn cmp(&self, other:&Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for MinHeapNode {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn build_adj_list(graph:&Vec<WeightEdge>) ->  HashMap<usize, Vec<WeightEdge>> {
    let mut hgraph:HashMap<usize, Vec<WeightEdge>> = HashMap::new();
    for wedge in graph {
        hgraph.entry(wedge.src).or_default().push(wedge.clone());
    }
    hgraph
}


fn djisktras(start:usize, end:usize, hgraph:&HashMap<usize, Vec<WeightEdge>>) -> usize {
    let mut queue:BinaryHeap<MinHeapNode> = BinaryHeap::new();
    let mut cost: HashMap<usize, usize> = HashMap::new();
    queue.push(MinHeapNode::new(start, 0)); 
    cost.entry(start).or_insert(0);
    while let Some(pos) = queue.pop() {
        if pos.node == end {
            return pos.cost
        }
        if let Some(neighs) = hgraph.get(&pos.node) {
            for edge in neighs {
                let new_cost= pos.cost + edge.weight;
                if new_cost < *cost.get(&edge.tgt).unwrap_or(&usize::MAX) {
                    cost.insert(edge.tgt, new_cost);
                    queue.push(MinHeapNode::new(edge.tgt, new_cost));
                }
            }
        } else {
            continue;
        }
    }
    usize::MAX
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
    let hgraph = build_adj_list(&edges);
    let dist = djisktras(0, 3, &hgraph);
    println!("Distance from 0 to 3: {:?}", dist);
}


