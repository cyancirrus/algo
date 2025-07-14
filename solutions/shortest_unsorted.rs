fn shortest_unsorted_subarray(x:&[usize]) -> &[usize] {
    let n = x.len();
    let (mut l, mut r) = (0, n-1);
    // find the bounds
    while l < n-1 && x[l] <= x[l+1] {
        l+=1;
    }
    if l == n-1 { return &[] };
    while r > 0 && x[r] <= x[r-1] {
        r-=1;
    }
    // find the lowest and highest vals in the unsorted
    let mut lowest = usize::MAX;
    let mut highest = usize::MIN;
    for idx in l..=r {
        lowest = lowest.min(x[idx]);
        highest = highest.max(x[idx]);
    }
    // adjust the boundaries as post sort could adjust boundaries
    while l > 0 && lowest < x[l-1] {
        l-=1;
    }
    while r < n-1 && x[r+1] < highest {
        r+=1;
    }
    &x[l..r]
}


fn main() {
    println!("Shortest subarray {:?}", shortest_unsorted_subarray(&[2,6,4,8,10,9,15]));
}
