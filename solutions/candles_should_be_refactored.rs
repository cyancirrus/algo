
struct Candles{
    mem:Vec<usize>,
    nxt:Vec<usize>,
}

impl Candles {
    fn new(s:&str) -> Self {
        let s = s.as_bytes();
        let n = s.len();
        let mut mem = vec![0;n+1];
        let mut curr = 0;
        let mut seen_candle = false;
        for i in 0..n {
            mem[i+1] = mem[i];
            match s[i] {
                b'|' => {
                    if seen_candle {
                        mem[i+1] += curr;
                    }
                    seen_candle = true;
                    curr = 0;
                },
                b'*' => { curr += 1; },
                _ => panic!("unexpected token"),
            }
        }
        let mut nxt = vec![0;n+1];
        let mut right = usize::MAX;
        for i in (0..n).rev() {
            if s[i] == b'|' { right = i; }
            nxt[i] = right;
        }
        Self {
            mem,
            nxt,
        }
    }
    fn query(&self, l:usize, u:usize) -> usize {
        if l > u || u + 1 >= self.mem.len() { return 0; }
        let lidx = self.nxt[l];
        if lidx == usize::MAX || lidx > u {
            return 0;
        }
        self.mem[u+1].saturating_sub(self.mem[lidx+1])
    }
}

fn candles(s:&str, start:usize, end:usize) -> usize {
    let sb = s.as_bytes();
    let mut idx = start;
    while sb[idx] != b'|' && idx < end {
        idx += 1;
    }
    idx += 1;
    let mut total = 0;
    let mut curr = 0;
    while idx < end {
        match sb[idx] {
            b'*' => {curr += 1},
            b'|' => {
                total += curr;
                curr = 0;
            }
            _ => { panic!("unexpected token");}
        }
    }
    total
}


fn test(s:&str, start:usize, end:usize) -> usize {
    let c = Candles::new(s);
    c.query(start, end)
}

fn main() {
    let s = "|**|**|";
    println!("tst {:?}", test(s, 1, 4));

}
