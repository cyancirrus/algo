#![allow(dead_code)]
use std::collections::HashMap;

fn solve_queens_mn(m:u16, n:u16) -> Vec<Vec<(u16, u16)>> {
    // solutions for m queens on n x n
    let mut sol= vec![];
    if n == 0 || n > 15 || m > n { return sol };

    fn dp(
        m:u16, n:u16, c:u16, r:u16,
        cols:u16, back:u16, fwrd:u16,
        mask:u16,
        e:&mut Vec<(u16,u16)>,
        sol:&mut Vec<Vec<(u16,u16)>>,
    ) {
        if c == m {
            sol.push(e.clone()); 
            return;
        } else if n + c < m + r {
            return;
        } else if r == n {
            return;
        }
        let mut free = !(cols | back | fwrd) & mask;
        dp( m, n, c, r + 1,
            cols,
            (back << 1) & mask,
            (fwrd >> 1) & mask,
            mask, e, sol,
        );
        while free != 0 {
            let b = free & (!free + 1);
            free ^= b;
            e.push((r, b.trailing_zeros() as u16));
            dp(
                m, n, c + 1, r + 1,
                cols | b , ((back | b) << 1) & mask, ((fwrd | b) >> 1) & mask,
                mask, e, sol,
            );
            e.pop();
        }
    }
    dp( m, n, 0, 0, 0, 0, 0, (1<<n) - 1, &mut vec![], &mut sol);
    sol
}

fn solve_queens(n:usize) -> Vec<Vec<(u16, u16)>> {
    let mut sol= vec![];
    if n == 0 || n > 15 { return sol };
    fn dp(
        n:u16,
        r:u16,
        cols:u16,
        back:u16, 
        fwrd:u16,
        mask:u16,
        e:&mut Vec<(u16,u16)>,
        sol:&mut Vec<Vec<(u16,u16)>>,
    ) -> usize {
        if r == n {
            sol.push(e.clone()); 
            return 1;
        }
        let mut total = 0;
        let mut free = !(cols | back | fwrd) & mask;
        while free != 0 {
            let b = free & (!free + 1);
            free ^= b;
            e.push((r, b.trailing_zeros() as u16));
            total += dp(
                n,
                r + 1,
                cols | b ,
                (back | b) << 1,
                (fwrd | b) >> 1,
                mask,
                e,
                sol,
            );
            e.pop();
        }
        total
    }
    dp( n as u16, 0, 0, 0, 0, (1<<n) - 1, &mut vec![], &mut sol);
    sol
}

fn main() {
    // println!("solve queens {:?}", solve_queens(4));
    println!("solve queens {:?}", solve_queens_mn(3, 5));
}
