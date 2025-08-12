use std::str;


fn generate_parentheses(n:usize) -> Vec<String> {
    let mut sols = vec![];
    generate(n, 0, &mut vec![], &mut sols);
    sols.into_iter().map(|bytes| String::from_utf8(bytes).unwrap()).collect()
}

fn generate(open_avail:usize, close_need:usize, word:&mut Vec<u8>,  sols:&mut Vec<Vec<u8>>) {
    if open_avail == 0 && close_need == 0 {
        println!("hello world");
        sols.push(word.clone());
        return
    }
    if open_avail > 0 {
        word.push(b'(');
        generate(open_avail - 1, close_need + 1, word, sols);
        word.pop();
    }
    if close_need > 0 {
        word.push(b')');
        generate(open_avail, close_need - 1, word, sols);
        word.pop();
    }
}


// fn generate_parentheses(n:usize) -> Vec<String> {
//     let mut sols = vec![];
//     generate(n, &mut vec![], &mut vec![], &mut sols);
//     sols.into_iter().map(|bytes| String::from_utf8(bytes).unwrap()).collect()
// }

// fn generate(n:usize, word:&mut Vec<u8>, stack:&mut Vec<u8>, sols:&mut Vec<Vec<u8>>) {
//     println!("n {n:}");
//     if n == 0 && stack.is_empty() {
//         sols.push(word.clone());
//         return
//     }
//     if n > 0 {
//         word.push(b'(');
//         stack.push(b')');
//         generate(n-1, word, stack, sols);
//         stack.pop();
//         word.pop();
//     }
//     if let Some(p) = stack.pop() {
//         word.push(p);
//         generate(n, word, stack, sols);
//         stack.push(word.pop().unwrap());
//     }
// }

fn main() {
    println!("generate parens {:?}", generate_parentheses(3));
}
