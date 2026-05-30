use std::array;

pub fn black_pawn_cover(board: [[String; 8]; 8], row: usize, col: usize) -> [[String; 8]; 8] {
    let mut possible: [[String; 8]; 8] = array::from_fn(|_| array::from_fn(|_| String::from(".")));
    if row < 7 {
        if board[row + 1][col] == ".".to_string() {
            possible[row + 1][col] = "M".to_string();
        }

        if board[row + 2][col] == ".".to_string() {
            if row == 1 {
                possible[row + 2][col] = "M".to_string();
            }
        }

        if col < 7 {
            if board[row + 1][col + 1] != "." {
                possible[row + 1][col + 1] = "C".to_string();
            } else {
                possible[row + 1][col + 1] = "PC".to_string();
            }
        }
        if col > 0 {
            if board[row + 1][col - 1] != "." {
                possible[row + 1][col - 1] = "C".to_string();
            } else {
                possible[row + 1][col - 1] = "PC".to_string();
            }
        }
    }

    return possible;
}

pub fn white_pawn_cover(board: [[String; 8]; 8], row: usize, col: usize) -> [[String; 8]; 8] {
    let mut possible: [[String; 8]; 8] = array::from_fn(|_| array::from_fn(|_| String::from(".")));
    if row > 0 {
        if board[row - 1][col] == ".".to_string() {
            possible[row - 1][col] = "M".to_string();
        }

        if board[row - 2][col] == ".".to_string() {
            if row == 6 {
                possible[row - 2][col] = "M".to_string();
            }
        }

        if col < 7 {
            if board[row - 1][col + 1] == ".".to_string() {
                possible[row - 1][col + 1] = "C".to_string();
            } else {
                possible[row - 1][col + 1] = "PC".to_string();
            }
        }
        if col > 0 {
            if board[row - 1][col - 1] == ".".to_string() {
                possible[row - 1][col - 1] = "C".to_string();
            } else {
                possible[row - 1][col - 1] = "PC".to_string();
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
