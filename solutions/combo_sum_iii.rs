fn combo_sum_iii(k:usize, n:usize) -> Vec<Vec<usize>> {
    if n > 45 || n == 0 { return vec![]; }
    let mut res = vec![];
    fn bt(
        i:usize,
        n:usize,
        k:usize,
        sum:usize,
        count:usize,
        coll:&mut Vec<usize>,
        res:&mut Vec<Vec<usize>>,
    ) {
        if n == sum && k == count {
            res.push(coll.clone());
            return;
        }
        if count >= k { return; }
        for num in i..=9 {
            if num + sum > n { return; }
            coll.push(num);
            bt(num+1, n, k, num + sum, 1 + count, coll, res);
            coll.pop();
        }
    }
    bt(1, n, k, 0, 0, &mut vec![], &mut res);
    res
}

fn main() {
}
