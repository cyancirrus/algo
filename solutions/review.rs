use std::collections::HashMap;


fn create_prefix(sb:&[u8]) -> Vec<usize> {
    let mut a = vec![0;sb.len()];
    let mut j = 0;
    for i in 1..sb.len() {
        while sb[i] != sb[j] && j > 0 {
            j = a[j-1];
        }
        if sb[i] == sb[j] {
            j += 1;
        }
        a[i] = j;
    }
    a
}

fn first_occurance(haystack:&str, needle:&str) -> isize {
    let nb = needle.as_bytes();
    let hb = haystack.as_bytes();
    let npre = create_prefix(nb);
    let (mut idx, mut jdx) = (0, 0);
    while idx < haystack.len() {
        while jdx > 0 && nb[jdx] != hb[idx] {
            jdx = npre[jdx-1];
        }
        if nb[jdx] == hb[idx] {
            jdx += 1;
        }
        if jdx == nb.len() {
            return (idx + 1 - jdx) as isize;
        }
        idx += 1;
    }
    -1
}

fn main() {
    println!(
        "result {:?}",
        // first_occurance("abcsadbutsad", "sad")
        // first_occurance("sasasad", "sasad")
        first_occurance("sasasad", "sssad")
    );
}
