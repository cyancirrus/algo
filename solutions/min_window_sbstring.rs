use std::collections::HashMap;


fn min_window_substr<'a>(s:&'a str, t:&'a str) -> &'a str {
    let sb = s.as_bytes();
    let tb = t.as_bytes();

    let mut freqs: HashMap<u8, usize> = HashMap::new();
    let mut seen: HashMap<u8, usize> = HashMap::new();
    for &b in tb { *freqs.entry(b).or_default()+=1 } 
    let need = freqs.len();
    let mut have = 0;
    let mut cur_l = 0;
    let mut start = 0;
    let mut end = usize::MAX;
    
    for (cur_r, &b) in sb.iter().enumerate() {
        if freqs.contains_key(&b) {
            *seen.entry(b).or_default()+=1;
            if seen[&b] == freqs[&b] {
                have += 1;
            }
        }
        while have == need {
            if cur_r - cur_l < end - start {
                start = cur_l;
                end = cur_r;
            }
            if freqs.contains_key(&sb[cur_l]) {
                *seen.get_mut(&sb[cur_l]).unwrap() -= 1;
                if seen[&sb[cur_l]] < freqs[&sb[cur_l]] {
                    have -= 1;
                }
            }
            cur_l += 1;
        }
    }
    if end < sb.len() { &s[start..=end] } else { "" }
}


fn main() {
    println!("min window sub {:?}", min_window_substr("ADOBECODEBANC", "ABC"));
}
