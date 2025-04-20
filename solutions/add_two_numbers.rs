use std::collections::LinkedList;

fn add_two_numbers(left:&LinkedList<i32>,right:&LinkedList<i32>) -> LinkedList<i32> {
    let mut result:LinkedList<i32> = LinkedList::new();
    let mut carry:i32 = 0;
    let mut sum:i32;
    let mut l_iter = left.iter();
    let mut r_iter = right.iter();
    loop {
        let l = l_iter.next().copied();
        let r = r_iter.next().copied();
        if l.is_none() && r.is_none() && carry == 0 {
            break;
        }
        sum = l.unwrap_or(0) + r.unwrap_or(0) + carry;
        carry = sum / 10;
        result.push_back(sum % 10);
    }
    result
}

fn main() {
    let l1 = LinkedList::from([2,4,3,]);
    let l2 = LinkedList::from([5,6,4,]);
    println!("add two numbers {:?}", add_two_numbers(&l1, &l2));
}
