use std::collections::HashMap;
use std::collections::VecDeque;


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
