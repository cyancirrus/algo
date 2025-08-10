#![allow(dead_code)]
use std::collections::HashMap;

fn solve_queens_mn(m:u16, n:u16) -> Vec<Vec<(u16, u16)>> {
    let mut sol= vec![];
    if n == 0 || n > 15 || m > n { return sol };
    fn dp(
        m:u16,
        n:u16,
        c:u16,
        r:u16,
        cols:u16,
        back:u16, 
        fwrd:u16,
        mask:u16,
        e:&mut Vec<(u16,u16)>,
        sol:&mut Vec<Vec<(u16,u16)>>,
    ) -> usize {
        if c == m {
            sol.push(e.clone()); 
            return 1;
        } else if r == n {
            return 0;
        }
        let mut total = 0;
        let mut unset = !(cols | back | fwrd) & mask;
        total += dp(
            m,
            n,
            c,
            r + 1,
            cols,
            (back << 1) & mask,
            (fwrd >> 1) & mask,
            mask,
            e,
            sol,
        );
        while unset != 0 {
            let b = unset & (!unset + 1);
            unset ^= b;
            e.push((r, b.ilog2() as u16));
            total += dp(
                m,
                n,
                c + 1,
                r + 1,
                cols | b ,
                ((back | b) << 1) & mask,
                ((fwrd | b) >> 1) & mask,
                mask,
                e,
                sol,
            );
            e.pop();
        }
        total
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
        let mut unset = !(cols | back | fwrd) & mask;
        while unset != 0 {
            let b = unset & (!unset + 1);
            unset ^= b;
            e.push((r, b.ilog2() as u16));
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
    println!("solve queens {:?}", solve_queens_mn(3, 4));
}
