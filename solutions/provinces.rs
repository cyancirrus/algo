use std::collections::LinkedList;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::mem;

fn number_of_provinces(graph:&[Vec<bool>]) -> u32 {
    // square matrix
    let n = graph.len();
    let mut p = 0;
    let mut seen = vec![false;n];
    for i in 0..n {
        if !seen[i] {
            p+=1;
            // bidirectional graph so i <= j
            _provinces_(i,  &mut seen, graph);
        }
    }
    p as u32
}
fn _provinces_(r:usize, seen:&mut Vec<bool>, graph:&[Vec<bool>]) {
    seen[r]=true;
    // depth first search as it goes to the bottom of the callgraph
    for j in 0..graph.len() {
        if !seen[j] && graph[r][j] {
            _provinces_(j, seen, graph);
        }
    }
}


fn main() {
    assert_eq!(2, number_of_provinces(&[vec![true,true,false],vec![true,true,false],vec![false,false,true]]));
    assert_eq!(3, number_of_provinces(&[vec![true,false,false],vec![false,true,false],vec![false,false,true]]));
}
