fn backspace_smart(a:&str, b:&str) -> bool {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    let (mut i, mut j) = ((a.len() - 1) as isize, (b.len() -1) as isize);

    while i >= 0 && j >= 0 {
        let mut delechars = 0;
        while i >= 0 {
            if ab[i as usize] == b'#' { delechars += 1; }
            else if delechars > 0 { delechars -= 1; }
            else { break; }
            i -= 1;
        }
        let mut delechars = 0;
        while j >= 0 {
            if bb[j as usize] == b'#' { delechars += 1; }
            else if delechars > 0 { delechars -= 1; }
            else { break; }
            j -= 1;
        }
        let ca = if i >= 0 { ab[i as usize] } else { 0 };
        let cb = if i >= 0 { bb[i as usize] } else { 0 };
        if i >= 0 && j >= 0 {
            if ca != cb  {
                return false;
            }
        } else if i >= 0 || j >= 0 {
            return false;
        }
        i-=1;
        j-=1;
    }
    true
}




fn backspace_compare(a:&str, b:&str) -> bool {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    let mut a: Vec<u8> = vec![];
    let mut b:Vec<u8> = vec![];

    for c in ab {
        match c {
            b'#' => {a.pop();},
            _ => {a.push(*c);},
        };
    }
    for c in bb {
        match c {
            b'#' => {b.pop();},
            _ => {b.push(*c);},
        }
    }
    ab == bb
}


fn newline_compare(a:&str, b:&str) -> bool {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    let mut m = ab.len();
    let mut n = bb.len();
    loop {
        match (m, n) {
            (0, 0) => return true,
            (_, 0) => return ab[m-1] == b'#',
            (0, _) => return bb[n-1] == b'#',
            (_, _) => {
                if ab[m-1] != bb[n-1] {
                    return false;
                } else if ab[m-1] == b'#' {
                    return true;
                }
                m-=1;
                n-=1;
            }
        }
    }
}

fn main() {
    println!("backspace compare {:?}", newline_compare("ab##c", "c#d#"));
    // println!("backspace compare {:?}", newline_compare("", "c#d#"));
}
