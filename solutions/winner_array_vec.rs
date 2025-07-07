use std::collections::VecDeque;


fn winner_array_vec(x:&mut [usize], k:usize) -> usize {
    let n = x.len();
    let mut wins = vec![0;n];
    for i in 0..n-1 {
        if x[i+1] < x[i] {
            x.swap(i+1, i);
        }
        wins[x[i+1] - 1] += 1;
        if wins[x[i+1] -1] == k {
            return x[i+1]; 
        }
    }
    x[n-1]
}

// needs clarificying quesiton on like if players are 1..n
// but assuming they are
fn winner_array(x:&mut VecDeque<usize>, k:usize) -> usize {
    let n = x.len();
    let mut wins = vec![0;n];
    for _ in 0..n {
        if x[1] < x[0] {
            x.swap(1, 0);
        }
        if let Some(f) = x.pop_front() {
            x.push_back(f);
        }
        wins[x[0] - 1] += 1;
        if wins[x[0] -1] == k {
            return x[0]; 
        }
    }
    x[0]
}


fn main() {
    // let result = winner_array(&mut VecDeque::from(vec![2,1,3,5,4,6,7]), 2);
    // println!("Result {result:}");
    // let result = winner_array(&mut VecDeque::from([3,2,1]), 10);
    // println!("Result {result:}");
    // let result = winner_array(&mut VecDeque::from([1,2,3]), 10);
    // println!("Result {result:}");
    // println!("------------------");
    // let result = winner_array_vec(&mut [2,1,3,5,4,6,7], 2);
    // println!("Result {result:}");
    let result = winner_array_vec(&mut [2,1,3,5,4], 2);
    println!("Result {result:}");
    // let result = winner_array_vec(&mut [3,2,1], 10);
    // println!("Result {result:}");
    // let result = winner_array_vec(&mut [1,2,3], 10);
    // println!("Result {result:}");
}
