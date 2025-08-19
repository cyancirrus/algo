use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Node {
    idx: usize,
    val: usize,
    prime: usize,
}

impl Ord for Node {
    fn cmp(&self, other:&Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn superugly_numberheap(mut n:usize, primes:Vec<usize>) -> usize {
    let mut res = vec![];
    let mut cands = BinaryHeap::new();
    for p in primes {
        cands.push( Node { idx: 0, val: p, prime: p } );
    }
    while n > 0 {
        let Node {idx, val, prime} = cands.pop().unwrap();
        if val > *res.last().unwrap() {
            res.push(val);
        }
        cands.push( Node { idx: idx + 1, val: res[idx + 1] * prime, prime:prime } );
        n-=1;
    }
    *res.last().unwrap()
}

fn superugly_number(mut n:usize, primes:Vec<usize>) -> usize {
    let mut res = vec![1];
    let mut indices = vec![0;primes.len()];
    let mut cands = primes.clone();
    while res.len() < n {
        let mut updates = vec![];
        let mut next = usize::MAX;
        for p in 0..primes.len(){
            if cands[p] < next {
                updates = vec![p];
                next = cands[p];
            } else if cands[p] == next {
                updates.push(p)
            }
        }
        res.push(next);
        for pdx in 0..primes.len() {
            if next == cands[pdx] {
                indices[pdx] += 1;
                cands[pdx] = primes[pdx] * res[indices[pdx]];
            }
        }
    }
    *res.last().unwrap()
}



// fn superugly_number(mut n:usize, primes:Vec<usize>) -> usize {
//     let mut res = vec![1];
//     let mut indices = vec![0;primes.len()];
//     while res.len() < n {
//         let next = primes.iter()
//             .enumerate()
//             .map(|(p, &prime)| res[p] * prime)
//             .min()
//             .unwrap();
//         res.push(next);
//         for (p, &prime) in primes.iter().enumerate() {
//             if prime * res[indices[p]] == next {
//                 indices[p] += 1;
//             }
//         }
//     }
//     *res.last().unwrap()
// }



fn ugly_numberiidp(mut n:usize) -> Vec<usize> {
    let mut twos = 0;
    let mut threes = 0;
    let mut fives = 0;
    let mut res = vec![1];
    
    while res.len() < n {
        let tw = res[twos] * 2;
        let th = res[threes] * 3;
        let fv = res[fives] * 5;
        let next = tw.min(th.min(fv));
        res.push(next);
        if next == tw { twos += 1;}
        if next == th { threes += 1;}
        if next == fv { fives += 1;}
    }
    res
}

fn ugly_numberii(mut n:usize) -> Vec<usize> {
    let mut order:BinaryHeap<Reverse<usize>> = BinaryHeap::from([Reverse(1)]);
    let mut prev = 0;
    let mut res = vec![];
    while n > 0 {
        let c = order.pop().unwrap();
        order.push( Reverse ( c.0 * 2 ) );
        order.push( Reverse ( c.0 * 3 ) );
        order.push( Reverse ( c.0 * 5 ) );
        if c.0 > prev {
            prev = c.0;
            res.push(c.0);
            n -= 1;
        }
    }
    res
}

fn ugly_number(mut n:usize) -> bool {
    while n % 2 == 0 { n /= 2; }
    while n % 3 == 0 { n /= 3; }
    while n % 5 == 0 { n /= 5; }
    n == 1
}




fn happy_number(mut n:usize) -> bool {
    let mut seen:HashSet<usize> = HashSet::new();
    while seen.insert(n) && n != 1 {
        let mut h = 0;
        while n > 0 {
            let d = n % 10;
            h += d * d;
            n /= 10;
        }
        n = h;
    }
    n == 1
}


// fn happy_number(mut n:usize) -> bool {
//     let mut memo:HashMap<Vec<usize>, bool> = HashMap::new();
//     loop {
//         let mut digits = Vec::new();
//         let mut h = 0;
//         while n > 0 {
//             let d = n % 10;
//             digits.push(d);
//             h += d * d;
//             n /= 10;
//         }
//         digits.sort();
//         println!("digits {digits:?}");
//         if let Some(&r) = memo.get(&digits) {
//             return r;
//         }
//         if h == 1 { return true; }
//         memo.insert(digits, false);
//         n = h;
//     }
// }










// fn reverse_bits(mut n:u32) -> u32 {
//     const B = u32::BITS;
//     for pos in 0..B/2 {
//         let high = n >> (B - pos);
//         let low = (n >> pos) & 1;
//         n -= high << (B - pos) - low << (pos);
//         n += low << (B -pos) + high << pos;
//     }
//     n
// }

fn one_bits_hack(mut x: u8) -> u8 {
    // careful handling
    x = x - ((x >> 1) & 0x55);
    // considering the aabb -> aa + bb and storing
    x = (x & 0x33) + ((x>> 2) & 0x33);
    x + (x >> 4) & 0x0F
}
//1010 1010 

// fn one_bits(mut x:u8) -> u8 {
//     let mut bits = 0;
//     for s in 0..u8::BITS {
//         if x == 0 { return bits; }
//         bits += (x >> s) & 1;
//     }
//     bits
// }

// 110
// 110 & 001 + 1

fn one_bits(mut x:u8) -> u8 {
    let mut bits = 0;
    while x > 0 {
        x &= x - 1;
        bits += 1;
    }
    bits
}

fn reverse_bits(mut x: u8) -> u8 {
    x = (( x >> 1 ) & 0b0101_0101) | ((x & 0b0101_0101) << 1);
    x = (( x >> 2 ) & 0b0011_0011) | ((x & 0b0011_0011) << 2);
    x = ( x >> 4 ) | (x << 4);
    x
}


// fn reverse_bits(mut n:u8) -> u8 {
//     const B:u8 = u8::BITS  as u8 - 1;
//     for pos in 0..B/2 {
//         let high = n >> (B - pos) & 1;
//         let low = (n >> pos) & 1;
//         if high != low {
//             n ^=  (1 << (B - pos)) | (1 << pos) ;
//         }
//     }
//     n
// }



// fn reverse_bits(mut n:u8) -> u8 {
//     const B:u8 = u8::BITS  as u8 - 1;
//     for pos in 0..B/2 {
//         let high = n >> (B - pos) & 1;
//         let low = (n >> pos) & 1;
//         n = n 
//             - (high << (B - pos)) - (low << pos)
//             + (low << (B - pos)) + (high << pos)
//         ;
//     }
//     n
// }


fn main() {
    println!("ugly number {:?}", ugly_numberiidp(15));
    println!("ugly number {:?}", ugly_numberii(15));
    // println!("ugly number {:?}", ugly_number(6));
    // println!("happy_number {:?}", happy_number(12386));
    // println!("happy_number {:?}", happy_number(19));
    // println!("test {:b}", 0x33);
    // println!("one bits {:?}", one_bits(3));
    // println!("test {:x}", 0b1111_0000);
    // // println!("test {:b}", 0x33);
    // let mut x:u8 = 0b1010_0000;
    // x = reverse_bits(x);
    // println!("x {x:b}", );
}
