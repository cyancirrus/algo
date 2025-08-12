use std::collections::VecDeque;
use std::fmt;
use std::mem;


#[derive(Clone, Copy, Debug)]
struct IpAddress {
    next:Option<usize>, 
    vals:[u8;4],
}

impl IpAddress {
    fn new() -> Self {
        Self {
            next: Some(0),
            vals: [0, 0, 0, 0],
        }
    }
    fn insert(&mut self, val:u8) -> bool{
        if let Some(pos) = self.next {
            self.vals[pos] = val;
            self.next = match pos {
                0..3 => { Some(pos + 1) },
                _ => { None }
            };
            return true;
        }
        false
    }
    fn is_valid(&self) -> bool { self.next.is_none()
    }
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}.{}.{}.{}",
            self.vals[0], self.vals[1], self.vals[2], self.vals[3]
        )
    }
}

fn careful_step(run:u8, digit:u8) -> u8 {
    let ones = digit - b'0';
    if let Some(n) = (run % 100).checked_mul(10) {
        if let Some(t) = n.checked_add(ones) {
            return t
        }
    }
    (run % 10) * 10 + ones
}

fn restore_ip_addresses(s:&str) -> Vec<IpAddress> {
    let sb = s.as_bytes();
    let mut addresses:VecDeque<Vec<IpAddress>> = VecDeque::from(vec![
        vec![],
        vec![],
        vec![],
        vec![IpAddress::new()],
    ]);
    let mut run:u8 = 0;
    for c in sb.iter() {
        // NOTE: did not find an easy manip
        // running forward one number forward in base 10 and auto truncates due to u8
        // run = run.wrapping_mul(10).wrapping_add(c - b'0');
        run = careful_step(run, *c);
        addresses.pop_front();
        addresses.push_back(vec![]);
        if run / 100 > 0 {
            // println!("-------------");
            for add in addresses[0].clone() {
                let mut new = add;
                if new.insert(run) {
                    addresses[3].push(new);
                }
            }
        }
        if run % 100 / 10 > 0 {
            for add in addresses[1].clone() {
                let mut new = add;
                if new.insert(run % 100) {
                    addresses[3].push(new);
                }
            }
        }
        for add in addresses[2].clone() {
            let mut new = add;
            if new.insert(run % 10) {
                addresses[3].push(new);
            }
        }
    }
    {
        addresses.pop_back().unwrap().into_iter()
        .filter(|addr| addr.is_valid()).collect()
    }
}

fn main() {
    let s = "25525511135";
    // let s = "2551234";
    // let s = "25555";
    println!("Addresses {:?}", restore_ip_addresses(s));
}
