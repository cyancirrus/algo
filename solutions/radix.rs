use std::collections::HashSet;
use std::collections::HashMap;
use std::mem;
use std::fmt::Debug;
use std::ops::Shr;


fn radix_sort(elems: &[usize]) -> Vec<usize>
{
    const RADIX:usize = 2;
    let mut cursor = RADIX;
    let mut prev:Vec<Vec<usize>> = vec![vec![];1<<RADIX];
    let mut least_zeros = u32::MAX;
    for &e in elems {
        least_zeros = least_zeros.min(e.leading_zeros());
        prev[
            e & ((1<<RADIX) - 1)
        ].push(e);
    }
    for _ in 0..(usize::BITS - least_zeros) / RADIX as u32 + 1 {
        cursor+=RADIX;
        let mut curr:Vec<Vec<usize>> = vec![vec![];1<<RADIX];
        for digit_place in &prev {
            for e in digit_place {
                curr[
                    (e >> (cursor - RADIX ))
                    & ((1<< RADIX) - 1)
                ].push(*e);
            }
        }
        mem::swap(&mut prev, &mut curr);
        if prev == curr {
            // No more significant digits
            break;
        }
    }
    prev.into_iter().flatten().collect()
}






fn quick_sort<T>(elems:&mut [T])
where T: PartialOrd,
{
    if elems.is_empty() { return };
    let mut stack = Vec::with_capacity(usize::BITS as usize - elems.len().max(1));
    stack.push((0, elems.len()-1));
    while let Some((low, high)) = stack.pop() {
        if low < high {
            let pivot = partition(low, high, elems);
            if pivot > 0 {
                stack.push((low, pivot - 1));
            }
            if pivot < high {
                stack.push((pivot + 1, high));
            }
        }
    }
}

fn partition<T>(low:usize, high:usize, elems:&mut [T]) -> usize
where T: PartialOrd,
{
    let mut pidx= low;

    for i in low..high {
        // elems high is the partition idx
        if elems[i] < elems[high]{
            elems.swap(pidx, i);
            pidx+=1;
        }
    }
    elems.swap(pidx, high);
    pidx
}

fn test() {
    println!("testing");
    let a = 0b10101110;
    println!("first four bytes {:b}", a & ((1 << 4) - 1));
    println!("second four bytes {:b}", (a >> 4) & ((1 << 4) - 1));

}

fn main() {
    // test();

    // println!(" {:?}", quick_sort(&mut [1,2,4,55,6]));
    // println!(" {:?}", quick_sort(&mut [1,2,4,7,6, 31]));
    // println!(" {:?}", quick_sort(&mut [1,2,3,4]));
    // println!(" {:?}", quick_sort(&mut [2,2,1,3,10,4]));
    // println!(" {:?}", quick_sort(&mut [1,2,1,3,15,4, 20, 13,0 , 5, -1]));
    println!(" {:?}", radix_sort(&mut [1,2,1,3,15,4, 20, 13,0, 5]));
}
