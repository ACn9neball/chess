pub fn w_pawn(
    mut board: [[String; 8]; 8],
    row: usize,
    col: usize,
    nrow: usize,
    ncol: usize,
) -> [[String; 8]; 8] {
    let value = board[row][col].clone();
    if nrow > 0 {
        if board[nrow][ncol] == ".".to_string() {
            if row == 6 && (nrow == 5 || nrow == 4) && col == ncol {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            } else if nrow == (row - 1) && col == ncol {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            }
        } else if nrow == 0 {
        }
    }

    return board;
}

pub fn b_pawn(
    mut board: [[String; 8]; 8],
    row: usize,
    col: usize,
    nrow: usize,
    ncol: usize,
) -> [[String; 8]; 8] {
    let value = board[row][col].clone();
    if nrow < 7 {
        if board[nrow][ncol] == ".".to_string() {
            if row == 1 && (nrow == 2 || nrow == 3) && col == ncol {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            } else if nrow == (row + 1) && col == ncol {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            }
        } else if nrow == 7 {
        }
    }

    return board;
}

pub fn rook(
    mut board: [[String; 8]; 8],
    row: usize,
    col: usize,
    nrow: usize,
    ncol: usize,
) -> [[String; 8]; 8] {
    let value = board[row][col].clone();
    if board[nrow][ncol] == ".".to_string() {
        if col == ncol {
            board[nrow][ncol] = value;
            board[row][col] = ".".to_string();
        } else if row == nrow {
            board[nrow][ncol] = value;
            board[row][col] = ".".to_string();
        }
    }

    return board;
}

pub fn bishop(
    mut board: [[String; 8]; 8],
    row: usize,
    col: usize,
    nrow: usize,
    ncol: usize,
) -> [[String; 8]; 8] {
    let value = board[row][col].clone();
    let count_row = nrow.abs_diff(row);
    let count_col = ncol.abs_diff(col);
    if board[nrow][ncol] == ".".to_string() {
        if count_row == count_col {
            board[nrow][ncol] = value;
            board[row][col] = ".".to_string();
        }
    }

    return board;
}

pub fn king(
    mut board: [[String; 8]; 8],
    row: usize,
    col: usize,
    nrow: usize,
    ncol: usize,
) -> [[String; 8]; 8] {
    let value = board[row][col].clone();
    let count_row = nrow.abs_diff(row);
    let count_col = ncol.abs_diff(col);
    if board[nrow][ncol] == ".".to_string() {
        if (count_row == 1 || count_row == 0) && (count_col == 1 || count_col == 0) {
            board[nrow][ncol] = value;
            board[row][col] = ".".to_string();
        }
    }

    return board;
}

pub fn queen(
    mut board: [[String; 8]; 8],
    row: usize,
    col: usize,
    nrow: usize,
    ncol: usize,
) -> [[String; 8]; 8] {
    let value = board[row][col].clone();
    let count_row = nrow.abs_diff(row);
    let count_col = ncol.abs_diff(col);
    if board[nrow][ncol] == ".".to_string() {
        if count_row == count_col {
            board[nrow][ncol] = value;
            board[row][col] = ".".to_string();
        } else if nrow == row {
            board[nrow][ncol] = value;
            board[row][col] = ".".to_string();
        } else if ncol == col {
            board[nrow][ncol] = value;
            board[row][col] = ".".to_string();
        }
    }

    return board;
}

pub fn knight(
    mut board: [[String; 8]; 8],
    row: usize,
    col: usize,
    nrow: usize,
    ncol: usize,
) -> [[String; 8]; 8] {
    let value = board[row][col].clone();
    let count_row = nrow.abs_diff(row);
    let count_col = ncol.abs_diff(col);
    if board[nrow][ncol] == ".".to_string() {
        if (count_row == 2) && (count_col == 1) {
            board[nrow][ncol] = value;
            board[row][col] = ".".to_string();
        } else if (count_col == 2) && (count_row == 1) {
            board[nrow][ncol] = value;
            board[row][col] = ".".to_string();
        }
    }

    return board;
}
