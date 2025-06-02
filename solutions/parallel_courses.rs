// a course that can be taken now should be taken now as i can take as many courses in par
// given a set of courses that have been taken the total time should
//  - max(time) + time(this)
// perhaps we can store in an array of aggregate time to take courses
//
// but this would double count but would be a viable to determine the time to take a course
// what about if we had a logical course at the end which requires all courses
// this simply just rephrames the previous notion of sum time to max time +
// time
// - could i just keep a running time and max + time and process the list?
fn bfs_parallel_courses(edges:&[(usize,usize)],time:&[usize]) -> usize {
    let n = time.len();
    let mut memo:Vec<usize> = vec![0;n];
    let mut requires = vec![0;n];
    let mut enables = vec![vec![];n];
    let mut total = 0;
    let mut available = Vec::with_capacity(n);
    
    for &(pre, crs) in edges {
        requires[crs-1] += 1;
        enables[pre-1].push(crs-1);
    }
    for (crs, prereqs) in requires.iter().enumerate() {
        if *prereqs == 0 {
            available.push(crs);
            memo[crs] = time[crs];
        }
    }
    while let Some(crs) = available.pop() {
        for other in &enables[crs] {
            memo[*other] = memo[*other].max(memo[crs]);
            requires[*other] -= 1;
            if requires[*other] == 0 {
                available.push(*other);
                memo[*other] += time[*other];
            }
        }
        total = total.max(memo[crs]);
    }
    total
}

fn dfs_parallel_courses(edges:&[(usize,usize)],time:&[usize]) -> usize {
    let n = time.len();
    let mut memo:Vec<usize> = vec![0;n];
    let mut requires = vec![vec![];n];
    let mut total = 0;
    
    for &(pre, crs) in edges {
        requires[crs-1].push(pre-1);
    }
    for i in 0..n {
        total = total.max(
            process_edge(i, &requires, &time, &mut memo)
        )
    }
    total    
}

fn process_edge(
    i:usize,
    requires:&Vec<Vec<usize>>,
    time:&[usize],
    memo:&mut Vec<usize>
) -> usize {
    if memo[i] != 0 {
        return memo[i];
    }
    let mut total = 0;
    for j in &requires[i] {
        total=total.max(
            process_edge(*j, requires, time, memo)
        );
    }
    total += time[i];
    memo[i] = total;
    total
}


fn main() {
    assert_eq!(8, dfs_parallel_courses(&[(1,3),(2,3)],&[3,2,5]));
    assert_eq!(12, dfs_parallel_courses(&[(1,5),(2,5),(3,5),(3,4),(4,5)], &[1,2,3,4,5]));
    assert_eq!(8, bfs_parallel_courses(&[(1,3),(2,3)],&[3,2,5]));
    assert_eq!(12, bfs_parallel_courses(&[(1,5),(2,5),(3,5),(3,4),(4,5)], &[1,2,3,4,5]));
}

