use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::mem;
use std::fmt::Debug;
use std::ops::Shr;

fn merge_sort(elems: &mut[usize])
{
    if !elems.is_empty() {
        let mut buffer = vec![0;elems.len()];
        sort(0, elems.len() - 1, elems, &mut buffer);
    }

}

fn sort(start:usize, end:usize, elems:&mut [usize], buffer:&mut [usize]) {
    if end <= start { 
        return;
    } else if end == start + 1 {
        if elems[end] < elems[start] {
            return elems.swap(start, end)
        }
    } else {
        let mid = start + (end - start) / 2;
        sort(start, mid, elems, buffer);
        sort(mid + 1, end, elems, buffer);
        merge(start, mid, end, elems, buffer)
    }
}

fn merge(start:usize, mid:usize, end:usize, elems:&mut[usize], buffer:&mut [usize]) {
    let (mut idx, mut l, mut r) = (0, start, mid+1);
    while l <= mid && r <= end {
        if elems[l] < elems[r] {
            buffer[idx] = elems[l];
            l+=1;
        } else {
            buffer[idx] = elems[r];
            r+=1;
        }
        idx+=1;
    }
    while l <= mid {
        buffer[idx] = elems[l];
        l+=1;
        idx+=1;
    }
    while r <= end {
        buffer[idx] = elems[r];
        r+=1;
        idx+=1;
    }
    elems[start..=end].copy_from_slice(&buffer[..idx]);
}

// fn merge_sort(elems: &mut[usize])
// {
//     if !elems.is_empty() {
//         sort(0, elems.len() - 1, elems);
//     }

// }

// fn sort(start:usize, end:usize, elems:&mut [usize]) {
//     if end <= start { 
//         return;
//     } else if end == start + 1 {
//         if elems[end] < elems[start] {
//             return elems.swap(start, end)
//         }
//     } else {
//         let mid = start + (end - start) / 2;
//         sort(start, mid, elems);
//         sort(mid + 1, end, elems);
//         merge(start, mid, end, elems)
//     }
// }

// fn merge(start:usize, mid:usize, end:usize, elems:&mut[usize]) {
//     let mut merged = Vec::with_capacity(end- start + 1);
    
//     let (mut l, mut r) = (start, mid+1);
//     while l <= mid && r <= end {
//         if elems[l] < elems[r] {
//             merged.push(elems[l]);
//             l+=1;
//         } else {
//             merged.push(elems[r]);
//             r+=1;
//         }
//     }
//     if l <= mid {
//         merged.extend_from_slice(&elems[l..=mid]);
//     } else {
//         merged.extend_from_slice(&elems[r..=end]);
//     }
//     elems[start..=end].copy_from_slice(&merged);
// }

// fn merge_sort(elems: &mut[usize])
// {
//     if !elems.is_empty() {
//         sort(0, elems.len() - 1, elems);
//     }

// }

// fn sort(start:usize, end:usize, elems:&mut [usize]) {
//     if end <= start { 
//         return;
//     } else if end == start + 1 {
//         if elems[end] < elems[start] {
//             return elems.swap(start, end)
//         }
//     } else {
//         let mid = start + (end - start) / 2;
//         sort(start, mid, elems);
//         sort(mid + 1, end, elems);
//         merge(start, mid, end, elems)
//     }
// }

// fn merge(start:usize, mid:usize, end:usize, elems:&mut[usize]) {
//     let mut ridx = mid + 1;
//     let mut lsize = mid-start + 1;

//     for lidx in start..=end {
//         if ridx > end || lsize == 0 {
//             break;
//         }
//         if elems[ridx] < elems[lidx] {
//             for idx in (lidx..ridx).rev() {
//                 elems.swap(idx, idx+1);
//             }
//             ridx += 1;
//         } else {
//             lsize -= 1;
//         }
//     }
// }


fn insertion_sort<T>(elems:&mut Vec<T>)
where T: PartialOrd
{
    for idx in 1..elems.len() {
        let mut sidx = idx;
        while sidx > 0 && elems[sidx] < elems[sidx-1] {
            elems.swap(sidx, sidx-1);
            sidx-=1;
        }
    }
}


// fn insertion_sort<T>(elems:&mut Vec<T>)
// where T: PartialOrd
// {
//     for idx in 1..elems.len() {
//         let mut sidx = 0;
//         while elems[sidx] < elems[idx] && sidx <= idx {
//             sidx+=1;
//         }
//         // doubly linkedlist could remove the reported swap
//         for ridx in (sidx..idx).rev() {
//             elems.swap(ridx+1, ridx) ;
//         }
//     }
// }


fn bubblesort<T>(elems:&mut Vec<T>)
where T: PartialOrd
{
    let mut upper = elems.len();
    let mut changed = true;
    while changed {
        changed = false;
        for idx in 1..upper {
            if elems[idx] < elems[idx-1] {
                elems.swap(idx, idx-1);
                changed = true;
            }
        }
        upper-=1;
    }
}

fn radix_sort_flat(elems: &mut Vec<usize>) 
{
    if elems.is_empty() {
        return;
    }
    const RADIX:usize = 4;
    let n = elems.len();
    let word = 1<<RADIX;
    let mut bit_offset = RADIX;

    let mut curr:Vec<usize> = vec![0; n];
    let mut positions:Vec<usize> = vec![0;word];
    let mut least_zeros = u32::MAX;

    for &e in elems.iter() {
        least_zeros = least_zeros.min(e.leading_zeros());
        positions[e & (word - 1)] += 1;
    }
    for pdx in 1..word {
        positions[pdx] += positions[pdx-1];
    }
    for &e in elems.iter() {
        let radix = e & (word - 1);
        positions[radix] -= 1;
        curr[ positions[radix] ] = e;
    }
    mem::swap(&mut curr, elems);
    for _ in 0..(usize::BITS - least_zeros) / RADIX as u32 + 1 {
        bit_offset+=RADIX;
        positions.fill(0);
        // for pdx in 0..n {
        for &p in elems.iter() {
            let radix = ( p >> bit_offset - RADIX ) & ( word - 1 );
            positions[radix] += 1;
        }
        for i in 1..word{
            positions[i] += positions[i-1]
        }
        for &e in elems.iter().rev() {
            let radix = ( e >> bit_offset - RADIX ) & ( word - 1 );
            positions[radix] -= 1;
            curr[ positions[radix] ] = e;
        }
        mem::swap(&mut curr, elems);
    }
}
// fn radix_sort_flat(elems: &mut Vec<usize>) 
// {
//     if elems.is_empty() {
//         return;
//     }
//     const RADIX:usize = 4;
//     let n = elems.len();
//     let word = 1<<RADIX;
//     let mut bit_offset = RADIX;

//     let mut curr:Vec<usize> = vec![0; n];
//     let mut cursors:Vec<usize> = vec![0;word];
//     let mut positions:Vec<usize> = vec![0;word+1];
//     let mut least_zeros = u32::MAX;

//     for edx in 0..n {
//         least_zeros = least_zeros.min(elems[edx].leading_zeros());
//         // want the left bounds not right
//         positions[(elems[edx] & (word - 1)) + 1] += 1;
//     }
//     for pdx in 1..word {
//         positions[pdx] += positions[pdx-1];
//     }
//     for edx in 0..n {
//         let radix = elems[edx] & (word - 1);
//         curr[ positions[radix] + cursors[radix] ] = elems[edx];
//         cursors[radix] += 1;
//     }
//     mem::swap(&mut curr, elems);
//     // elements is nothing then we insert and then swap
//     for _ in 0..(usize::BITS - least_zeros) / RADIX as u32 + 1 {
//         bit_offset+=RADIX;
//         cursors.fill(0);
//         positions.fill(0);
//         for pdx in 0..n {
//             let radix = ( elems[pdx] >> bit_offset - RADIX ) & ( word - 1 );
//             positions[radix+1] += 1;
//         }
//         for i in 1..word{
//             positions[i] += positions[i-1]
//         }
//         for edx in 0..n {
//             let radix = ( elems[edx] >> bit_offset - RADIX ) & ( word - 1 );
//             curr[ positions[radix] + cursors[radix] ] = elems[edx];
//             cursors[radix] += 1;
//         }
//         mem::swap(&mut curr, elems);
//     }
// }

fn radix_sort(elems: &[usize]) -> Vec<usize>
{
    if elems.is_empty() {
        return vec![];
    }
    const RADIX:usize = 4;
    let mut bit_offset = RADIX;
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
        bit_offset+=RADIX;
        for digit_place in &prev {
            for e in digit_place {
                curr[
                    ((e >> (bit_offset - RADIX ))
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
    // let mut x = vec![1,2,1,3,15,4, 20, 13,0, 5];
    // radix_sort_flat(&mut x);
    // println!("{x:?}")
    // let mut x = vec![1,2,1,3,15,4, 20, 13,0, 5];
    // bubblesort(&mut x);
    // println!("{x:?}")
    let mut x = vec![1,2,1,3,15,4, 20, 13,0, 5];
    insertion_sort(&mut x);
    println!("{x:?}");

    // let mut x = [1, 3, 2,4];
    // merge(0,2, &mut x);
    // println!("what is this {x:?}");
    
    // let mut x = [1, 3, 5, 6, 2, 4, 4, 5];
    // merge(0,4,&mut x);
    // println!("what is this {x:?}");
    
    // let mut x = vec![1,2,1,3,15,4, 20, 13,0, 5, 0, 3];
    let mut x = vec![1,2,1,3,15,4, 10, 13, 11, 2, 5];
    merge_sort(&mut x);
    println!("{x:?}")
}
