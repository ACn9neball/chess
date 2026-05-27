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
    if board[nrow][ncol] == ".".to_string() {
        if ncol == col + 1 && nrow == row + 1 {
            board[nrow][ncol] = value;
            board[row][col] = ".".to_string();
        } else if ncol == col + 2 && nrow == row + 2 {
            board[nrow][ncol] = value;
            board[row][col] = ".".to_string();
        }
    }

    return board;
}
