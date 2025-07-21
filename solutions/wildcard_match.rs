// want a basic ? and * regex
// ? = single none
// * = anything
//
// knowns all regexes can be compiled into a state machine ie a DAG i think b/c u need extended
// regex for recursion
//
// match should cover the entire thing
// -- assume that like
// fn wm("*a", "alskdfjaskdljfas"); shouldn't match
// here could search from the reverse
//
// hmmm there are two options so might be able to like levenshtein this thing
// left | right dp table,
// options are we could * or we could ?
// so like given can we do this tho with the table would it just contain like the character?


// * abc * 
// hi abc wrld
//
//   * a b c *
// h *  
// i *
// a * a a a
// b * * b b
// c * * c c
// w * * * *
// r * * * *
// l * * * *
// d * * * *

// hmmm maybe if we just consider "*" then we simply need to check of in an order that we see the
// strings
//
// like status = * (anything, watch for character a) if we match char then see if we match full if
// not reset, but if we have like a next index to chceck while parsing that would stink ie
//
// hiababcworld
//  when we hit ab abc, we need to track a when we hit it, or the first letter so need to track
//  - the position in the regex we are trying to match, any potentials
//  - the last valid state (ie say wildcard) consider, so after exhausting the stack and state
//  here state can just be like "regex index" so if we clear the queue to check we can flip back to
//  last regex that was valid


//// simplified version in order to match *'s
//fn wildcard_match(regex:&str, text:&str) -> bool{
//    let regex = regex.as_bytes();
//    let text = text.as_bytes();
//    let rlen = regex.len();
//    let tlen = text.len();
//    //rgx, idx 
//    let mut potentials:Vec<(usize, usize)> = vec![(0,0)];
//    while let Some( (ref mut r, ref mut t)) = potentials.pop() {
//        let pre_r = *r;
//        *r +=1;
//        *t +=1;
//        println!("r {r:}, t {t:}"); 
//        loop {
//            // Handle boundary conditions
//            if *r == rlen && *t == tlen {
//                // fully matched
//                return true;
//            } else if *r >= rlen || *t >= tlen{ 
//                // did not fully match try other options
//                break;
//            } 
//            if regex[*r] == b'*' {
//                // new potential wildcard
//                potentials.push((*r, *t));
//                println!("Potentials {potentials:?}");
//                // no match
//                potentials.push((*r+1, *t));
//            }
//            if regex[pre_r] == b'*'{
//                // push potential matches
//                potentials.push((pre_r, *t));
//            }
//            if regex[*r] == text[*t] {
//                *r +=1;
//                *t +=1;
//                continue;
//            }
//            break;
//        }
//    }
//    println!("here");
//    false
//}

fn wildcard_match(regex:&str, text:&str) -> bool {
    let regex = regex.as_bytes();
    let text = text.as_bytes();
    let rlen = regex.len();
    let tlen = text.len();
    //rgx, idx 
    let mut potentials:Vec<(usize, usize)> = vec![(0,0)];
    while let Some( (ref mut r, ref mut t)) = potentials.pop() {
        *t +=1;
        loop {
            println!("r {r:}, t {t:}"); 
            // Handle boundary conditions
            if *r == rlen && *t == tlen {
                // fully matched
                return true;
            } else if *r >= rlen || *t >= tlen{ 
                // did not fully match try other options
                break;
            }
            if regex[*r] == text[*t] {
                *r +=1;
                *t +=1;
                continue;
            }
            // greedy match this
            if regex[*r] == b'*' {
                potentials.push((*r, *t));
                potentials.push((*r+1, *t));
                *t+=1;
            } else if regex[*r] == b'?' {
                // if required letter
                potentials.push((*r+1, *t));
                *t+=1;
                // if not required
                // continue;
            }
            break;
        }
    }
    false
}


fn main() {
    // assert!(wildcard_match("*abc*", "hi abc world"));
    // assert!(wildcard_match("*", "hi abc world"));
    // assert!(wildcard_match("**", "hi abc world"));
    // assert!(wildcard_match("*abc*", "hi ab abc world"));
    assert!(wildcard_match("*a", "aaaaa"));

}
