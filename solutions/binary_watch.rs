fn next_bincombo(k:u32) -> u32 {
    let smallest = k & (!k + 1);
    let ripple = k + smallest;
    let ones = k ^ ripple;
    let ones = (ones >> 2) / smallest;
    ripple | ones

}

fn binary_watch(n:u8) -> Vec<String> {
    let mut times:Vec<String> = Vec::new();
    for hr in 0..4.min(n) {
        let mn = n-hr;
        if mn > 6 { continue };
        let hours = combinations(hr, 12);
        let mins = combinations(mn, 60);
        for h in &hours {
            for m in &mins {
                times.push(format!("{}:{:2}", h, m));
            }
        }
    }
    times
}

fn combinations(bits:u8, max:u8) -> Vec<u8> {
    let mut valids:Vec<u8> = Vec::new();
    fn backtrack(bits:u8, val:u8, cursor:u8, valids:&mut Vec<u8>, max:u8) {
        if bits == 0 {
            if val < max {
                valids.push(val);
            }
            return;
        }
        for i in cursor..=6 {
            backtrack(bits - 1, val | (1 << i), i + 1, valids, max);
        }

    }
    backtrack(bits, 0, 1, &mut valids, max);
    valids
}


fn binary_watch_naive(n:u32) -> Vec<String> {
    let mut times: Vec<String> = Vec::new();
    for hr in 0u8..11 {
        for mn in 0u8..60 {
            if hr.count_ones() + mn.count_ones() == n  as u32 {
                times.push(format!("{}:{:02}", hr, mn));
            }
        }
    }
    times
}


fn binary_watch_analytical(n:u8) -> Vec<String> {
    // hours are between 1-12
    if n > 10 { return vec![] };
    let mut times: Vec<String> = Vec::new();
    for hr in 1..n {
        let mn = n-hr;
        let min_hr:u32 = (2 as u32).pow(hr as u32 ) - 1;
        let min_mn:u32 = (2 as u32).pow(mn as u32 ) - 1;
        // hr is strickly increasing and all future will be invalid
        if min_hr > 12 { break; }
        // proceed to the next iteration
        if min_mn > 59 { continue; }
        
        let mut hour =  0;
        let mut min = 0;
        // need to implement the thing
        // what does it look like for strictly smallest increase
        // 3 -> 1101 = 12 + 1 = 13
        // 13 -> 1110 = 14
        for i in 0..hr {
            hour += 1<<i;
        }
        for j in 0..mn {
            min += 1<<j;
        }
        let mut bit_h = hr;
        let mut bit_m = hr;
        let mut h = hr;
        let mut m = mn;
        while hour <= 12 {
            while min < 60 {
                times.push(format!("{}:{}", hour, min));
                min ^= m;
                min += 1<<m+1;
                if bit_m - m == 1 {
                    bit_m += 1;
                } else {
                    m -= 1;
                }

            }
            if bit_h - h == 1 {
                bit_h += 1;
            } else {
                h -= 1;
            }
            hour ^= 1<<h;
            hour += 1<<h+1;
        }
    }
    times
}

    // how do i generate all sets of 4
    // start with 1111
    // 1234 
    // 1235 not 1<<4  + 1<<5
    // 1245 not 1<<3 + 1<<4
    // 1345 not 1<<2 + 1<<3
    // 2345 not 1<<1 + 1<<2

    // This part was wrong here in my first solution
    // // // increment
    // // 2346 
    // // 2356
    // // 2456
    // // 3456
    //
    //
    // next increment should be (check the combinations at the top)
    // 1236 not 2346





fn main() {
    binary_watch(1);
    // assert_eq!(&VecDeque::from([5]), easy_plus_one(&mut VecDeque::from(vec![4])));
    // assert_eq!(&VecDeque::from(vec![1]), easy_plus_one(&mut VecDeque::from(vec![0])));
    // assert_eq!(&VecDeque::from(vec![1,0]), easy_plus_one(&mut VecDeque::from(vec![9])));
}
