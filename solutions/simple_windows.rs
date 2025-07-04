#![allow(dead_code)]
use std::collections::HashMap;

// 1..n friends n+1 -> 1 ring(of n)
fn circle_game(n:usize, k:usize) -> usize {
    if n == 0 { return 0 }
    let mut players=(1..=n).collect::<Vec<usize>>();
    let mut curr = 0;
    let mut len = n;

    while len > 1 {
        curr=(curr + k)%len;
        players.remove(curr);
        len-=1;
    }
    players.pop().unwrap()
}

fn circle_game_josephus(k:usize, n:usize) -> usize {
    // this one feels like trivia
    let mut ans = 0;
    for i in 2..n {
        ans=(ans + k) % i;
    }
    ans+1
}

// player 0 requires ball
// player 1 requires 2 balls same
// player n requires n+1 balls same
// colors unordered unbounded <- no vecs

fn winning_players(n:usize, pick:&Vec<(usize,usize)>) -> Vec<usize> {
    let mut state: HashMap<usize, HashMap<usize, usize>> = HashMap::with_capacity(n);
    let mut winners: Vec<usize> = Vec::with_capacity(n);
    let mut included: Vec<bool> = vec![true;n];


    for (player, colour) in pick {
        if included[*player] {
            let colour_count = state
                .entry(*player)
                .or_insert(HashMap::new())
                .entry(*colour)
                .or_insert(0);
            *colour_count += 1;
            if *colour_count > *player {
                winners.push(*player);
                included[*player] = false;
            }
        }

    }
    winners    
}


fn main() {
    println!("main");
}
