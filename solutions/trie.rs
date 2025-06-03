use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    word: Option<String>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            word:None,
        }
    }
    fn insert(&mut self, word:&str) {
        let mut node = self;
        for ch in word.chars() {
            node = node.children.entry(ch)
                .or_insert_with(|| TrieNode::new());
        }
        node.word = Some(word.to_string());
    }
    fn build(words:Vec<&str>) ->Self {
        let mut trie = TrieNode::new();
        for w in words {
            trie.insert(w);
        }
        trie
    }
}

fn find_words<'a>(board: &[Vec<char>], words: &[&'a str]) -> Vec<String> {
    let mut found: HashSet<String> = HashSet::new();
    let trie = TrieNode::build(words.to_vec());
    dfs(board, &trie, &mut found);
    found.into_iter().collect::<Vec<String>>()
}

fn dfs(board:&[Vec<char>], trie:&TrieNode, found:&mut HashSet<String>) {
    let mut used:HashSet<(usize, usize)> = HashSet::new();
    let m = board.len();
    let n = board[0].len();

    for i in 0..m {
        for j in 0..n {
            search(i, j, trie, board,  &mut used, found);
            used.clear();
        }
    }
}


fn search(
    x:usize, y:usize,
    trie:&TrieNode,
    board:&[Vec<char>],
    used:&mut HashSet<(usize, usize)>,
    found:&mut HashSet<String>,
) {
    if let Some(w) = &trie.word {
        found.insert(w.clone());
    }
    let m = board.len();
    let n = board[0].len();
    // this already exists
    if !used.insert((x,y)) || x >= m || y >= n { return };
    if let Some(tri) = trie.children.get(&board[x][y]) {
        let dirs = [(1,0), (0,1), (!0, 0), (0, !0)];
        for (dx, dy) in dirs {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            search(nx, ny, tri, board, used, found);
        }
    }
    used.remove(&(x,y));
}



fn main() {
    let board = vec![
        vec!['o','a','a','n'],
        vec!['e','t','a','e'],
        vec!['i','h','k','r'],
        vec!['i','f','l','v'],
    ];

    let words = ["oath", "pea", "eat", "rain"];
    // let words = ["oath"];
    let found = find_words(&board, &words);
    println!("{:?}", found); // Should print ["oath", "eat"]
}
// fn find_words<'a>(board: &[Vec<char>], words: &[&'a str]) -> Vec<&'a str> {
//     let mut found = Vec::new();

//     for &word in words {
//         if find_word(board, word) {
//             found.push(word);
//         }
//     }

//     found
// }

// fn find_word(board:&[Vec<char>], word:&str) -> bool {
//     let mut used:HashSet<(usize, usize)> = HashSet::new();
//     let word:Vec<char> = word.chars().collect();
//     let m = board.len();
//     let n = board[0].len();

//     for i in 0..m {
//         for j in 0..n {
//             used.clear();
//             if search(i, j, &word, board,  &mut used) {
//                 return true;
//             }
//             used.clear();
//         }
//     }
//     false
// }


// fn search(
//     x:usize, y:usize,
//     word:&[char],
//     board:&[Vec<char>],
//     used:&mut HashSet<(usize, usize)>
// ) -> bool {
//     if word.len() == 0 { return true };
//     // this already exists
//     if !used.insert((x,y)) { return false};
//     let m = board.len();
//     let n = board[0].len();
//     // println!("x,y ({}, {})", x,y);
//     if board[x][y] == word[0] {
//         let dirs = [(1,0), (0,1), (!0, 0), (0, !0)];
//         for (dx, dy) in dirs {
//             let nx = x.wrapping_add(dx);
//             let ny = y.wrapping_add(dy);
//             if nx < m  && ny < n {
//                 if search(nx, ny, &word[1..], board, used) {
//                     return true
//                 }
//             }
//         }
//     }
//     used.remove(&(x,y));
//     false
// }

