use std::hash::Hash;
use std::str::from_utf8;

const PRIME:u64= 135_123;
const MOD:u64= 1_003;


fn build_lps(pattern: &[u8]) -> Vec<usize> {
    // longest prefix suffix
    let mut fallback = vec![0..pattern.len()];
    let mut len = 0;
    for idx in 1..pattern.len() {
        while len > 0 && pattern[idx] != pattern[j] {
            len = lps[len-1]
        }
        if pattern[idx] == pattern[len] {
            len += 1;
            fallback[idx] = len;
        }
    }
    fallback
}

fn first_occur(needle:&str, haystack:&str) -> i32 {
    let needle = needle.as_bytes();
    let len = needle.len();
    let haystack = haystack.as_bytes();
    let lps = build_lps(needle);
    let mut i= 0; 
    let mut j= 0; 
    while j < needle.len() {
        if haystack[j] == needle[i] {
            i+=1;
            j+=1;
            if i == len {
                return (j-i) as i32;
            }
        } else if i > 0 {
            i = lps[i-1];
        } else {
            j+=1;
        }
    }
    -1
}


fn rolling_hash(bytes:&[u8]) -> u64{
    // p^n [b1] + p^n-1 [b2] + ... p[bn]
    let mut hashed: u64 = 0;
    for &b in bytes.iter() {
        hashed = hashed.wrapping_add(b as u64) % MOD;
        hashed = (hashed * PRIME) % MOD;
    }
    hashed
}

fn mod_power(base:u64, modulus:u64, mut power:usize) -> u64 {
    let mut result = 1;
    let mut factor = base;
    while power > 0 {
        if power & 1 > 0 {
            result = (result * factor) % modulus;
        }
        factor = (factor * factor) % modulus;
        power >>= 1;
    }    
    result
}


fn first_occurence(needle:&str, haystack:&str) -> i32 {
    let n_len = needle.len();
    let h_len = haystack.len();
    if n_len > h_len { return -1; }
    let needle = needle.as_bytes();
    let haystack = haystack.as_bytes();
    let needed = rolling_hash(needle);
    let mut seed = rolling_hash(&haystack[0..n_len]);
    let removal_prime = mod_power(PRIME,MOD,  n_len);
    for (idx, &byte) in haystack[n_len-1..].iter().enumerate() {
        if idx > 0 {
            seed = (seed + MOD)  - (removal_prime) * (haystack[idx-1] as u64) % MOD;
            seed = (seed + byte as u64) % MOD;
            seed = (seed * PRIME) % MOD;
        }
        if needed == seed {
            return idx as i32;
        }
    }
    -1
}


fn remove_duplicates_from_sorted(nums:&mut [u32]) -> &[u32] {
    if nums.is_empty() { return nums }; 
    let mut sidx = 0;
    let mut idx = 1;
    while idx < nums.len() {
        if nums[idx] != nums[sidx] {
            sidx += 1;
            if idx != sidx {
                nums.swap(idx, sidx);
            }
        }
        idx += 1;
    }
    &nums[0..sidx]
}

fn main() {
    // println!("needle hay {:?}", first_occurence("ba", "fba"));
    // // println!("---------------------");
    // let x1 = rolling_hash("ba".as_bytes());
    // let x2 = rolling_hash("fb".as_bytes());
    // let x3 = rolling_hash("f".as_bytes())*4 % MOD;
    // let x4 = rolling_hash("a".as_bytes()) % MOD;
    // println!("---------------------");
    // println!("first {x2:?}");
    // println!("removing {x3:?}");
    // println!("adding {x4:?}");
    // // println!("should have {}", (x2 + x4 + MOD - x3) % MOD);
    // // println!("should have {}", 4 * (x2 + MOD - x3) % MOD + x4);
    // println!("should have {}", 4 * (x2 + MOD - x3 + b'a' as u64) % MOD );
    // // println!("should have {}", x2 - x3);
    // // println!("---------------------");

    // println!("test {:?}", remove_duplicates_from_sorted(&mut vec![0,0,1,1,1,2,2,3,3,4]));
    // println!("needle hay {:?}", first_occurence("sad", "csadbutsad"));
    // println!("needle hay {:?}", first_occurence("ab", "axaab"));
    // println!("needle hay {:?}", first_occurence("ba", "dfbab"));
    println!("needle hay {:?}", first_occurence("zsad", "afadslkjzsad"));
    // println!("mod_power 3, 3: {:?}", mod_power(3, 12, 2));
    // println!("mod_power 3, 3: {:?}", mod_power(3, 12, 2));
    // println!("what is this {:?}", rolling_hash("a".as_bytes()) );
    // println!("what is this {:?}", rolling_hash("a".as_bytes()) * 4 );
    // println!("what is this {:?}", rolling_hash("aa".as_bytes())  );
    
    // println!("what is this {:?}", rolling_hash("a".as_bytes()) );
    // println!("what is this {:?}", rolling_hash("b".as_bytes()) * 4 );
    

    // let x1 = rolling_hash("a".as_bytes());
    // let x2 = rolling_hash("za".as_bytes());
    // let x3 = rolling_hash("z".as_bytes())*4;

    // println!("debug -------");
    // println!("(x1:{x1:}, x3:{x3:}) :: actual {x2:}, derived {:?}", x1 + x3);

    // println!("what is this {:?}", b'a' + b'b');
}
