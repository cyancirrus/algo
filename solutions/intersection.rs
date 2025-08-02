use std::collections::VecDeque;


// find the element in which has most in common with a & b 
// for the intersection leetcode instead of equality i would check the pointer for the memory is
// equivalent
fn common_sequence(a:&VecDeque<usize>, b: &VecDeque<usize>) -> usize {
    let i = 0;
    if a.is_empty() || b.is_empty() { return usize::MAX};
    let mut idx_a = a.len() - 1;
    let mut idx_b = b.len() - 1;
    let mut elem = &usize::MAX;
    loop {
        match (a.get(idx_a), b.get(idx_b)) {
            (Some(elem_a), Some(elem_b)) => {
                if elem_a == elem_b {
                    idx_a -= 1;
                    idx_b -= 1;
                    elem = elem_a;
                } else {
                    return *elem;
                }
            },
            (None, Some(_)) => {
                return *elem;
            },
            (Some(_), None) => {
                return *elem;
            },
            (None, None) => {
                return *elem;
            }
        }
    }
}

fn main() {
    let a = VecDeque::from(vec![4,1,8,4,5]);
    let b = VecDeque::from(vec![5,6,1,8,4,5]);
    println!("Intersection a b {:?}", common_sequence(&a, &b));
    println!("hello");
}
