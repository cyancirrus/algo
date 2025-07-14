// rx <-> xr
// lx <-> lx
//condition 1 "too many x's"
//condition 2 "next elem isn't previous"
//

fn swap_adjacent(start:&str, result:&str) -> bool {
    let n = start.len();
    let start = start.as_bytes();
    let result = result.as_bytes();
    let mut start_index = 0;
    let mut result_index = 0;

    while start_index < n && result_index < n {
        while start_index < n && start[start_index] == b'X' {
            start_index += 1;
        }
        while result_index < n && result[result_index] == b'X' {
            result_index += 1;
        }
        if start_index == n && result_index == n {
            return true;
        }
        if start_index == n || result_index == n {
            return false;
        }
        if start[start_index] != result[result_index] {
            return false;
        }
        if start[start_index] == b'R' && result_index < start_index {
            // cannot transform XXR into RXX
            return false;
        }
        if start[start_index] == b'L' && start_index < result_index {
            // cannot transform LXX into XXL
            return false;
        }
        start_index += 1;
        result_index += 1;
    }
    true
}



fn main() {
    assert_eq!(swap_adjacent("RXXLRXRXL", "XRLXXRRLX"), true);
    assert_eq!(swap_adjacent("XRLXXRRLX", "RXXLRXRXL"), false);
    assert_eq!(swap_adjacent("XX", "XX"), true);
    assert_eq!(swap_adjacent("LXX", "XXL"), false);
}
