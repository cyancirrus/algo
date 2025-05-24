
fn main() {
    assert_eq!(&VecDeque::from([5]), easy_plus_one(&mut VecDeque::from(vec![4])));
    assert_eq!(&VecDeque::from(vec![1]), easy_plus_one(&mut VecDeque::from(vec![0])));
    assert_eq!(&VecDeque::from(vec![1,0]), easy_plus_one(&mut VecDeque::from(vec![9])));
}
