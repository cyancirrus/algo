use std::collections::HashMap;

fn distinct_subseqopt(a:&str, b:&str) -> usize {
    if b.is_empty() { return 0; }
    const ASCII:usize = 256;
    let m = b.len();
    let mut dp = vec![0;m+1];
    dp[0] = 1;
    let bb = b.as_bytes();
    let mut pos = vec![vec![];ASCII];
    for (i, &c) in bb.iter().enumerate() {
        pos[c as usize].push(i);
    }
    for &c in a.as_bytes() {
        for &j in pos[c as usize].iter().rev() {
            dp[j+1] += dp[j]
        }
    }
    dp[m]
}


fn distinct_subsequences(a:&str, b:&str) -> usize {
    if b.is_empty() { return 0; }
    let n = b.len();
    let mut state = vec![0;n+1];
    state[0]=1;
    let mut map_state:HashMap<u8, Vec<usize>> = HashMap::with_capacity(n);
    let a = a.as_bytes();
    let b = b.as_bytes();
    for (i, &c) in b.iter().enumerate() {
        map_state.entry(c).or_default().push(i)
    }
    for &c in a {
        if let Some(m) = map_state.get(&c) {
            for &u in m.iter().rev() {
                state[u+1] += state[u]
            }
        }
    }
    state[n]
}

fn main() {
}
