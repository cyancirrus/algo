use std::collections::LinkedList;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::mem;

fn clear_seen(status:& mut HashMap<&str, (i32,i32)>) {
    for val in status.values_mut() {
        val.0 = 0;
    }
}


fn substring_concat(s:&str, words:&[&str]) -> Vec<usize> {
    let s_len = s.len();
    let c_len = words.len();
    let w_len = words[0].len();
    let mut circle: VecDeque<&str> = VecDeque::with_capacity(w_len * s_len);
    let mut status: HashMap<&str, (i32, i32)> = HashMap::with_capacity(s_len);
    let mut found: Vec<usize> = Vec::new();

    for  &w in words {
        let entry = status.entry(w).or_insert((0,0));
        entry.1 += 1; 
    }
    // offset
    for o in 0..w_len {
        for i in (o..s_len - w_len).step_by(w_len) {
            let curr = &s[i..i+w_len];
            if let Some(entry) = status.get_mut(curr) {
                if entry.0 < entry.1 {
                    entry.0 +=1;
                    circle.push_back(curr);
                } else {
                    // clear reset and clear
                    while let Some(v) = circle.pop_front() {
                        if let Some(entry) = status.get_mut(v) {
                            entry.0 = 0;
                        }
                        if v == curr {
                            break;
                        }
                    }
                }
            } else {
                circle.clear();
                clear_seen(&mut status);
            }
            if circle.len() == c_len {
                let sidx= o + i + w_len - c_len * w_len ;
                found.push(sidx);
            }
        }
        // reset the status before exploring next offset
        circle.clear();
        clear_seen(&mut status);
    }
    found
}

fn main() {
    assert_eq!(vec![0,9],substring_concat("barfoothefoobarman", &["foo","bar"]));
    assert_eq!(Vec::new() as Vec<usize>,substring_concat("wordgoodgoodgoodbestword", &["word","good","best","word"]));
}
