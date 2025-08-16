use std::collections::{HashSet, HashMap};
fn wordbreak(s:&str, wordbank:Vec<&str>) -> bool {
    let worddict:HashSet<&str> = wordbank.into_iter().collect();
    let mut memo= HashMap::new();
    fn bt<'a>(s:&'a str, worddict:&'a HashSet<&str>, memo:&mut HashMap<&'a str, bool>) -> bool {
        if s.is_empty() { return true; }
        if let Some(&ans) = memo.get(s) { return ans; }
        for i in 0..=s.len() {
            if worddict.contains(&s[0..i]) && bt(&s[i..], worddict, memo) {
                memo.insert(&s, true);
                return true
            }
        }
        memo.insert(s, false);
        false
    }
    bt(s, &worddict, &mut memo)
}

fn wordbreak_iter(s:&str, wordbank:Vec<&str>) -> bool {
    let n = s.len();
    let worddict: HashSet<&str> = wordbank.into_iter().collect();
    let mut dp = vec![false;n+1];
    dp[0] = true;
    
    for i in 0..n {
        if dp[i] {
            for &w in &worddict {
                if i + w.len() <= n && s[i..i+w.len()] == *w {
                    dp[i+w.len()] = true;
                }
            }
        }
    }
    println!("dp {:?}", dp);
    dp[n]
}




fn main() {
    println!("test {:?}" , wordbreak("leetcode", vec!["leet","code"]));
    println!("test {:?}" , wordbreak_iter("leetcode", vec!["leet","code"]));
    // permutation_sequence(4, 9);
}
