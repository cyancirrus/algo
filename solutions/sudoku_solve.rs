struct SudokuState {
    pub rows:[[u8;9];9],
    pub cols:[[u8;9];9],
    pub quad:[[u8;9];9],
}

fn parse_str_board(board:&[[&str; 9];9]) -> Result<[[u8;9];9], String> {
    let mut parsed = [[0u8; 9];9];
    
    for i in 0..9 {
        for j in 0..9 {
            match board[i][j] {
                "." => parsed[i][j] = 0,
                s => {
                    if let Ok(n) = s.parse::<u8>() {
                        if n >= 1 && n <= 9 {
                            parsed[i][j] = n;
                        } else {
                            return Err(format!("Invalid number at ({}, {}): {}", i, j, s));
                        }
                    } else {
                        return Err(format!("Invalid character at ({}, {}): {}", i, j, s));
                    }
                }
            }
        }
    }

    Ok(parsed)
}


fn parse_board(board:&[[u8;9];9]) -> SudokuState {
    let mut rows = [[0;9];9];
    let mut cols = [[0;9];9];
    let mut quad = [[0;9];9];
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] != 0 {
                let idx = board[i][j] as usize - 1;
                rows[i][idx] = 1; 
                cols[j][idx] = 1;
                quad[(i/3)*3 + j/3][idx] = 1;
            }
        }
    }
    SudokuState{rows, cols, quad}
}

fn solve_sudoku(board:&mut [[u8;9];9]) -> [[u8;9];9] {
    let mut state = parse_board(&board); 
    let res = _solve_sudoku_(0, &mut state, board);
    println!("Res {:?}", res);
    *board
}

// cleaner version
fn _solve_sudoku_(pos:usize, state:&mut SudokuState, board:&mut [[u8;9];9]) -> bool {
    if pos == 81 { return true };
    let i = pos / 9;
    let j = pos % 9;
    let val = board[i][j] as usize;
    if val != 0 { return _solve_sudoku_(pos+1, state,board) }
    for k in 0..9 {
         {
            // ensure number is available
            if {
                state.rows[i][k] == 0
                 && state.cols[j][k] == 0
                 && state.quad[(i/3)*3 + j/3][k] == 0
            }{
                state.rows[i][k] += 1;
                state.cols[j][k] += 1;
                state.quad[(i/3)*3 + j/3][k] += 1;
                board[i][j] = k as u8 + 1;
                // try to solve
                if _solve_sudoku_(pos+1, state, board) {
                    return true
                };
                board[i][j] = 0;
                // undo the previous i,j inference and continue through loop
                state.rows[i][k] = 0;
                state.cols[j][k] = 0;
                state.quad[(i/3)*3 + j/3][k] = 0;
            }
        }
    }
    false
}

// // from scratch
// fn _solve_sudoku_(pos:usize, state:&mut SudokuState, board:&mut [[u8;9];9]) -> bool {
//     if pos == 81 { return true };
//     println!("pos {:?}", pos);
//     let r = pos / 9;
//     for i in r..9 {
//         for j in 0..9 {
//             let val = board[i][j] as usize;
//             if val == 0 {
//                 // find a potential value
//                     for k in 0..9 {
//                         if {
//                             state.rows[i][k] == 0
//                              && state.cols[j][k] == 0
//                              && state.quad[(i/3)*3 + j/3][k] == 0
//                         }{
//                             state.rows[i][k] += 1;
//                             state.cols[j][k] += 1;
//                             state.quad[(i/3)*3 + j/3][k] += 1;
//                             board[i][j] = k as u8 + 1;
//                             // try to solve
//                             if _solve_sudoku_(pos+1, state, board) {
//                                 return true;
//                             };
//                             board[i][j] = 0;
//                             // undo the previous i,j inference and continue through loop
//                             state.rows[i][k] = 0;
//                             state.cols[j][k] = 0;
//                             state.quad[(i/3)*3 + j/3][k] = 0;
//                         }
//                         if k==8 {
//                             return false
//                         };
//                     }
//                 }
//             }
//         }
//    true
// }

fn main() {
    let input = [
        ["5","3",".",".","7",".",".",".","."],
        ["6",".",".","1","9","5",".",".","."],
        [".","9","8",".",".",".",".","6","."],
        ["8",".",".",".","6",".",".",".","3"],
        ["4",".",".","8",".","3",".",".","1"],
        ["7",".",".",".","2",".",".",".","6"],
        [".","6",".",".",".",".","2","8","."],
        [".",".",".","4","1","9",".",".","5"],
        [".",".",".",".","8",".",".","7","9"]
    ];

    match parse_str_board(&input) {
        Ok(mut board) => {
            println!("hello wolrd!");
            solve_sudoku(&mut board);
            println!("{:?}", board);
        }
        Err(e) => {
            println!("Parsing failed: {}", e);
        }
    }
}

