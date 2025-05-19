use std::mem;

fn common_substring<'a>(s1:&'a str, s2:&'a str) -> &'a str {
    if s1.len() < s2.len() {
        _common_substring_(s2,s1)
    } else {
        _common_substring_(s1,s2)
    }
}
fn _common_substring_<'a>(s1:&'a str, s2:&'a str) -> &'a str {
    let chars1:Vec<_> = s1.chars().collect();
    let chars2:Vec<_> = s2.chars().collect();
    let mut common_chars = 0;
    let mut end_char = 0;
    let mut prev= vec![0;chars2.len()+1];
    let mut curr= vec![0;chars2.len()+1];

    for i in 1..=chars1.len() {
        for j in 1..=chars2.len() {
            if chars1[i-1] == chars2[j-1] {
                curr[j] = 1 + prev[j-1];
                if curr[j] > common_chars {
                    common_chars = curr[j];
                    end_char=i;
                }
            }
        }
        mem::swap(&mut prev,&mut curr);
    }
    &s1[end_char-common_chars as usize..end_char]
}


fn main() {
    assert_eq!("hell", common_substring("hell", "hello world"));
    assert_eq!("ello", common_substring("ello", "hello world"));
    assert_eq!("abc", common_substring("xyzabc", "123abc456"));
    assert_eq!("", common_substring("abc", "def"));
    assert_eq!("abc", common_substring("ababc", "abcd"));

}

// fn common_substring<'a>(s1:&'a str, s2:&'a str) -> &'a str {
//     let chars1:Vec<_> = s1.chars().collect();
//     let chars2:Vec<_> = s2.chars().collect();
//     let pad1 = s1.len() + 1;
//     let pad2 = s2.len() + 1;
//     let mut max = 0;
//     let mut positions = (0,0);
//     let mut dp = vec![vec![0;pad2];pad1];

//     for i in 1..pad1 {
//         for j in 1..pad2 {
//             if chars1[i-1] == chars2[j-1] {
//                 dp[i][j] = 1 + dp[i-1][j-1];
//                 if dp[i][j] > max {
//                     max = dp[i][j];
//                     positions=(i,j);
//                 }
//             }
//         }
//     }
//     &s1[positions.0-max as usize..positions.0]
// }
