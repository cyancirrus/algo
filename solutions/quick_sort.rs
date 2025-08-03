use std::collections::HashSet;
use std::collections::HashMap;
use std::mem;
use std::fmt::Debug;


fn quick_sort<T>(elems:&mut [T])
where T : Eq + PartialOrd,
{
    sorter(0, elems.len()-1, elems);
}

fn sorter<T>(low:usize, high:usize, elems:&mut [T])
where T: Eq + PartialOrd,
{
    if low < high {
        let pivot = partition(low, high, elems);
        if pivot > 0 {
            sorter(low, pivot - 1, elems);
        }
        if pivot < high {
            sorter(pivot+1, high, elems);
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


fn iter_quick_sort<T>(elems:&mut [T])
where T: Eq + PartialOrd,
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


fn main() {
    // println!(" {:?}", quick_sort(&mut [1,2,4,55,6]));
    // println!(" {:?}", quick_sort(&mut [1,2,4,7,6, 31]));
    // println!(" {:?}", quick_sort(&mut [1,2,3,4]));
    // println!(" {:?}", quick_sort(&mut [2,2,1,3,10,4]));
    println!(" {:?}", iter_quick_sort(&mut [1,2,1,3,15,4, 20, 13,0 , 5, -1]));
}
