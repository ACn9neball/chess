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
                if nrow == 4 && board[5][ncol] == ".".to_string() {
                    board[nrow][ncol] = value;
                    board[row][col] = ".".to_string();
                } else if nrow == 5 {
                    board[nrow][ncol] = value;
                    board[row][col] = ".".to_string();
                }
            } else if nrow == (row - 1) && col == ncol {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            } else if nrow == 0 {
            }
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
                if nrow == 3 && board[2][ncol] == ".".to_string() {
                    board[nrow][ncol] = value;
                    board[row][col] = ".".to_string();
                } else if nrow == 2 {
                    board[nrow][ncol] = value;
                    board[row][col] = ".".to_string();
                }
            } else if nrow == (row + 1) && col == ncol {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            } else if nrow == 7 {
            }
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
    let count_row = nrow.abs_diff(row);
    let count_col = ncol.abs_diff(col);
    let mut found = false;

    if board[nrow][ncol] == ".".to_string() {
        if col == ncol {
            let row_dir = if nrow > row { 1 as isize } else { -1 as isize };
            let mut current_row = row as isize;
            for _ in 1..count_row {
                current_row += row_dir;

                if board[current_row as usize][ncol] != ".".to_string() {
                    found = true;
                }
            }
            if !found {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            }
        } else if row == nrow {
            let col_dir = if ncol > col { 1 as isize } else { -1 as isize };
            let mut current_col = col as isize;
            for _ in 1..count_col {
                current_col += col_dir;

                if board[nrow][current_col as usize] != ".".to_string() {
                    found = true;
                }
            }
            if !found {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            }
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
    let mut found = false;

    if board[nrow][ncol] == ".".to_string() {
        if count_row == count_col {
            let row_dir = if nrow > row { 1 as isize } else { -1 as isize };
            let col_dir = if ncol > col { 1 as isize } else { -1 as isize };
            let mut current_row = row as isize;
            let mut current_col = col as isize;
            for _ in 1..count_row {
                current_row += row_dir;
                current_col += col_dir;

                if board[current_row as usize][current_col as usize] != ".".to_string() {
                    found = true;
                }
            }

            if !found {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            }
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
    let mut found = false;

    if board[nrow][ncol] == ".".to_string() {
        if count_row == count_col {
            let row_dir = if nrow > row { 1 as isize } else { -1 as isize };
            let col_dir = if ncol > col { 1 as isize } else { -1 as isize };
            let mut current_row = row as isize;
            let mut current_col = col as isize;
            for _ in 1..count_row {
                current_row += row_dir;
                current_col += col_dir;

                if board[current_row as usize][current_col as usize] != ".".to_string() {
                    found = true;
                }
            }

            if !found {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            }
        } else if nrow == row {
            let col_dir = if ncol > col { 1 as isize } else { -1 as isize };
            let mut current_col = col as isize;
            for _ in 1..count_col {
                current_col += col_dir;

                if board[nrow][current_col as usize] != ".".to_string() {
                    found = true;
                }
            }
            if !found {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            }
        } else if ncol == col {
            let row_dir = if nrow > row { 1 as isize } else { -1 as isize };
            let mut current_row = row as isize;
            for _ in 1..count_row {
                current_row += row_dir;

                if board[current_row as usize][ncol] != ".".to_string() {
                    found = true;
                }
            }

            if !found {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            }
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
