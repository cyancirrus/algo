use std::str::from_utf8;
use std::mem;

fn len_post_math(s:&str, t:usize) -> usize  {
    let mut alpha = [0;26];
    for c in s.as_bytes() {
        alpha[(c - b'a') as usize] += 1;
    }
    for _ in 0.. t {
        let mut beta = [0;26];
        for idx in 0..26 {
            if idx == 25 {
                beta[1] += alpha[idx];
            }
            beta[(idx + 1)%26] += alpha[idx];
        }
        mem::swap(&mut alpha, &mut beta);
    }
    alpha.iter().fold(0, |x, y| x + y)
}


fn partial_loops(n:usize, t:usize) -> usize {
    // Estimation
    // k(n - k/2) = t
    // k*n - k^2 /2 - t = 0
    // k^2/2 - k*n + t = 0
    // root (n ^2 - 4 * t / 2)
    // root (n ^2 - 2 * t )
    // sqrt(n * t)
    let mut k = (n * n - 2 * t).isqrt();
    loop {
        // Summation
        // n + n-1 + n-2 ...
        // k * n - 1 + 2 + ...
        // k*n - k * (k-1) / 2
        let missing = k*(k - 1) / 2;
        let real =  k * n  - missing;
        let error = t - real;
        if real - n + k  < t && t < real + n - k {
            break
        }
        // First order correction
        // f(x + h) = f(x) + h * f'(x);
        // df / dh = f'(x) * h;
        // => h * derivative = error;
        // => h = error/derivative
        k +=  error / (n - k);
    }
    k
}




fn len_post_math(s:&str, t:usize) -> usize  {
    let mut alpha = [0;26];
    let mut beta = [0;26];
    let mut result = 0;
    for c in s.as_bytes() {
        alpha[(c - b'a') as usize] += 1;
        beta[(c - b'a') as usize] += 1;
    }
    let loops = t / 26;
    let remain = t % 26;
    // 26 + 25 + .../
    let partial_loops = partial_loops(26, t);
    for (offset, cnt) in alpha.iter().enumerate() {
        if *cnt == 0 {
            continue;
        }
        result += ( loops + 1 )* cnt;
        if remain  > 25 - offset {
            result += cnt;
        }
        for loop_inc in 0..partial_loops {
            // starts at b so equal to
            if remain + loop_inc >= ( 25 - offset - loop_inc ) % 26 {
                result += (cnt - loop_inc) * (loops-loop_inc);
            }
        }
    }
    result
}

// fn len_post_math(s:&str, t:usize) -> usize  {
//     let mut alpha = [0;26];
//     let mut beta = [0;26];
//     for c in s.as_bytes() {
//         alpha[(c - b'a') as usize] += 1;
//         beta[(c - b'a') as usize] += 1;
//     }
//     let full = t / 26;
//     let remain = t % 26;
//     let mut partial = 0;
//     for (offset, cnt) in alpha.iter().enumerate() {
//         for loop_inc in 1..full {
//             beta[(offset + loop_inc) % 26] += cnt * (full - loop_inc);
//         }
//     }
//     for (offset, cnt) in beta.iter().enumerate() {
//         // 25 is zero indexed alphabet len 
//         if remain > 25 - offset {
//             partial+=cnt; 
//         }
//     }
//     let base = beta.iter().fold(0, |x, y| x + y);
//     base + partial
// }
fn len_post_trans(s:&str, t:usize) -> usize {
    let mut s = s.bytes().collect();
    for _ in 0..t {
        worker(&mut s);
    }
    // println!("ending {:?}", from_utf8(&s));
    s.len()
}

fn worker(s:&mut Vec<u8>) {
    let mut idx = 0;
    let mut n = s.len();
    while idx < n {
        if s[idx] != b'z' {
            s[idx] +=1;
        } else {
            s[idx] = b'a';
            s.insert(idx+1,b'b');
            idx+=1;
            n+=1;
        }
        idx+=1;
    }
}

fn main() {
    assert_eq!(5, len_post_math(&"za", 26));

    assert_eq!(7, len_post_trans(&"abcyy", 2));
    assert_eq!(2, len_post_math(&"z", 1));
    assert_eq!(1, len_post_math(&"a", 25));
    assert_eq!(1, len_post_trans(&"a", 25));
    assert_eq!(4, len_post_math(&"zz", 1));
    assert_eq!(4, len_post_trans(&"zz", 1));
    assert_eq!(7, len_post_math(&"abcyy", 2));
    assert_eq!(5, len_post_math(&"abcyy", 1));
    assert_eq!(5, len_post_math(&"za", 26));
    assert_eq!(2, len_post_math(&"b", 25));
    
    assert_eq!(8, len_post_math(&"zaz", 26));
    assert_eq!(8, len_post_trans(&"zaz", 26));
    assert_eq!(13, len_post_math(&"zaz", 51));
    // // assert_eq!(3, len_post_trans(&"za", 25));
    // assert_eq!(3, len_post_trans(&"z", 26));
    // assert_eq!(2, len_post_trans(&"z", 1));
    // for t in 0..100 {
    //     let input = "zaz";
    //     let a = len_post_math(input, t);
    //     let mut s = input.bytes().collect();
    //     for _ in 0..t {
    //         worker(&mut s);
    //     }
    //     let b = s.len();
    //     assert_eq!(a, b, "Failed at t = {}: math = {}, sim = {}", t, a, b);
    // }
}
