fn v(section:&[u8; 9]) -> bool {
    let mut available = [0 as u8;9];
    for &num in section.iter() {
        if num == 0 { continue };
        let idx = (num - 1) as usize;
        available[idx] += 1;
        if available[idx] > 1 { return false }
    }
    true
}

fn v_row(row:usize, board:&[[u8; 9];9]) -> bool {
    v(&board[row])
}

fn v_local(quadrant:usize, board:&[[u8;9];9]) -> bool {
    let row_start = (quadrant / 3) * 3;
    let col_start = (quadrant % 3) * 3;
    let mut block = [0; 9];
    let mut idx = 0;
    for i in 0..3 {
        for j in 0..3 {
            block[idx] = board[row_start+i][col_start+j];
            idx += 1;
        }
    }
    v(&block)
}

fn v_column(column:usize, board:&[[u8;9];9]) -> bool {
    let mut col= [0;9];
    for i in 0..9 {
        col[i] = board[i][column];
    }
    v(&col)
}



fn valid_sudoku(board:[[u8;9];9]) -> bool {
    for i in 0..9 {
        if !v_row(i, &board) { return false };
        if !v_column(i, &board) { return false };
        if !v_local(i, &board) { return false };
    }
    true
}


fn main() {
}
