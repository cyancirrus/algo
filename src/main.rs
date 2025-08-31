
fn mult_strings_naive(num1:&str, num2:&str) -> String {
    let mut lpos = 1;
    let mut res:usize = 0;
    for &b1 in num1.as_bytes().into_iter().rev() {
        let mut rpos = 1;
        for &b2 in num2.as_bytes().into_iter().rev() {
            res += (b1 - b'0') as usize * (b2 - b'0') as usize * lpos * rpos;
            rpos *= 10;
        }
        lpos *= 10;
    }
    res.to_string()
}

fn mult_strings(num1:&str, num2:&str) -> String {
    let m = num1.len();
    let n = num2.len();
    let mut res = vec![0; n + m];
    let b1 = num1.as_bytes();
    let b2 = num2.as_bytes();
    
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            let mul = (b1[i] - b'0')  * (b2[j] - b'0');
            let mut k = n + m - 2 - i - j;
            res[k] += mul;
            while res[k] > 10 {
                let carry = res[k] / 10;
                res[k] %= 10;
                res[k+1] += carry;
                k += 1;
            }
        }
    }
    while let Some(&v) =  res.last() {
        if v == 0 { res.pop(); }
        else { break; }
    }
    for i in 0..res.len() {
        res[i] += b'0';
    }
    res.reverse();
    String::from_utf8(res).unwrap()
}


fn min_operations(nums:&[u32], k:u32) -> usize {
    let mut count = 0;
    for &n in nums { if n < k { count +=1; } }
    count
}


fn main() {
    println!(
        "result {:?}",
        // mult_strings("12", "32")
        mult_strings("21", "34")
    );
}
