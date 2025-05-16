use std::collections::HashMap;

fn longest_pallindrome(s:&str) -> &str {
    let n = s.len();
    let mut dp = vec![vec![false;n];n];
    let mut longest = (0,0);
    let b = s.as_bytes();

    for len in 0..n {
        for l in 0..n-len {
            let r = l + len;
            if b[l] == b[r] && (len < 2 || dp[l+1][r-1]) {
                dp[l][r] = true;
                if len > longest.1 - longest.0 {
                    longest = (l,r);
                }
            }
        }
    }
    &s[longest.0..=longest.1]
}

// fn longest_pallindrome(s:&str) -> &str {
//     let n = s.len();
//     let mut cache:HashMap<(usize,usize),(usize,usize)> = HashMap::new();
    
//     let indices = _pallindrome_(0, n-1, s.as_bytes(), &mut cache);
//     &s[indices.0..=indices.1]

// }

// fn _pallindrome_(l:usize,r:usize, s:&[u8], cache:&mut HashMap<(usize,usize), (usize,usize)>) -> (usize,usize) {
//     if let Some(val) = cache.get(&(l,r)) {
//         return *val
//     } else if l == r {
//         cache.insert((l,r),(l,r));
//         return (l,r)
//     };
//     if s[l] == s[r] {
//         // handle odds and evens
//         if r-l == 1 {
//             cache.insert((l,r),(l,r));
//             return (l,r)
//         } else if r - l > 1 {
//             let inner = _pallindrome_(l+1, r-1, s, cache);
//             if l+1 == inner.0 && r-1 == inner.1 {
//                 cache.insert((l,r),(l,r));
//                 return (l, r)
//             }
//         }
//     }
//     let left = _pallindrome_(l, r-1, s, cache);
//     let right = _pallindrome_(l+1, r, s, cache);
//     let max = if left.1 - left.0 < right.1 - right.0 {
//         right
//     } else {
//         left
//     };
//     cache.insert((l,r),max);
//     max
// }

fn main() {
    assert_eq!("badab", longest_pallindrome(&"badab"));
    assert_eq!("dad", longest_pallindrome(&"dadab"));
    assert_eq!("ada", longest_pallindrome(&"ada"));
    assert_eq!("bb", longest_pallindrome(&"cbbd"));
    assert_eq!("abba", longest_pallindrome(&"cabbad"));
}
