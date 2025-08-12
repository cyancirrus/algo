use std::collections::HashMap;
use std::collections::VecDeque;

fn rotate_bisect(mut list:Vec<usize>, n:usize) -> Vec<usize> {
    let l1:Vec<usize> = list[0..n].to_vec();
    let mut l2:Vec<usize> = list[n..].to_vec();
    l2.extend(l1);
    l2
}

fn reverse_lltwo(mut list:Vec<usize>, s:usize, e:usize) -> Vec<usize> {
    let mut stack = Vec::with_capacity(e-s);
    for i in s..e {
        stack.push(list[i]);
    }
    for i in s..e {
        list[i] = stack.pop().unwrap();
    }
    list
}


fn rotate_sketch(mut list:Vec<usize>, n:usize) -> Vec<usize> {
    let mut queue:VecDeque<usize> = VecDeque::with_capacity(n);

    for _ in 0..n {
        if let Some(v) = list.pop() {
            queue.push_front(v);
        }
    }
    let n = list.len();
    for i in 0..n {
        queue.push_back(list[i]);
        if let Some(v) = queue.pop_front() {
            list[i] = v;
        }
    }
    while let Some(v) = queue.pop_front() {
        list.push(v);
    }
    list
}

fn main() {
    println!("rotate_sketch {:?}", rotate_sketch(vec![1,2,3,4,5], 2));
}
