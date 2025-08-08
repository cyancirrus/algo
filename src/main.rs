#[derive(Debug, Eq, PartialEq)]
enum Tokens {
    Number(i32),
    Add,
    Sub,
    Mult,
    Div,
    Lparen,
    RParen,
}

fn link(glyph:u8) -> Option<Tokens> {
    match glyph {
        b'+' => Some(Tokens::Add),
        b'-' => Some(Tokens::Sub),
        b'*' => Some(Tokens::Mult),
        b'/' => Some(Tokens::Div),
        b'(' => Some(Tokens::Lparen),
        b')' => Some(Tokens::RParen),
        b'0'..=b'9' => Some(Tokens::Number(( glyph - b'0' ) as i32)),
        _ => None,
    }
}
fn lex(stream:&[u8]) -> Vec<Tokens> {
    let mut words = vec![]; 
    let mut previous:Option<Tokens> = None;

    for &s in stream  {
        let symbol= link(s);
        match (symbol, previous.take()) {
            (Some(Tokens::Number(c)), Some(Tokens::Number(p))) => {
                previous = Some(Tokens::Number(c  + p * 10));
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

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Div,
    Mult,
}

fn precidence(op: &Operation) -> u8 {
    match op {
        Operation::Add | Operation::Sub => 1,
        Operation::Mult | Operation::Div => 2,
    }
}

#[derive(Debug)]
enum Expr {
    Number(i32),
    Binary {
        op:Operation,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    }
}

fn match_operator(op:&Tokens) -> Operation {
    match op {
        Tokens::Add => Operation::Add,
        Tokens::Sub => Operation::Sub,
        Tokens::Mult => Operation::Mult,
        Tokens::Div => Operation::Div,
        _ => panic!("unexpected token {op:?}"),
    }
}


fn parse_expr(tokens: &[Tokens], idx:&mut usize, min_prec:u8) -> Expr {
    let mut lhs = match tokens[*idx] {
        Tokens::Number(n) => {
            *idx += 1;
            Expr::Number(n)
        },
        Tokens::Lparen => {
            *idx += 1;
            let expr = parse_expr(tokens, idx, 0);
            debug_assert_eq!(tokens[*idx],Tokens::RParen);
            *idx += 1;
            expr
        },
        _ => panic!("unexpected token"),
    };
    while let Some(op) = tokens.get(*idx) {
        if *op == Tokens::RParen {
            break;
        }
        let o = match_operator(op);
        let prec = precidence(&o);
        if prec < min_prec {
            break;
        }
        *idx += 1;
        let rhs = parse_expr(tokens, idx, prec + 1);
        lhs = Expr::Binary {
            op:o,
            lhs:Box::new(lhs),
            rhs:Box::new(rhs),
        }
    }
    lhs
}

fn eval(expr:&Expr) -> i32 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Binary {op, lhs, rhs} => {
            let l = eval(lhs);
            let r = eval(rhs);
            return identity(op, l, r);
        }
    }
}

fn identity(op:&Operation, lhs:i32, rhs:i32) -> i32 {
    match op {
        Operation::Add => lhs + rhs,
        Operation::Sub => lhs - rhs,
        Operation::Div => lhs / rhs,
        Operation::Mult => lhs * rhs,
    }
}

fn run(input:&str) -> i32 {
    let mut tokens = lex(input.as_bytes());
    let ast = parse_expr(&mut tokens,&mut 0, 0);
    eval(&ast)
}

fn main() {
    let input = b"12 + 3 * (4 + 5) + 4";
    let tokens = lex(input);
    for t in tokens {
        println!("{:?}", t);
    }

    let input = "12 + 3 * (4 + 5) + 4";
    // let input = "12 + 3 * 4 + 5 + 4";
    println!("testing {:?}", run(input));

}
