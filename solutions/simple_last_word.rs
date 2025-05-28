fn last_word_length(s:&str) -> usize {
    let mut c = 0;
    for ch in s.bytes().rev() {
        if ch != b' ' {
            c += 1;
        } else if ch == b' ' && c != 0 {
            return c;
        }
    }
    c
}

fn main() {
    assert_eq!(5, last_word_length("hello world")); 
    assert_eq!(5, last_word_length("hello world  ")); 
    assert_eq!(5, last_word_length("world  ")); 
    assert_eq!(5, last_word_length("world")); 
    assert_eq!(0, last_word_length("")); 
}
