use std::collections::HashMap;
use std::collections::HashSet;

type Adj = HashMap<usize,HashSet<usize>>;

fn detect_loop(req_list:&[(usize,usize)])-> bool {
    let mut graph:Adj = HashMap::new();
    let mut visited:HashMap<usize, bool> = HashMap::new();
    let mut on_stack:HashMap<usize, bool> = HashMap::new();
    for &(course, prereq) in req_list {
        graph.entry(course).or_default().insert(prereq);
    }
    let courses = graph.keys().copied().collect::<Vec<usize>>();
    for crs in courses {
        if dfs(crs, &mut graph, &mut visited, &mut on_stack) {
            return true;
        }
    }
    false
}

fn dfs(
    course:usize,
    graph:&mut Adj,
    visited:&mut HashMap<usize, bool>,
    on_stack:&mut HashMap<usize, bool>
) -> bool {
    // have looped back-around
    if *on_stack.get(&course).unwrap_or(&false) { return true };
    // already evaluated
    if *visited.get(&course).unwrap_or(&false) {return false };

    visited.insert(course, true);
    on_stack.insert(course, true);
    let neighbors = graph[&course].clone();
    for neighbor in neighbors {
        if dfs(neighbor, graph, visited, on_stack) {
            return true;
        }
    }
    // clears the on_stack vars
    on_stack.insert(course, false);
    false
}

fn course_schedule(n:usize, req_list:&[(usize,usize)])-> bool {
    let mut requires:Adj = HashMap::new();
    let mut enables:Adj = HashMap::new();
    for &(course, prereq) in req_list {
        requires.entry(course).or_default().insert(prereq);
        requires.entry(prereq).or_default(); // ensure node is present
        enables.entry(prereq).or_default().insert(course);
        enables.entry(course).or_default();
    }
    let courses = requires.keys().copied().collect::<Vec<usize>>();
    for crs in courses {
        cdfs(crs, &mut requires, &enables);
    }
    let completed = requires.values().filter(|reqs| reqs.is_empty()).count();
    n == completed
}

fn cdfs(crs:usize, reqs:&mut Adj, enbl:&Adj) {
    if reqs[&crs].is_empty() { 
        for c in &enbl[&crs] {
            if let Some(prereqs) = reqs.get_mut(&c) {
                prereqs.remove(&crs);
            }
            cdfs(*c, reqs, enbl);
        }
    }
}


fn main() {
    assert_eq!(false, course_schedule(2, &[(1,0),(0,1)]));
    assert_eq!(false, course_schedule(1, &[(1,0),(0,1)]));
    assert_eq!(false, course_schedule(1, &[(0,1)]));
    assert_eq!(true, course_schedule(2, &[(0,1)]));
}
