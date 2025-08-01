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
    assert_eq!(true, valid_parens("hello (world)"));
    assert_eq!(true, valid_parens("hello (w(o)rld)"));
    assert_eq!(false, valid_parens("hello w(o)rld)"));
    assert_eq!(true, valid_parens("hello world"));
    assert_eq!(true, valid_parens("()hello world"));
    assert_eq!(true, valid_parens("(hello world)"));
    assert_eq!(true, valid_parens("hello world()"));
    assert_eq!(true, valid_parens("[]{}[]"));
    assert_eq!(false, valid_parens("[([)]]"));
    assert_eq!(true, valid_parens("{[()]}"));
}
