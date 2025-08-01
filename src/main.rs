use std::collections::HashMap;
use std::hash::Hash;
use std::str::from_utf8;

#[derive(Debug)]
struct TrieNode <'a, T> {
    children: HashMap<T, TrieNode<'a, T>>,
    word: Option<&'a [T]>,
}

impl <'a, T> TrieNode <'a, T>
where T: Eq + Hash + Copy,
{
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            word: None,
        }
    }
    fn insert(&mut self, word:&'a [T]) {
        let mut node = self;
        for &v in word {
            node = node.children.entry(v).or_insert_with(TrieNode::new)
        }
        node.word = Some(word);
    }
    fn build(words:Vec<&'a [T]>) -> Self {
        let mut trie = Self::new();
        for word in words {
            trie.insert(word);
        }
        trie
    }
}

impl <'a> TrieNode<'a, u8>
{
    fn build_words(words:Vec<&'a str>) -> Self {
        let mut trie = Self::new();
        for word in words {
            trie.insert(word.as_bytes());
        }
        trie
    }
}



fn longest_prefix(strings:Vec<&str>) -> String {
    let mut node = &TrieNode::build_words(strings);
    let mut prefix = Vec::new();
    while node.children.len() == 1 && node.word.is_none() {
        let (&ch, child) = node.children.iter().next().unwrap();
        prefix.push(ch);
        node = child;
    }
    String::from_utf8(prefix).unwrap()
}


fn valid_parens(input:&str) -> bool {
    let mut order = Vec::new();
    let input = input.as_bytes();
    for &c in input {
        match (c, order.last()) {
            (b'(', _) => order.push(b'('),
            (b'[', _) => order.push(b'['),
            (b'{', _) => order.push(b'{'),
            (b')', Some(o)) => { if *o == b'(' { order.pop(); } else {return false }},
            (b']', Some(o)) => { if *o == b'[' { order.pop(); } else {return false }},
            (b'}', Some(o)) => { if *o == b'{' { order.pop(); } else {return false }},
            (b')', _) => { return false; },
            (b']', _) => { return false },
            (b'}', _) => { return false },
            (_, _) => {},
        }
    }
    order.is_empty()
}

// ([)]

fn main() {
    let v:&[char] = &vec!['a', 'b', 'c'];
    let mut a = "hello world";
    println!("longest_prefix {:?}", longest_prefix(vec!["hello", "help"]));
    // assert_eq!(true, valid_parens("hello (world)"));
    // assert_eq!(true, valid_parens("hello (w(o)rld)"));
    // assert_eq!(false, valid_parens("hello w(o)rld)"));
    // assert_eq!(true, valid_parens("hello world"));
    // assert_eq!(true, valid_parens("()hello world"));
    // assert_eq!(true, valid_parens("(hello world)"));
    // assert_eq!(true, valid_parens("hello world()"));
    // assert_eq!(true, valid_parens("[]{}[]"));
    // assert_eq!(false, valid_parens("[([)]]"));
    // assert_eq!(true, valid_parens("{[()]}"));
}
