use std::collections::HashSet;
use std::collections::VecDeque;

fn word_break(string:&str, words:Vec<&str>) -> bool{
    let mut dp = vec![false;string.len()+1];
    let words:HashSet<&str> = words.into_iter().collect();
    dp[0] = true;

    for start in 0..string.len() {
        for end in start+1..=string.len() {
            if dp[start] && words.contains(&string[start..end]) {
                dp[end] = true;
            }
        }
    }
    dp[string.len()]
}
// fn word_break(string:&str, words:Vec<&str>) -> bool{
//     let mut stack = VecDeque::from([ 0 ]);
//     let mut explored = vec![false;string.len()+1];
//     let words:HashSet<&str> = words.into_iter().collect();

//     while let Some(idx) = stack.pop_front() {
//         if idx == string.len() { return true; }
//         if explored[idx] { continue; }
//         for edx in idx+1..=string.len() {
//             if words.contains(&string[idx..edx]) {
//                  stack.push_back(edx+1);
//             }
//         }
//         explored[idx] = true;
//     }
//     false
// }

// fn word_break(string:&str, words:Vec<&str>) -> bool{
//     let mut stack = VecDeque::from([ 0 ]);
//     let mut explored = vec![false;string.len()+1];

//     while let Some(idx) = stack.pop_front() {
//         if idx == string.len() { return true; }
//         if explored[idx] { continue; }
//         for edx in idx+1..=string.len() {
//             for &w in &words {
//                 if &string[idx..edx] == w { stack.push_front(edx+1); }
//             }
//         }
//         explored[idx] = true;
//     }
//     false
// }


fn main() {
    println!(
        "result {:?}",
        1234
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
