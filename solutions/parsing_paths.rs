use std::mem;
use std::str;

fn path_parsing_final(p:&str) -> String {
    let b = p.as_bytes();
    let n = b.len();
    let abs = n > 0 && b[0] == b'/';

    let mut out = Vec::with_capacity(n);
    if abs { out.push(b'/'); }
    let mut checkpoints:Vec<usize> = Vec::new();

    let mut i = 0;
    while i < n {
        while i < n && b[i] == b'/' { i+= 1; }
        if i >= n { break; }
        
        let start = i;
        while i < n && b[i] != b'/' { i += 1; }
        let seg = &b[start..i];

        match seg {
            b"." => {},
            b".." => {
                if let Some(cp) = checkpoints.pop() {
                    out.truncate(cp);
                } else if !abs {
                    let cp = out.len();
                    if !out.is_empty() && *out.last().unwrap() != b'/' { out.push(b'/'); }
                    out.extend_from_slice(b"..");
                    checkpoints.push(cp);
                }
            },
            _ => {
                let cp = out.len();
                if !out.is_empty() && *out.last().unwrap() != b'/' { out.push(b'/'); }
                out.extend_from_slice(seg);
                checkpoints.push(cp);
            }
        }
    }
    if out.is_empty() {
        return ".".to_string();
    }
    String::from_utf8(out).unwrap()
}

fn path_parsing(p:&str) -> String {
    let mut path:Vec<u8> = Vec::new();
    let mut prev= &b'*';
    let mut start = 0;
    let mut dirs = vec![0];
    let pb = p.as_bytes(); 
    for (idx, c) in pb.iter().enumerate() {
        if *c == b'/'  {
            match str::from_utf8(&pb[start..idx]) {
                Ok("..") => {
                    let tdx = dirs.len().saturating_sub(2);
                    path.truncate(dirs[tdx]);
                },
                Ok("/") =>  if *prev != b'/' { path.extend(&pb[start..=idx]); },
                Ok(_) => {
                    dirs.push(idx);
                    path.extend(&pb[start..=idx])
                },
                _ => {},
            }
            start = idx + 1;
        }
        prev = c;
    }
    match str::from_utf8(&pb[start..]) {
        Ok("/") => if *prev != b'/' { path.push(b'/') },
        Ok("..") => { 
            let tdx = dirs.len().saturating_sub(2);
            path.truncate(dirs[tdx]);
         },
        Ok(_) => { path.extend(&pb[start..]) },
        _ => {},
    }
    String::from_utf8(
       path 
    ).unwrap() 
}

fn path_parsing_collection(p:&str) -> String {
    let mut path:Vec<Vec<u8>> = Vec::new();
    let mut prev= &b'*';
    let mut word:Vec<u8> = vec![];
    let pb = p.as_bytes(); 
    for c in pb {
        word.push(*c);
        if *c == b'/'  {
            match str::from_utf8(&word) {
                Ok("../") => {
                    if path.len() > 0 { path.pop(); } ;
                    word.clear();
                },
                Ok("/") =>  if *prev != b'/' { path.push(mem::take(&mut word)); },
                _ => { path.push(mem::take(&mut word)) },
            }
        }
        prev = c;
    }
    match str::from_utf8(&word) {
        Ok("/") => if *prev != b'/' { path.push(vec![b'/']) },
        Ok("..") => if path.len() > 0 { path.pop(); },
        _ => { path.push(mem::take(&mut word)) },
    }
    // i don't know how to flatten cleanly can u help 
    let test = path.into_iter().fold(vec![], |mut a, mut b| {
        a.extend(b);
        a
    });
    String::from_utf8(
        test
    ).unwrap() 
}

fn main() {
    // println!("path parsing {:?}", path_parsing("/abc/../hello_world"));
    // println!("path parsing {:?}", path_parsing("/abc//hello_world/../a"));
    // println!("path parsing {:?}", path_parsing_final("/abc//hello_world/../a"));
    // println!("path parsing {:?}", path_parsing("abc/.."));
    println!("path parsing {:?}", path_parsing_final("abc/test/../a/.."));

}
