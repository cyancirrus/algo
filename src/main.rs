#![allow(dead_code)]
use std::ptr::NonNull;
use std::marker::PhantomData;

#[derive(Debug)]
enum Symbols {
    Number(i32),
    Add,
    Sub,
    Mult,
    Div,
    Lparen,
    RParen,
}

impl Symbols {
    fn link(glyph:u8) -> Option<Symbols> {
        match glyph {
            b'+' => Some(Symbols::Add),
            b'-' => Some(Symbols::Sub),
            b'*' => Some(Symbols::Mult),
            b'/' => Some(Symbols::Div),
            b'(' => Some(Symbols::Lparen),
            b')' => Some(Symbols::RParen),
            b'0'..b'9' => Some(Symbols::Number(( glyph - b'0' ) as i32)),
            _ => None,
        }
    }
    fn lex(stream:&[u8]) -> Vec<Self> {
        let mut words = vec![]; 
        let mut previous:Option<Self> = None;

        for &s in stream  {
            let symbol= Self::link(s);
            match (symbol, previous.take()) {
                (Some(Symbols::Number(c)), Some(Symbols::Number(p))) => {
                    previous = Some(Symbols::Number(c  + p * 10));
                },
                (Some(s), Some(p)) => {
                    words.push(p);
                    previous = Some(s);
                },
                (Some(s), None) => {
                    previous.take();
                    previous = Some(s);
                },
                (None, Some(prev)) => {
                    words.push(prev);
                }
                (None, None) => {}
            }
        }
        if let Some(last) = previous {
            words.push(last);
        }
        words
    }
}

enum Operation {
    Add,
    Sub,
    Div,
    Mult,
}

impl Operation {
    fn eval(&self, lhs:i8, rhs:i8) -> i8 {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Sub => lhs - rhs,
            Operation::Div => lhs / rhs,
            Operation::Mult => lhs * rhs,
        }
    }
}

type Link<T> = Option<NonNull<Node<T>>>;

enum Expr {
    Number(i32),
    Binary {
        op:Operation,
        lhs: Link<Expr>,
        rhs: Link<Expr>,
    }
}

struct Node <T> {
    elem:T,
    left:Link<T>,
    right:Link<T>,
}

struct BinaryTree <T> {
    len: usize,
    head: Link<T>,
    _ghost: PhantomData<T>,
}


impl <T> BinaryTree <T>
where T: PartialOrd
{
    fn new() -> Self {
        Self {
            len:0,
            head:None,
            _ghost:PhantomData,
        }
    }
    fn insert(&mut self, elem:T) {
        let mut left = true;
        unsafe {
            let mut node = self.head;
            while let Some(n) = node {
                if (*n.as_ptr()).elem > elem {
                    node = (*n.as_ptr()).left;
                    left = true;
                } else {
                    node = (*n.as_ptr()).right;
                    left = false;
                }
            }
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(
                Node {
                        elem,
                        left: None,
                        right: None,
                }
            )));
            if let Some(n) = node {
                match left {
                    true => (*n.as_ptr()).left = Some(new),
                    false => (*n.as_ptr()).right = Some(new),
                }
            }
        }
    }
}



fn main() {
    let input = b"12 + 3 * (4 + 56)";
    let tokens = Symbols::lex(input);
    for t in tokens {
        println!("{:?}", t);
    }

}
