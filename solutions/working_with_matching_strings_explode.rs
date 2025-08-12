use std::mem;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[inline]
fn build_pows(base: u128, L:usize) -> Vec<u128> {
    let mut p = vec![1u128;L];
    for i in 1..L { p[i] = p[i-1].wrapping_mul(base); }
    p
}

#[inline]
fn word_hash(w: &[u8], base:u128, pow:&[u128]) -> u128 {
    let l = w.len();
    let mut h = 0u128;
    for i in 0..l {
        h = h.wrapping_add((w[i] as u128).wrapping_mul(pow[l - i]));
    }
    h
}

#[inline]
fn pattern_hash_from(h:u128, w:&[u8], pos:usize, base:u128, pow:&[u128], star:u8) -> u128 {
    let l = w.len();
    let exp = pow[l - 1 - pos];
    let delta = (star as u128).wrapping_sub(w[pos] as u128);
    h.wrapping_add(delta.wrapping_mul(exp))
}

#[inline]
fn pattern_key(w: &[u8], pos:usize, pow1:&[u128], pow2:&[u128]) -> (u128, u128) {
    const STAR:u8 = b'*';
    const B1:u128 = 131;
    const B2:u128 = 257;
    let h1 = word_hash(w, B1, pow1);
    let h2 = word_hash(w, B2, pow2);
    (
        pattern_hash_from(h1, w, pos, B1, pow1, STAR),
        pattern_hash_from(h2, w, pos, B2, pow2, STAR)
    )
}



fn ladder_bytes(start:&str, end:&str, words:&[&str]) -> i32 {
    let l = start.len();
    let mut words: Vec<Vec<u8>> = {
        words.iter()
        .filter_map(|&w|
            if l == w.len() {
                Some(w.as_bytes().to_vec())
            } else {
                None
            }
        ).collect()
    };
    if words.is_empty() {
        return -1;
    }
    ladder(start.as_bytes(), end.as_bytes(), &mut words)

}

fn ladder(start:&[u8], end:&[u8], words:&mut Vec<Vec<u8>>) -> i32 {
    words.push(start.to_vec());
    let l = start.len();
    let n = words.len();
    let mut idxmap:HashMap<Vec<u8>, usize> = HashMap::with_capacity(n);
    let mut buckets:HashMap<Vec<u8>, Vec<usize>> = HashMap::with_capacity(l * n);
    let mut contains_target = false;

    for idx in  0..n {
        let w = &mut words[idx];
        contains_target |= w == end;
        idxmap.insert(w.to_vec(), idx);
        for s in 0..l {
            let temp = w[s];
            w[s] = b'*';
            buckets.entry(w.to_vec()).or_default().push(idx);
            w[s] = temp;
        }
    }
    if !contains_target {
        return -1
    }
    let s = idxmap[start];
    let t = idxmap[end];
    let mut dist = vec![i32::MAX;n];
    let mut queue = VecDeque::from([s]);
    dist[s] = 0;
    
    while let Some(u) = queue.pop_front() {
        if u == t {
            return dist[u];
        }
        let w = &mut words[u];
        for s in 0..l {
            let temp = w[s];
            w[s] = b'*';
            if let Some(conn) = buckets.get_mut(w) {
                for &n in conn.iter() {
                    if dist[n] == i32::MAX {
                        dist[n] = dist[u] + 1;
                        queue.push_back(n);
                    }
                }
                conn.clear()
            }
            w[s] = temp
        }
    }
    -1
}



// fn ladder_buckets<'a>(start:&'a String, end:&'a String, words:&'a mut Vec<&'a String>) -> i32 {
//     // explode the input space got help on this one
//     let l = start.len(); 
//     if l == 0 || l != end.len() { return -1; }
//     if !words.iter().any(|&w| w == end) { return -1; }
//     words.push(start);
//     let dict:Vec<&String> = words.iter().copied().filter(|&w| w.len() == start.len()).collect();
//     let idxmap:HashMap<&String, usize> = dict.iter().copied().zip(0..).collect();
//     let l = start.len();
//     let n = dict.len();
//     let s = idxmap[&start];
//     let t = idxmap[&end];

//     let mut buckets: HashMap<Vec<u8>, Vec<usize>> = HashMap::with_capacity(n * l);
//     for (i, &w) in dict.iter().enumerate() {
//         let mut bytes = w.as_bytes().to_vec();
//         for p in 0..bytes.len() {
//             let temp = bytes[p];
//             bytes[p] = b'*';
//             buckets.entry(bytes.clone()).or_default().push(i);
//             bytes[p] = temp;
//         }
//     }

//     let mut dist = vec![i32::MAX;n];
//     let mut q = VecDeque::from([s]);
//     dist[s] = 0;
    
//     while let Some(u) = q.pop_front() {
//         if u == t { return dist[t]; }
//         let mut w = dict[u].as_bytes().to_vec();
//         for p in 0..l {
//             let temp = w[p];
//             w[p] = b'*';
//             if let Some(neighs) = buckets.get_mut(&w) {
//                 for &n in neighs.iter() {
//                     if dist[n] == i32::MAX {
//                         dist[n] = dist[u] + 1;
//                         q.push_back(n);
//                     }
//                 }
//                 neighs.clear();
//             }
//             w[p] = temp;
//         }
//     }
//     -1
// }

fn build_buckets(words: &[String]) -> HashMap<String, Vec<usize>> {
    let mut buckets:HashMap<String, Vec<usize>> = HashMap::new();
    for (i, w) in words.iter().enumerate() {
        let bytes = w.as_bytes();
        for p in 0..bytes.len() {
            let mut pat = bytes.to_vec();
            pat[p] = b'*';
            let key = String::from_utf8(pat).unwrap();
            buckets.entry(key).or_default().push(i);
        }
    }
    buckets
}





fn ladder_length_idx<'a >(start:&'a str, end:&'a str, words: &mut Vec<&'a str>) -> i32 {
    if !words.iter().any(|&w| w==end) { return -1; }
    words.push(start);
    let mut idx: HashMap<&str, usize> = HashMap::new();
    for (i, &w) in words.iter().enumerate() {
        idx.insert(w, i);
    }
    let s = idx[start];
    let t = idx[end];
    let n = words.len();
    let mut g = vec![vec![];n];
    for i in 0..n {
        for j in i+1..n {
            if dist_one(words[i], words[j]) {
                g[i].push(j);
                g[j].push(i);
            }
        }
    }
    let mut dist = vec![i32::MAX; n];
    let mut q = VecDeque::new();
    dist[s] = 0;
    q.push_back(s);
    while let Some(u) = q.pop_front() {
        if u == t { return dist[u]; } 
        let nd = dist[u] + 1;
        for &v in &g[u] {
            if dist[v] == i32::MAX {
                dist[v] = nd;
                q.push_back(v);
            }
        }
    }
    -1
}

fn dist_one(a: &str, b:&str) -> bool {
    let (ab, bb) = (a.as_bytes(), b.as_bytes());
    if !ab.len() == bb.len() { return false; };
    let mut d = 0;
    for i in 0..ab.len() {
        if ab[i] != bb[i] {
            if d > 0 { return false; } 
            d+=1;
        }
    }
    d == 1
}



// // create a graph with edges of distance
// fn ladder_length<'a>(s1:String, s2:String, wordlist:&mut Vec<String>) -> i32 {
//     wordlist.push(s1.clone());
//     wordlist.push(s2.clone());
//     let mut edges = parse_wordlist(wordlist);
//     let mut queue = Vec::from([s1]);
//     let mut next:Vec<String> = Vec::new();
//     let mut visited:HashSet<String> = HashSet::new();
//     let mut d = 0;
//     while !queue.is_empty() {
//         while let Some(f) = queue.pop() {
//             if *f == s2 {
//                 return d;
//             }
//             if let Some(neighs) = edges.remove(&f) {
//                 for n in neighs {
//                     if visited.insert(n.clone()) {
//                         next.push(n);
//                     }
//                 }
//             }
//         }
//         d += 1;
//         mem::swap(&mut queue, &mut next);
//     }
//     -1
// }

// fn parse_wordlist(wordlist:&Vec<String>) -> HashMap<String, Vec<String>> {
//     let mut edges:HashMap<String, Vec<String>> = HashMap::new();
//     let n = wordlist.len();
//     for i in 0..n {
//         for j in i+1..n {
//             let s1 = &wordlist[i];
//             let s2 = &wordlist[j];
//             if dist_one(&s1, &s2) {
//                 edges.entry(s1.to_string()).or_default().push(s2.to_string());
//                 edges.entry(s2.to_string()).or_default().push(s1.to_string());
//             }
//         }
//     }
//     edges
// }


// fn dist_one(s1:&String, s2:&String) -> bool {
//     let s1 = s1.as_bytes();
//     let s2 = s2.as_bytes();
//     let n = s1.len();
//     let mut dist_none = true;
//     for i in 0..n {
//         if s1[i] != s2[i] {
//             if !dist_none {
//                 return false
//             } else {
//                 dist_none = false;
//             }
//         }
//     }
//     !dist_none
// }

// fn distance(s1:&String, s2:&String) -> usize {
//     s1.chars().zip(s2.chars()).
//         map(|(a, b)| if a==b { 0 } else { 1 } )
//         .fold(0, |a, b| a + b)
// }


fn main() {
}
