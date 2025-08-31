
fn maximum_gap(nums:&[usize]) -> usize {
    let mut n = nums.len();
    let mut buckets = vec![(None, None);n-1];
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for &n in nums {
        min = min.min(n);
        max = max.max(n);
    }
    let gap = (max - min)/(n-1);
    for &n in nums {
        if n == min || n == max { continue; }
        let idx = (n - min) /gap;
        match buckets[idx] {
            (Some(mn), Some(mx)) => {
                buckets[idx] = ( Some(n.min(mn)), Some(n.max(mx)),);
            },
            (None, None) => {
                buckets[idx] = (Some(n), Some(n));
            },
            _ => {},
        }
    }
    let mut prev = min;
    let mut res = 0;
    for b in buckets {
        match b {
            (Some(bmin), Some(bmax)) => {
                res = res.max(bmin - prev);
                prev = bmax;
            },
            _ => { continue; }
        }
    }
    res = res.max(max - prev);
    res

}


fn main() {
    println!(
        "result {:?}",
        maximum_gap(&[1,3,6,9])
    );
}
