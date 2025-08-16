fn permutation_sequence(n:u8, k:u8) -> String {
    let mut nums:Vec<u8> = (0..n).collect();

    fn heap(k:&mut usize, n:usize, nums:&mut [u8]) {
        if *k == 0 {
            return;
        } else if n == 1 {
            *k -= 1;
            return;
        }
        for i in 0..n {
            heap(k, n-1, nums);
            if n & 1 == 0 {
                nums.swap(i, n-1);
            } else {
                nums.swap(0, n-1); }
        }
    }
    println!("nums {nums:?}");
    // nums.iter().fold(String::new(), |mut acc, &digit| {
    //     acc.push_str(word + b'0');
    //     acc
    // })
    String::new()
}


fn main() {
    permutation_sequence(3, 2);
}
