use std::array;

pub fn black_pawn_cover(board: [[String; 8]; 8], row: usize, col: usize) -> [[String; 8]; 8] {
    let mut possible: [[String; 8]; 8] = array::from_fn(|_| array::from_fn(|_| String::from(".")));
    for nrow in 0..8 {
        for ncol in 0..8 {
            if nrow < 7 {
                if board[nrow][ncol] == ".".to_string() {
                    if row == 1 && (nrow == 2 || nrow == 3) && col == ncol {
                        if nrow == 3 && board[2][ncol] == ".".to_string() {
                            possible[nrow][ncol] = "M".to_string();
                        } else if nrow == 2 {
                            possible[nrow][ncol] = "M".to_string();
                        }
                    } else if nrow == (row + 1) && col == ncol {
                        possible[nrow][ncol] = "M".to_string();
                    }
                } else if board[nrow][ncol].starts_with("W")
                    && board[nrow][ncol] != "WK".to_string()
                {
                    if nrow == (row + 1) && ncol == (col - 1) {
                        possible[nrow][ncol] = "C".to_string();
                    } else if nrow == (row + 1) && ncol == (col + 1) {
                        possible[nrow][ncol] = "C".to_string();
                    }
                }
            }
        }
    }
    return possible;
}

pub fn white_pawn_cover(board: [[String; 8]; 8], row: usize, col: usize) -> [[String; 8]; 8] {
    let mut possible: [[String; 8]; 8] = array::from_fn(|_| array::from_fn(|_| String::from(".")));
    for nrow in 0..8 {
        for ncol in 0..8 {
            if nrow > 0 {
                if board[nrow][ncol] == ".".to_string() {
                    if row == 6 && (nrow == 5 || nrow == 4) && col == ncol {
                        if nrow == 4 && board[5][ncol] == ".".to_string() {
                            possible[nrow][ncol] = "M".to_string();
                        } else if nrow == 5 {
                            possible[nrow][ncol] = "M".to_string();
                        }
                    } else if nrow == (row - 1) && col == ncol {
                        possible[nrow][ncol] = "M".to_string();
                    }
                } else if board[nrow][ncol].starts_with("B")
                    && board[nrow][ncol] != "BK".to_string()
                {
                    if nrow == (row - 1) && ncol == (col - 1) {
                        possible[nrow][ncol] = "C".to_string();
                    } else if nrow == (row - 1) && ncol == (col + 1) {
                        possible[nrow][ncol] = "C".to_string();
                    }
                }
            }
        }
    }

    return possible;
}

pub fn rook_cover(board: [[String; 8]; 8], row: usize, col: usize) -> [[String; 8]; 8] {
    let mut possible: [[String; 8]; 8] = array::from_fn(|_| array::from_fn(|_| String::from(".")));
    let value = board[row][col].clone();
    for i in 0..8 {
        for j in 0..8 {
            let nrow: usize = i;
            let ncol: usize = j;
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
                            possible[nrow][ncol] = "M".to_string();
                        } else {
                            possible[nrow][ncol] = "C".to_string();
                        }
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
                            possible[nrow][ncol] = "M".to_string();
                        } else {
                            possible[nrow][ncol] = "C".to_string();
                        }
                    }
                }
            }
        }
    }

    return possible;
}
pub fn bishop_cover(board: [[String; 8]; 8], row: usize, col: usize) -> [[String; 8]; 8] {
    let mut possible: [[String; 8]; 8] = array::from_fn(|_| array::from_fn(|_| String::from(".")));
    for i in 0..8 {
        for j in 0..8 {
            let nrow: usize = i;
            let ncol: usize = j;
            let count_row = nrow.abs_diff(row);
            let count_col = ncol.abs_diff(col);
            let mut found = false;
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
                        possible[nrow][ncol] = "M".to_string();
                    } else {
                        possible[nrow][ncol] = "C".to_string();
                    }
                }
            }
        }
    }
    return possible;
}
pub fn queen_cover(board: [[String; 8]; 8], row: usize, col: usize) -> [[String; 8]; 8] {
    let mut possible: [[String; 8]; 8] = array::from_fn(|_| array::from_fn(|_| String::from(".")));
    for i in 0..8 {
        for j in 0..8 {
            let nrow: usize = i;
            let ncol: usize = j;
            let count_row = nrow.abs_diff(row);
            let count_col = ncol.abs_diff(col);
            let mut found = false;
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
                        possible[nrow][ncol] = "M".to_string();
                    } else {
                        possible[nrow][ncol] = "C".to_string();
                    }
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
                        possible[nrow][ncol] = "M".to_string();
                    } else {
                        possible[nrow][ncol] = "C".to_string();
                    }
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
                        possible[nrow][ncol] = "M".to_string();
                    } else {
                        possible[nrow][ncol] = "C".to_string();
                    }
                }
            }
        }
    }
    return possible;
}
pub fn knight_cover(board: [[String; 8]; 8], row: usize, col: usize) -> [[String; 8]; 8] {
    let mut possible: [[String; 8]; 8] = array::from_fn(|_| array::from_fn(|_| String::from(".")));
    for i in 0..8 {
        for j in 0..8 {
            let nrow: usize = i;
            let ncol: usize = j;
            let count_row = nrow.abs_diff(row);
            let count_col = ncol.abs_diff(col);
            if (count_row == 2) && (count_col == 1) {
                if board[nrow][ncol] == ".".to_string() {
                    possible[nrow][ncol] = "M".to_string();
                } else {
                    possible[nrow][ncol] = "C".to_string();
                }
            } else if (count_col == 2) && (count_row == 1) {
                if board[nrow][ncol] == ".".to_string() {
                    possible[nrow][ncol] = "M".to_string();
                } else {
                    possible[nrow][ncol] = "C".to_string();
                }
            }
        }
    }
    return possible;
}
pub fn king_cover(board: [[String; 8]; 8], row: usize, col: usize) -> [[String; 8]; 8] {
    let mut possible: [[String; 8]; 8] = array::from_fn(|_| array::from_fn(|_| String::from(".")));
    for i in 0..8 {
        for j in 0..8 {
            let nrow: usize = i;
            let ncol: usize = j;
            let count_row = nrow.abs_diff(row);
            let count_col = ncol.abs_diff(col);
            if (count_row == 1 || count_row == 0) && (count_col == 1 || count_col == 0) {
                if board[nrow][ncol] == ".".to_string() {
                    possible[nrow][ncol] = "M".to_string();
                } else {
                    possible[nrow][ncol] = "C".to_string();
                }
            }
        }
    }

    return possible;
}
