fn find_plateus(n:&[u32]) -> Vec<usize> {
    if n.len() < 2 { return vec![];}
    let mut res = vec![];
    let mut left_increase = false;
    let mut cands = vec![];
    for i in 1..n.len() {
        if n[i-1] < n[i] {
            left_increase = true;
            cands.clear();
            cands.push(i);
        } else if n[i-1] == n[i] && left_increase   {
            cands.push(i);
        } else if n[i] < n[i-1] {
            res.extend(cands.drain(..));
            left_increase = false;
        }
    }
    res
}


fn find_relmax(n:&[u32]) -> Vec<usize> {
    let mut res = vec![];
    if n.len() < 2 { return vec![];}
    for i in 1..n.len() - 1 {
        if n[i-1] < n[i] && n[i] > n[i+1] {
            res.push(i);
        }
    }
    res
}


fn main() {
    println!("test {:?}", find_relmax(&[1,2,1,3,5,6,4]));
    println!("test {:?}", find_plateus(&[1,2,1,3,5,6,6,4]));
}
