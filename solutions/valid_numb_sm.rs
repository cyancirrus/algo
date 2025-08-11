// sign < num < decimal < num < e < num 
// sign needs to be at begin
// nums can be placed anywhere (ignoring sign)
// e needs suffix
// decimal cannot follow e

fn valid_number(s:&str) -> bool {
    // let mut sign = true;
    let mut signed = false;
    let mut decimal = false;
    let mut scientific = false;
    let mut nums = false;

    for c in s.chars() {
        match c {
            '0'..='9' => {
                signed = true;
                nums = true
            },
            '.' => {
                if scientific || decimal { return false ; }
                signed = true;
                decimal = true;
            },
            'e' | 'E' => { 
                if scientific || !nums { return false };
                scientific = true;
                nums = false;
                signed = false;
            },
            '+' | '-' => {
                if signed { return false ; }
                signed = true;
            },
            _ => return false,
        }
    }
    nums
}

fn main() {
    assert_eq!(true, valid_number("2"));
    assert_eq!(true, valid_number("0089"));
    assert_eq!(true, valid_number("-0.1"));
    assert_eq!(true, valid_number("+3.14"));
    assert_eq!(true, valid_number("4."));
    assert_eq!(true, valid_number("-.9"));
    assert_eq!(true, valid_number("2e10"));
    assert_eq!(true, valid_number("-90E3"));
    assert_eq!(true, valid_number("3e+7"));
    assert_eq!(true, valid_number("+6e-1"));
    assert_eq!(true, valid_number("53.5e93"));
    assert_eq!(true, valid_number("-123.456e789"));
    assert_eq!(false, valid_number("abc"));
    assert_eq!(false, valid_number("1a"));
    assert_eq!(false, valid_number("1e"));
    assert_eq!(false, valid_number("e3"));
    assert_eq!(false, valid_number("99e2.5"));
    assert_eq!(false, valid_number("--6"));
    assert_eq!(false, valid_number("-+3"));
    assert_eq!(false, valid_number("95a54e53"));
}
