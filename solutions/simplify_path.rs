use std::collections::HashSet;
use std::collections::VecDeque;

fn simplify_path(s:&str) -> String {
    let mut stack = vec![];
    for part in s.split("/") {
        match part {
            "." | "" => continue,
            ".." => {stack.pop();},
            _ => stack.push(part),
        }
    }
    format!("/{}", stack.join("/"))
}


fn main() {
    println!("result of simplify {:?}", simplify_path("/home/user/Documents/../Pictures"));
    // println!("result of simplify {:?}", simplify_path("/home/.."));
    // println!("longest palin {:?}", longest_palindrome("babad"));
    // println!("longest palin {:?}", longest_palindrome("cbbc"));
    // println!("longest palin {:?}", longest_palindrome_string("babad"));
}
