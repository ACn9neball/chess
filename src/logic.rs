pub fn w_pawn(
    mut board: [[String; 8]; 8],
    row: usize,
    col: usize,
    nrow: usize,
    ncol: usize,
) -> ([[String; 8]; 8], String) {
    let value = board[row][col].clone();
    let pos = position(nrow, ncol);
    let mut piece_move = String::new();
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
            }
            piece_move = String::from(pos);
        } else if board[nrow][ncol].starts_with("B") && board[nrow][ncol] != "BK".to_string() {
            if nrow == (row - 1) && ncol == (col - 1) {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            } else if nrow == (row - 1) && ncol == (col + 1) {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            }
            let capture = format!("{}x{}", capture_pos(col), pos);
            piece_move = String::from(capture);
        }
    }

    return (board, piece_move);
}

pub fn b_pawn(
    mut board: [[String; 8]; 8],
    row: usize,
    col: usize,
    nrow: usize,
    ncol: usize,
) -> ([[String; 8]; 8], String) {
    let value = board[row][col].clone();
    let pos = position(nrow, ncol);
    let mut piece_move = String::new();
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
            }
            piece_move = String::from(pos);
        } else if board[nrow][ncol].starts_with("W") && board[nrow][ncol] != "WK".to_string() {
            if nrow == (row + 1) && ncol == (col - 1) {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            } else if nrow == (row + 1) && ncol == (col + 1) {
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            }
            let capture = format!("{}x{}", capture_pos(col), pos);
            piece_move = String::from(capture);
        }
    }

    return (board, piece_move);
}

pub fn rook(
    mut board: [[String; 8]; 8],
    row: usize,
    col: usize,
    nrow: usize,
    ncol: usize,
) -> ([[String; 8]; 8], String) {
    let value = board[row][col].clone();
    let pos = position(nrow, ncol);
    let mut piece_move = String::new();
    let count_row = nrow.abs_diff(row);
    let count_col = ncol.abs_diff(col);
    let mut found = false;

    if board[nrow][ncol] == ".".to_string()
        || (value.as_str() == "WR"
            && board[nrow][ncol].starts_with("B")
            && board[nrow][ncol].as_str() != "BK")
        || (value.as_str() == "BR"
            && board[nrow][ncol].starts_with("W")
            && board[nrow][ncol].as_str() != "WK")
    {
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
                if board[nrow][ncol] == ".".to_string() {
                    piece_move = String::from(format!("R{}", pos));
                } else {
                    piece_move = String::from(format!("Rx{}", pos));
                }
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
                if board[nrow][ncol] == ".".to_string() {
                    piece_move = String::from(format!("R{}", pos));
                } else {
                    piece_move = String::from(format!("Rx{}", pos));
                }
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            }
        }
    }

    return (board, piece_move);
}

pub fn bishop(
    mut board: [[String; 8]; 8],
    row: usize,
    col: usize,
    nrow: usize,
    ncol: usize,
) -> ([[String; 8]; 8], String) {
    let value = board[row][col].clone();
    let pos = position(nrow, ncol);
    let mut piece_move = String::new();
    let count_row = nrow.abs_diff(row);
    let count_col = ncol.abs_diff(col);
    let mut found = false;

    if board[nrow][ncol] == ".".to_string()
        || (value.as_str() == "WB"
            && board[nrow][ncol].starts_with("B")
            && board[nrow][ncol].as_str() != "BK")
        || (value.as_str() == "BB"
            && board[nrow][ncol].starts_with("W")
            && board[nrow][ncol].as_str() != "WK")
    {
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
                if board[nrow][ncol] == ".".to_string() {
                    piece_move = String::from(format!("B{}", pos));
                } else {
                    piece_move = String::from(format!("Bx{}", pos));
                }
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            }
        }
    }

    return (board, piece_move);
}

pub fn king(
    mut board: [[String; 8]; 8],
    row: usize,
    col: usize,
    nrow: usize,
    ncol: usize,
) -> ([[String; 8]; 8], String) {
    let value = board[row][col].clone();
    let pos = position(nrow, ncol);
    let mut piece_move = String::new();
    let count_row = nrow.abs_diff(row);
    let count_col = ncol.abs_diff(col);
    if board[nrow][ncol] == ".".to_string()
        || (value.as_str() == "WK"
            && board[nrow][ncol].starts_with("B")
            && board[nrow][ncol].as_str() != "BK")
        || (value.as_str() == "BK"
            && board[nrow][ncol].starts_with("W")
            && board[nrow][ncol].as_str() != "WK")
    {
        if (count_row == 1 || count_row == 0) && (count_col == 1 || count_col == 0) {
            if board[nrow][ncol] == ".".to_string() {
                piece_move = String::from(format!("K{}", pos));
            } else {
                piece_move = String::from(format!("Kx{}", pos));
            }
            board[nrow][ncol] = value;
            board[row][col] = ".".to_string();
        }
    }

    return (board, piece_move);
}

pub fn queen(
    mut board: [[String; 8]; 8],
    row: usize,
    col: usize,
    nrow: usize,
    ncol: usize,
) -> ([[String; 8]; 8], String) {
    let value = board[row][col].clone();
    let pos = position(nrow, ncol);
    let mut piece_move = String::new();
    let count_row = nrow.abs_diff(row);
    let count_col = ncol.abs_diff(col);
    let mut found = false;

    if board[nrow][ncol] == ".".to_string()
        || (value.as_str() == "WQ"
            && board[nrow][ncol].starts_with("B")
            && board[nrow][ncol].as_str() != "BK")
        || (value.as_str() == "BQ"
            && board[nrow][ncol].starts_with("W")
            && board[nrow][ncol].as_str() != "WK")
    {
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
                if board[nrow][ncol] == ".".to_string() {
                    piece_move = String::from(format!("Q{}", pos));
                } else {
                    piece_move = String::from(format!("Qx{}", pos));
                }
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
                if board[nrow][ncol] == ".".to_string() {
                    piece_move = String::from(format!("Q{}", pos));
                } else {
                    piece_move = String::from(format!("Qx{}", pos));
                }
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
                if board[nrow][ncol] == ".".to_string() {
                    piece_move = String::from(format!("Q{}", pos));
                } else {
                    piece_move = String::from(format!("Qx{}", pos));
                }
                board[nrow][ncol] = value;
                board[row][col] = ".".to_string();
            }
        }
    }

    return (board, piece_move);
}

pub fn knight(
    mut board: [[String; 8]; 8],
    row: usize,
    col: usize,
    nrow: usize,
    ncol: usize,
) -> ([[String; 8]; 8], String) {
    let value = board[row][col].clone();
    let pos = position(nrow, ncol);
    let mut piece_move = String::new();
    let count_row = nrow.abs_diff(row);
    let count_col = ncol.abs_diff(col);
    if board[nrow][ncol] == ".".to_string()
        || (value.as_str() == "WN"
            && board[nrow][ncol].starts_with("B")
            && board[nrow][ncol].as_str() != "BK")
        || (value.as_str() == "BN"
            && board[nrow][ncol].starts_with("W")
            && board[nrow][ncol].as_str() != "WK")
    {
        if (count_row == 2) && (count_col == 1) {
            if board[nrow][ncol] == ".".to_string() {
                piece_move = String::from(format!("N{}", pos));
            } else {
                piece_move = String::from(format!("Nx{}", pos));
            }
            board[nrow][ncol] = value;
            board[row][col] = ".".to_string();
        } else if (count_col == 2) && (count_row == 1) {
            if board[nrow][ncol] == ".".to_string() {
                piece_move = String::from(format!("n{}", pos));
            } else {
                piece_move = String::from(format!("nx{}", pos));
            }
            board[nrow][ncol] = value;
            board[row][col] = ".".to_string();
        }
    }

    return (board, piece_move);
}

fn position(nrow: usize, ncol: usize) -> String {
    let str_col = match ncol {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => "",
    };
    let str_row = match nrow {
        0 => "8",
        1 => "7",
        2 => "6",
        3 => "5",
        4 => "4",
        5 => "3",
        6 => "2",
        7 => "1",
        _ => "",
    };
    return format!("{}{}", str_col, str_row);
}

fn capture_pos(col: usize) -> String {
    let str_col = match col {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => "",
    };
    return format!("{}", str_col);
}
