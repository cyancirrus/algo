use std::collections::HashSet;


fn longest_substring_no_repeats(s:&str) -> usize {
    let chars:Vec<_> = s.chars().collect();
    let mut i = 0;
    let mut j = 0;
    let mut longest = 0;
    let mut seen = HashSet::new();
    while j < chars.len()  {
        if seen.insert(chars[j]) {
            j+=1;
            longest=longest.max(j-i);
        } else {
            seen.remove(&chars[i]);
            i+=1;
        }
    }
    longest
}


fn main() {
    assert_eq!(3, longest_substring_no_repeats("abcabcbb"));
    assert_eq!(3, longest_substring_no_repeats("pwwkew"));
    assert_eq!(3, longest_substring_no_repeats("abcabcbb"));  // "abc"
    assert_eq!(3, longest_substring_no_repeats("pwwkew"));    // "wke"
    assert_eq!(1, longest_substring_no_repeats("bbbb"));      // "b"
    assert_eq!(0, longest_substring_no_repeats(""));          // ""
    assert_eq!(5, longest_substring_no_repeats("tacow"));     // "tacow"
}
