use std::mem;

fn interleaving_strings(s1:&str, s2:&str, s3:&str) -> bool {
    // this would need to like u need to look at feasability region and do special thigns
    // would need to reinvestigate this and like implement the lp like constraints
    let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
    let (m, n, p) = (s1.len(), s2.len(), s3.len());
    if m + n < p { return false; }
    let mut prev = vec![false;n+1];
    let mut curr = vec![false;n+1];
    prev[0] = true;
    for j in 1..=n {
        prev[j] = prev[ j - 1 ] && s2[ j - 1 ] == s3[ j - 1 ];
    }

    for i in 1..=m.min(p) {
        curr[0] = prev[0] && s1[i-1] == s3[i-1];
        for j in 1..=n.min(p) {
            curr[j] = {
                (prev[j] && s1[ i - 1 ] == s3[ i + j - 1 ])
                | (curr[ j-1 ] && s2[ j - 1 ] == s3[ i + j - 1 ])
            };
        }
        mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}


fn interleaving_strings_dfs(s1:&str, s2:&str, s3:&str) -> bool {
    let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
    let m = s1.len();
    let n = s2.len();
    let mut memo = vec![vec![true;n+1];m+1];
    dfs(0, 0, s1, s2, s3, &mut memo)
}

fn dfs(
    i:usize, j:usize,
    s1:&[u8], s2:&[u8], s3:&[u8],
    memo:&mut Vec<Vec<bool>>,
) -> bool {
    let n = s3.len();
    if i + j == n {
        return true;
    }
    if !memo[i][j] {
        return false;
    }
    if i < s1.len() && s1[i] == s3[i + j] && dfs(i+1, j, s1, s2, s3, memo){
        return true;
    }
    if j < s2.len() && s2[j] == s3[i + j] && dfs(i, j+1, s1, s2, s3, memo){
        return true;
    }
    memo[i][j] = false;
    false 
}


// fn interleaving_strings(s1:&str, s2:&str, s3:&str) -> bool {
//     let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
//     let m = s1.len();
//     let n = s2.len();
//     let mut memo = vec![vec![None;m+1];n+1];
//     dfs(0, 0, s1, s2, s3, &mut memo)
// }

// fn dfs(
//     i:usize, j:usize,
//     s1:&[u8], s2:&[u8], s3:&[u8],
//     memo:&mut Vec<Vec<Option<bool>>>,
// ) -> bool {
//     let n = s3.len();
//     if i + j == n {
//         return true;
//     }
//     if let Some(ans) = memo[i][j] {
//         return ans;
//     }

//     if i < s1.len() && s1[i] == s3[i + j] && dfs(i+1, j, s1, s2, s3, memo){
//         memo[i+1][j] = Some(true);
//         return true;
//     }
//     if j < s2.len() && s2[j] == s3[i + j] && dfs(i, j+1, s1, s2, s3, memo){
//         memo[i][j+1] = Some(true);
//         return true;
//     }
//     memo[i][j] = Some(false);
//     false 
// }








// fn interleaving_strings(s1:&str, s2:&str, s3:&str) -> Option<bool> {
//     let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
//     let n = s3.len() + 1;
//     let mut memo = vec![vec![None;n+1];n+1];
//     bt(s1, s2, s3, 0, 0, 0, &mut memo);
//     memo[n+1][n+1]
// }

// fn bt(
//     s1:&[u8], s2:&[u8], s3:&[u8],
//     i:usize, j:usize, k:usize,
//     memo:&mut Vec<Vec<Option<bool>>>,
// ) -> Option<bool> {
//     let n = s3.len();
//     if k == n {
//         memo[n][n] = Some(true);
//         return Some(true);
//     } else if let Some(res) = memo[n][n] {
//         // only set when result
//         return Some(res);
//     }

//     let mut idx = i;
//     let mut kdx = k;
//     match memo[i][j] {
//         Some(result) => return Some(result),
//         None => {},
//     }
//     while s1[idx] == s3[kdx] && idx < s1.len() && kdx < s3.len() {
//         memo[idx+1][j+1] = bt(s1, s2, s3, idx, j, kdx, memo);
//         idx+=1;
//         kdx+=1;
//     }
//     let mut jdx = j;
//     let mut kdx = k;
//     while s1[i] == s3[idx] && jdx < s2.len() && kdx < s3.len(){
//         memo[i+1][jdx+1] = bt(s1, s2, s3, i, jdx, kdx, memo);
//         jdx+=1;
//         kdx+=1;
//     }
//     memo[i][j]
// }


fn main() {
    // println!("test {:?}" , wordbreak("leetcode", vec!["leet","code"]));
    // println!("test {:?}" , wordbreak_iter("leetcode", vec!["leet","code"]));
    // permutation_sequence(4, 9);
}
