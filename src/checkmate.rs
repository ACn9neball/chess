use crate::{
    logic::{b_pawn, bishop, king, knight, queen, rook, w_pawn},
    possible::{
        bishop_cover, black_pawn_cover, king_cover, knight_cover, queen_cover, rook_cover,
        white_pawn_cover,
    },
    tui::check,
};

pub fn checkmate(board: [[String; 8]; 8], white: bool) -> bool {
    if white {
        for i in 0..8 {
            for j in 0..8 {
                match board[i][j].as_str() {
                    "WP" => {
                        let possibility = white_pawn_cover(board.clone(), i, j);
                        for row in 0..8 {
                            for col in 0..8 {
                                if possibility[row][col] == "M" || possibility[row][col] == "C" {
                                    let (nboard, _) = b_pawn(board.clone(), i, j, row, col);
                                    if !check(nboard, white) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                    "WR" => {
                        let possibility = rook_cover(board.clone(), i, j);
                        for row in 0..8 {
                            for col in 0..8 {
                                if possibility[row][col] == "M" || possibility[row][col] == "C" {
                                    let (nboard, _) = rook(board.clone(), i, j, row, col);
                                    if !check(nboard, white) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                    "WB" => {
                        let possibility = bishop_cover(board.clone(), i, j);
                        for row in 0..8 {
                            for col in 0..8 {
                                if possibility[row][col] == "M" || possibility[row][col] == "C" {
                                    let (nboard, _) = bishop(board.clone(), i, j, row, col);
                                    if !check(nboard, white) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                    "WQ" => {
                        let possibility = queen_cover(board.clone(), i, j);
                        for row in 0..8 {
                            for col in 0..8 {
                                if possibility[row][col] == "M" || possibility[row][col] == "C" {
                                    let (nboard, _) = queen(board.clone(), i, j, row, col);
                                    if !check(nboard, white) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                    "WN" => {
                        let possibility = knight_cover(board.clone(), i, j);
                        for row in 0..8 {
                            for col in 0..8 {
                                if possibility[row][col] == "M" || possibility[row][col] == "C" {
                                    let (nboard, _) = knight(board.clone(), i, j, row, col);
                                    if !check(nboard, white) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                    "WK" => {
                        let possibility = king_cover(board.clone(), i, j);
                        for row in 0..8 {
                            for col in 0..8 {
                                if possibility[row][col] == "M" || possibility[row][col] == "C" {
                                    let (nboard, _) =
                                        king(board.clone(), i, j, row, col, false, true, true);
                                    if !check(nboard, white) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    } else {
        for i in 0..8 {
            for j in 0..8 {
                match board[i][j].as_str() {
                    "BP" => {
                        let possibility = black_pawn_cover(board.clone(), i, j);
                        for row in 0..8 {
                            for col in 0..8 {
                                if possibility[row][col] == "M" || possibility[row][col] == "C" {
                                    let (nboard, _) = w_pawn(board.clone(), i, j, row, col);
                                    if !check(nboard, white) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                    "BR" => {
                        let possibility = rook_cover(board.clone(), i, j);
                        for row in 0..8 {
                            for col in 0..8 {
                                if possibility[row][col] == "M" || possibility[row][col] == "C" {
                                    let (nboard, _) = rook(board.clone(), i, j, row, col);
                                    if !check(nboard, white) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                    "BB" => {
                        let possibility = bishop_cover(board.clone(), i, j);
                        for row in 0..8 {
                            for col in 0..8 {
                                if possibility[row][col] == "M" || possibility[row][col] == "C" {
                                    let (nboard, _) = bishop(board.clone(), i, j, row, col);
                                    if !check(nboard, white) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                    "BQ" => {
                        let possibility = queen_cover(board.clone(), i, j);
                        for row in 0..8 {
                            for col in 0..8 {
                                if possibility[row][col] == "M" || possibility[row][col] == "C" {
                                    let (nboard, _) = queen(board.clone(), i, j, row, col);
                                    if !check(nboard, white) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                    "BN" => {
                        let possibility = knight_cover(board.clone(), i, j);
                        for row in 0..8 {
                            for col in 0..8 {
                                if possibility[row][col] == "M" || possibility[row][col] == "C" {
                                    let (nboard, _) = knight(board.clone(), i, j, row, col);
                                    if !check(nboard, white) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                    "BK" => {
                        let possibility = king_cover(board.clone(), i, j);
                        for row in 0..8 {
                            for col in 0..8 {
                                if possibility[row][col] == "M" || possibility[row][col] == "C" {
                                    let (nboard, _) =
                                        king(board.clone(), i, j, row, col, false, true, true);
                                    if !check(nboard, white) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    return true;
}
