use std::collections::HashSet;
use std::collections::VecDeque;

fn non_overlapping(intervals:&mut [(usize, usize)]) -> usize {
    if intervals.is_empty() { return 0; }
    intervals.sort_by(|x, y| x.0.cmp(&y.0).then_with(|| y.1.cmp(&x.1)));
    let mut count = 0; 
    for cur in 1..intervals.len() {
        if intervals[cur-1].1 > intervals[cur].0 { count +=1; }

    }
    count
}

fn non_overlapping_vii(intervals:&mut [(usize, usize)]) -> usize {
    if intervals.is_empty() { return 0; }
    intervals.sort_by(|x, y| x.0.cmp(&y.0));
    let mut count = 0; 
    let mut end = intervals[0].1;
    for cur in 1..intervals.len() {
        if intervals[cur-1].1 > intervals[cur].0 {
            count +=1;
            end = end.min(intervals[cur].1);
        } else {
            end = intervals[cur].1;
        }

    }
    count
}


fn main() {
    println!(
        "result {:?}",
        non_overlapping( &mut [( 1,2 ),( 1,2 ),( 1,2 )])
        // is_interleaving_dp("aabcc", "abcd", "aabcc")
        // maximum_mul_subarray(&[2,-1])
        // can_be_valid("))()))","010100")
        // can_be_valid("))","01")
        // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    );
    // println!(
    //     "result {:?}",
    //     // maximum_mul_subarray(&[2,-1])
    //     maximum_mul_subarray(&[2,3,-2,4])
    //     // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    // );
    // println!(
    //     "result {:?}",
    //     maximum_add_subarray(&[5,4,-1,7,8])
    //     // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    // );
}
