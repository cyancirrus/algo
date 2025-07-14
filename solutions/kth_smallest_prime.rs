use std::collections::BinaryHeap;
use std::cmp::Ordering;
// a = [1, 2, 3, 5]
// q = [30, 15, 10, 6]
// gcd = 30
//  <- take the product minus the number over the thing
// 1 / 3  ==  10 / 30 
//
// a[i] / a[j] = q[j] / q[i]
//
// how can this give an ordering tho?
// can i log?
//
// log(a[i] / a[j]) = log(a[i]) - log(a[j])
// // but this would only work for powers of 2 cleanly
// butttttt this would transform into addition
//
// how many fractions are there? // assuming fraction less than one or else use perm
// 4 C n =  4*3/2 = 6
// (1 / 2, 1 / 3, 1/ 4 , 2/3, 2/ 4, 3/ 4)
//
// 1/5
// 1/3
// 2/5 // has a look like fib 1 1 2 3
// 1/2
// 3/5
// 2/3
// could just keep a counter and explore and a rule
//
// no this sucks this isn't going to work hmmm
// fn kth_prime(arr:&[usize], mut k:usize) -> (usize, usize) {
//     let mut seen = HashSet::new();
//     let l = 0;
//     let r = arr.len() - 1;
//     seen.insert((arr[l], arr[r]));
//     k -= 1;
//     while k > 0  {
//         loop {
//             let h1 = (arr[l+1] , arr[r]);
//             let h2 = (arr[l] , arr[r-1]);
             
//             if seen.contains(h1) && seen.contains(h2) {
//                 l+=1; r-=1;
//             }
//         }

//     }
//     (arr[l], arr[r])
// }


// lim a[i] / a[i+1] as i-> inf != 1/2 it should actually grow unless it's twin primes
// twin primes are going to mess a lot of intuition up
//
// a = [1, 2, 3, 5]
// q = [30, 15, 10, 6]
// gcd = 30
// doing this with a mean heap would be so easy;
//
// a /b * 30 = 3/5 * 30 = 90 / 5 = 18
// 3/ 5 = 18, 3 == 10, 5 == 6 (nothing really there) 8 + 10 lol
//
// 2/ 5 = 4/10 * 30 = 12
// 2 = 15, 5 = 6 nothing there
//
// product of primes without a[i] = n-1 * n-2... missing a[i]
// product of primes without a[j] = missing a[j]
// which is why we can get like q[j]/q[i] = a[i]/a[j]
// but can we do anything with it?
//
// 1/5 :: (30, 6) 24 
// 1/3 :: (30, 10) 20
// 2/5 :: (15, 6) 9
// 1/2 :: (30, 15) 15
// 3/5 :: (10, 6) 4 
// 2/3 :: (15, 10) 5
// 3 = 3 * 1's
// 2 = 2 * 1's
// 5 = 5 * 1's
//
//
// (1/2), (1/3), (1/5)
// so i consume (1/5) then i increment it to 2/5 (ie a[i + 1], 5)
// then i compare the set
// nice i'm just going to use a binary heap i think that's it nice i'm just going to use a binary
// heap i think that's it 

#[derive(Eq, PartialEq)]
struct FractionNode {
    idxs:(usize, usize),
    val:usize,
}
impl Ord for FractionNode {
    fn cmp(&self, other:&Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}


impl PartialOrd for FractionNode {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn kth_smallest_prime_factor(arr:&[usize], mut k:usize) -> (usize, usize) {
    let n = arr.len();
    let mut factors = BinaryHeap::with_capacity(n-1);
    let gcd = arr.iter().fold(1, |u, v| u * v);
    
    for i in 1..n {
        factors.push(
            FractionNode {
                val: arr[0] * (gcd / arr[i]),
                idxs: (0, i),
            }
        )
    }

    loop {
        k -= 1;
        if let Some(node) = factors.pop() {
            let (i, j) = node.idxs;
            if k == 0 {
                return (arr[i], arr[j]);
            }
            if i + 1 < j {
                factors.push(
                    FractionNode {
                        idxs: (i+1, j),
                        val: arr[i+1] * (gcd / arr[j]),
                    }
                )
            }
        }
    }
}


fn main() {
}
