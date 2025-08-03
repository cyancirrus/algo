use std::collections::HashSet;
use std::collections::HashMap;
use std::mem;
use std::fmt::Debug;
use std::ops::Shr;

fn radix_sort_flat(elems: &mut Vec<usize>) 
{
    if elems.is_empty() {
        return;
    }
    const RADIX:usize = 4;
    let n = elems.len();
    let word = 1<<RADIX;
    let mut bit_cursor = RADIX;

    let mut curr:Vec<usize> = vec![0; n];
    let mut cursors:Vec<usize> = vec![0;word];
    let mut positions:Vec<usize> = vec![0;word+1];
    let mut least_zeros = u32::MAX;

    for edx in 0..n {
        least_zeros = least_zeros.min(elems[edx].leading_zeros());
        // want the left bounds not right
        positions[(elems[edx] & (word - 1)) + 1] += 1;
    }
    for pdx in 1..word {
        positions[pdx] += positions[pdx-1];
    }
    for edx in 0..n {
        let radix = elems[edx] & (word - 1);
        curr[ positions[radix] + cursors[radix] ] = elems[edx];
        cursors[radix] += 1;
    }
    mem::swap(&mut curr, elems);
    // elements is nothing then we insert and then swap
    for _ in 0..(usize::BITS - least_zeros) / RADIX as u32 + 1 {
        bit_cursor+=RADIX;
        cursors.fill(0);
        positions.fill(0);
        for pdx in 0..n {
            let radix = ( elems[pdx] >> bit_cursor - RADIX ) & ( word - 1 );
            positions[radix+1] += 1;
        }
        for i in 1..word{
            positions[i] += positions[i-1]
        }
        for edx in 0..n {
            let radix = ( elems[edx] >> bit_cursor - RADIX ) & ( word - 1 );
            curr[ positions[radix] + cursors[radix] ] = elems[edx];
            cursors[radix] += 1;
        }
        mem::swap(&mut curr, elems);
    }
}

fn radix_sort(elems: &[usize]) -> Vec<usize>
{
    if elems.is_empty() {
        return vec![];
    }
    const RADIX:usize = 4;
    let mut bit_cursor = RADIX;
    let mut prev:Vec<Vec<usize>> = vec![vec![];1<<RADIX];
    let mut least_zeros = u32::MAX;
    for &e in elems {
        least_zeros = least_zeros.min(e.leading_zeros());
        prev[
            (e & ((1<<RADIX) - 1)) as usize
        ].push(e);
    }
    let mut curr:Vec<Vec<usize>> = vec![Vec::with_capacity(RADIX + elems.len()/RADIX);1<<RADIX];
    for _ in 0..(usize::BITS - least_zeros) / RADIX as u32 + 1 {
        bit_cursor+=RADIX;
        for digit_place in &prev {
            for e in digit_place {
                curr[
                    ((e >> (bit_cursor - RADIX ))
                    & ((1<< RADIX) - 1)) as usize
                ].push(*e);
            }
        }
        mem::swap(&mut prev, &mut curr);
        for bucket in &mut curr {
            bucket.clear();
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
    // println!(" {:?}", radix_sort(&mut [1,2,1,3,15,4, 20, 13,0, 5]));
    let mut x = vec![1,2,1,3,15,4, 20, 13,0, 5];
    radix_sort_flat(&mut x);
    println!("{x:?}")
}
