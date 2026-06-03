use crate::{
    checkmate::checkmate,
    logic::{b_pawn, bishop, king, knight, queen, rook, w_pawn},
    possible::{
        bishop_cover, black_pawn_cover, king_cover, knight_cover, queen_cover, rook_cover,
        white_pawn_cover,
    },
};
use color_eyre::eyre::Ok;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::array;

struct Data {
    white_turn: bool,
}

pub fn start() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();

    let result = run(&mut terminal);

    ratatui::restore();

    result
}

fn run(terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
    let mut data = Data { white_turn: true };
    let mut board = set_board();
    let mut display = set_display(board.clone());
    let mut color_display = set_color(board.clone());
    let mut highlight: [[bool; 8]; 8] = array::from_fn(|_| array::from_fn(|_| false));
    let mut row = 7;
    let mut col = 4;
    highlight[row][col] = true;
    let mut current_row = 8;
    let mut current_col = 8;
    let mut next_row;
    let mut next_col;
    let mut white_moves: Vec<String> = vec![];
    let mut black_moves: Vec<String> = vec![];
    let mut check_mate: bool;
    let mut c: bool;

    loop {
        terminal.draw(|frame| {
            render(
                frame,
                &highlight,
                &current_row,
                &current_col,
                display.clone(),
                color_display.clone(),
                white_moves.clone(),
                black_moves.clone(),
            )
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Left => {
                        if col > 0 {
                            highlight[row][col] = false;
                            col -= 1;
                            highlight[row][col] = true;
                        }
                    }
                    KeyCode::Right => {
                        if col < 7 {
                            highlight[row][col] = false;
                            col += 1;
                            highlight[row][col] = true;
                        }
                    }
                    KeyCode::Up => {
                        if row > 0 {
                            highlight[row][col] = false;
                            row -= 1;
                            highlight[row][col] = true;
                        }
                    }
                    KeyCode::Down => {
                        if row < 7 {
                            highlight[row][col] = false;
                            row += 1;
                            highlight[row][col] = true;
                        }
                    }
                    KeyCode::Enter => {
                        if current_row == 8 && current_col == 8 {
                            current_row = row;
                            current_col = col;
                        } else {
                            next_row = row;
                            next_col = col;
                            if data.white_turn {
                                match board[current_row][current_col].as_str() {
                                    "WP" => {
                                        let (nboard, mv) = w_pawn(
                                            board.clone(),
                                            current_row,
                                            current_col,
                                            next_row,
                                            next_col,
                                        );

                                        if board != nboard {
                                            if !check(nboard.clone(), true) {
                                                display = set_display(nboard.clone());
                                                c = check(nboard.clone(), false);
                                                board = nboard;
                                                data.white_turn = false;
                                                check_mate = checkmate(board.clone(), false);
                                                if check_mate {
                                                    white_moves.push(format!("{}#", mv));
                                                } else {
                                                    if c {
                                                        white_moves.push(format!("{}+", mv));
                                                    } else {
                                                        white_moves.push(mv);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    "WR" => {
                                        let (nboard, mv) = rook(
                                            board.clone(),
                                            current_row,
                                            current_col,
                                            next_row,
                                            next_col,
                                        );

                                        if board != nboard {
                                            if !check(nboard.clone(), true) {
                                                display = set_display(nboard.clone());
                                                c = check(nboard.clone(), false);
                                                board = nboard;
                                                data.white_turn = false;
                                                check_mate = checkmate(board.clone(), false);
                                                if check_mate {
                                                    white_moves.push(format!("{}#", mv));
                                                } else {
                                                    if c {
                                                        white_moves.push(format!("{}+", mv));
                                                    } else {
                                                        white_moves.push(mv);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    "WB" => {
                                        let (nboard, mv) = bishop(
                                            board.clone(),
                                            current_row,
                                            current_col,
                                            next_row,
                                            next_col,
                                        );

                                        if board != nboard {
                                            if !check(nboard.clone(), true) {
                                                display = set_display(nboard.clone());
                                                c = check(nboard.clone(), false);
                                                board = nboard;
                                                data.white_turn = false;
                                                check_mate = checkmate(board.clone(), false);
                                                if check_mate {
                                                    white_moves.push(format!("{}#", mv));
                                                } else {
                                                    if c {
                                                        white_moves.push(format!("{}+", mv));
                                                    } else {
                                                        white_moves.push(mv);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    "WK" => {
                                        let (nboard, mv) = king(
                                            board.clone(),
                                            current_row,
                                            current_col,
                                            next_row,
                                            next_col,
                                        );

                                        if board != nboard {
                                            if !check(nboard.clone(), true) {
                                                display = set_display(nboard.clone());
                                                c = check(nboard.clone(), false);
                                                board = nboard;
                                                data.white_turn = false;
                                                check_mate = checkmate(board.clone(), false);
                                                if check_mate {
                                                    white_moves.push(format!("{}#", mv));
                                                } else {
                                                    if c {
                                                        white_moves.push(format!("{}+", mv));
                                                    } else {
                                                        white_moves.push(mv);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    "WQ" => {
                                        let (nboard, mv) = queen(
                                            board.clone(),
                                            current_row,
                                            current_col,
                                            next_row,
                                            next_col,
                                        );

                                        if board != nboard {
                                            if !check(nboard.clone(), true) {
                                                display = set_display(nboard.clone());
                                                c = check(nboard.clone(), false);
                                                board = nboard;
                                                data.white_turn = false;
                                                check_mate = checkmate(board.clone(), false);
                                                if check_mate {
                                                    white_moves.push(format!("{}#", mv));
                                                } else {
                                                    if c {
                                                        white_moves.push(format!("{}+", mv));
                                                    } else {
                                                        white_moves.push(mv);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    "WN" => {
                                        let (nboard, mv) = knight(
                                            board.clone(),
                                            current_row,
                                            current_col,
                                            next_row,
                                            next_col,
                                        );

                                        if board != nboard {
                                            if !check(nboard.clone(), true) {
                                                display = set_display(nboard.clone());
                                                c = check(nboard.clone(), false);
                                                board = nboard;
                                                data.white_turn = false;
                                                check_mate = checkmate(board.clone(), false);
                                                if check_mate {
                                                    white_moves.push(format!("{}#", mv));
                                                } else {
                                                    if c {
                                                        white_moves.push(format!("{}+", mv));
                                                    } else {
                                                        white_moves.push(mv);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            } else {
                                match board[current_row][current_col].as_str() {
                                    "BP" => {
                                        let (nboard, mv) = b_pawn(
                                            board.clone(),
                                            current_row,
                                            current_col,
                                            next_row,
                                            next_col,
                                        );

                                        if board != nboard {
                                            if !check(nboard.clone(), false) {
                                                display = set_display(nboard.clone());
                                                c = check(nboard.clone(), true);
                                                board = nboard;
                                                data.white_turn = true;
                                                check_mate = checkmate(board.clone(), true);
                                                if check_mate {
                                                    black_moves.push(format!("{}#", mv));
                                                } else {
                                                    if c {
                                                        black_moves.push(format!("{}+", mv));
                                                    } else {
                                                        black_moves.push(mv);
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    "BR" => {
                                        let (nboard, mv) = rook(
                                            board.clone(),
                                            current_row,
                                            current_col,
                                            next_row,
                                            next_col,
                                        );

                                        if board != nboard {
                                            if !check(nboard.clone(), false) {
                                                display = set_display(nboard.clone());
                                                c = check(nboard.clone(), true);
                                                board = nboard;
                                                data.white_turn = true;
                                                check_mate = checkmate(board.clone(), true);
                                                if check_mate {
                                                    black_moves.push(format!("{}#", mv));
                                                } else {
                                                    if c {
                                                        black_moves.push(format!("{}+", mv));
                                                    } else {
                                                        black_moves.push(mv);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    "BB" => {
                                        let (nboard, mv) = bishop(
                                            board.clone(),
                                            current_row,
                                            current_col,
                                            next_row,
                                            next_col,
                                        );

                                        if board != nboard {
                                            if !check(nboard.clone(), false) {
                                                display = set_display(nboard.clone());
                                                c = check(nboard.clone(), true);
                                                board = nboard;
                                                data.white_turn = true;
                                                check_mate = checkmate(board.clone(), true);
                                                if check_mate {
                                                    black_moves.push(format!("{}#", mv));
                                                } else {
                                                    if c {
                                                        black_moves.push(format!("{}+", mv));
                                                    } else {
                                                        black_moves.push(mv);
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    "BK" => {
                                        let (nboard, mv) = king(
                                            board.clone(),
                                            current_row,
                                            current_col,
                                            next_row,
                                            next_col,
                                        );

                                        if board != nboard {
                                            if !check(nboard.clone(), false) {
                                                display = set_display(nboard.clone());
                                                c = check(nboard.clone(), true);
                                                board = nboard;
                                                data.white_turn = true;
                                                check_mate = checkmate(board.clone(), true);
                                                if check_mate {
                                                    black_moves.push(format!("{}#", mv));
                                                } else {
                                                    if c {
                                                        black_moves.push(format!("{}+", mv));
                                                    } else {
                                                        black_moves.push(mv);
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    "BQ" => {
                                        let (nboard, mv) = queen(
                                            board.clone(),
                                            current_row,
                                            current_col,
                                            next_row,
                                            next_col,
                                        );

                                        if board != nboard {
                                            if !check(nboard.clone(), false) {
                                                display = set_display(nboard.clone());
                                                c = check(nboard.clone(), true);
                                                board = nboard;
                                                data.white_turn = true;
                                                check_mate = checkmate(board.clone(), true);
                                                if check_mate {
                                                    black_moves.push(format!("{}#", mv));
                                                } else {
                                                    if c {
                                                        black_moves.push(format!("{}+", mv));
                                                    } else {
                                                        black_moves.push(mv);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    "BN" => {
                                        let (nboard, mv) = knight(
                                            board.clone(),
                                            current_row,
                                            current_col,
                                            next_row,
                                            next_col,
                                        );

                                        if board != nboard {
                                            if !check(nboard.clone(), false) {
                                                display = set_display(nboard.clone());
                                                c = check(nboard.clone(), true);
                                                board = nboard;
                                                data.white_turn = true;
                                                check_mate = checkmate(board.clone(), true);
                                                if check_mate {
                                                    black_moves.push(format!("{}#", mv));
                                                } else {
                                                    if c {
                                                        black_moves.push(format!("{}+", mv));
                                                    } else {
                                                        black_moves.push(mv);
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    _ => {}
                                }
                            }

                            color_display = set_color(board.clone());

                            current_row = 8;
                            current_col = 8;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn render(
    frame: &mut Frame,
    highlight: &[[bool; 8]; 8],
    prow: &usize,
    pcol: &usize,
    display: [[String; 8]; 8],
    color_display: [[Color; 8]; 8],
    white_moves: Vec<String>,
    black_moves: Vec<String>,
) {
    let screen = frame.area();
    let split = Layout::horizontal([Constraint::Ratio(8, 16), Constraint::Ratio(8, 16)]);
    let [left, right] = split.areas(screen);
    let right_split = Layout::horizontal([
        Constraint::Percentage(5),
        Constraint::Percentage(10),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ]);
    let [col, num, white, black] = right_split.areas(right);
    let split = Layout::vertical([Constraint::Ratio(8, 9), Constraint::Ratio(1, 9)]);
    let [board, bottom] = split.areas(left);

    let mut constraints = vec![];
    for _ in 0..8 {
        constraints.push(Constraint::Fill(1));
    }

    let hor_split = Layout::horizontal(constraints);

    let [one, two, three, four, five, six, seven, eight] = hor_split.areas(board);

    let mut constraints = vec![];
    for _ in 0..8 {
        constraints.push(Constraint::Fill(1));
    }

    let ver_split = Layout::vertical(constraints);
    let [a8, a7, a6, a5, a4, a3, a2, a1] = ver_split.areas(one);
    let [b8, b7, b6, b5, b4, b3, b2, b1] = ver_split.areas(two);
    let [c8, c7, c6, c5, c4, c3, c2, c1] = ver_split.areas(three);
    let [d8, d7, d6, d5, d4, d3, d2, d1] = ver_split.areas(four);
    let [e8, e7, e6, e5, e4, e3, e2, e1] = ver_split.areas(five);
    let [f8, f7, f6, f5, f4, f3, f2, f1] = ver_split.areas(six);
    let [g8, g7, g6, g5, g4, g3, g2, g1] = ver_split.areas(seven);
    let [h8, h7, h6, h5, h4, h3, h2, h1] = ver_split.areas(eight);

    let squares = [
        [a8, b8, c8, d8, e8, f8, g8, h8],
        [a7, b7, c7, d7, e7, f7, g7, h7],
        [a6, b6, c6, d6, e6, f6, g6, h6],
        [a5, b5, c5, d5, e5, f5, g5, h5],
        [a4, b4, c4, d4, e4, f4, g4, h4],
        [a3, b3, c3, d3, e3, f3, g3, h3],
        [a2, b2, c2, d2, e2, f2, g2, h2],
        [a1, b1, c1, d1, e1, f1, g1, h1],
    ];

    for i in 0..8 {
        for j in 0..8 {
            let mut borders = Borders::RIGHT | Borders::BOTTOM;

            if i == 0 {
                borders |= Borders::TOP;
            }
            if j == 0 {
                borders |= Borders::LEFT;
            }
            let block = Block::default()
                .borders(borders)
                .border_type(BorderType::Plain);

            let mut icon = String::new();
            if display[i][j] != "." {
                icon = display[i][j].clone()
            }
            if &i == prow && &j == pcol {
                frame.render_widget(
                    Paragraph::new(icon.clone())
                        .alignment(Alignment::Center)
                        .block(block.fg(Color::Blue))
                        .fg(color_display[i][j].clone()),
                    squares[i][j],
                );
            } else {
                if highlight[i][j] {
                    frame.render_widget(
                        Paragraph::new(icon.clone())
                            .alignment(Alignment::Center)
                            .block(block.fg(Color::Green))
                            .fg(color_display[i][j].clone()),
                        squares[i][j],
                    );
                } else {
                    frame.render_widget(
                        Paragraph::new(icon.clone())
                            .alignment(Alignment::Center)
                            .block(block.fg(Color::Reset))
                            .fg(color_display[i][j].clone()),
                        squares[i][j],
                    );
                }
            }
        }
        let mut numbers = String::new();
        let mut blacks = String::new();
        let mut whites = String::new();
        for i in 0..white_moves.len() {
            numbers.push_str(&format!("{}.\n", i + 1));
            whites.push_str(&format!("{}\n", white_moves[i]));
            if i < black_moves.len() {
                blacks.push_str(&format!("{}\n", black_moves[i]));
            } else {
                blacks.push_str("\n");
            }
        }
        frame.render_widget(Paragraph::new(numbers), num);
        frame.render_widget(Paragraph::new(whites), white);
        frame.render_widget(Paragraph::new(blacks), black);
        let mut constraints = vec![];
        for _ in 0..8 {
            constraints.push(Constraint::Fill(1));
        }

        let bottom_split = Layout::horizontal(constraints);
        let [a, b, c, d, e, f, g, h] = bottom_split.areas(bottom);
        let bottom_squares = [a, b, c, d, e, f, g, h];
        for i in 0..8 {
            let unicode = (97 + i as u8) as char;
            frame.render_widget(
                Paragraph::new(unicode.to_string()).centered(),
                bottom_squares[i],
            );
        }
        let mut constraints = vec![];
        for _ in 0..8 {
            constraints.push(Constraint::Fill(1));
        }
        constraints.push(Constraint::Ratio(1, 9));

        let col_split = Layout::vertical(constraints);
        let [one, two, three, four, five, six, seven, eight, _empty] = col_split.areas(col);
        let col_squares = [one, two, three, four, five, six, seven, eight];
        for i in 0..8 {
            let value = 8 - i;
            let target_square = col_squares[i];

            let vertical_chunks = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Fill(1),
            ])
            .split(target_square);

            let paragraph = Paragraph::new(value.to_string());
            frame.render_widget(paragraph, vertical_chunks[1]);
        }
    }
}

fn set_board() -> [[String; 8]; 8] {
    let mut board: [[String; 8]; 8] = array::from_fn(|_| array::from_fn(|_| String::from(".")));
    board[7][0] = "WR".to_string();
    board[7][1] = "WN".to_string();
    board[7][2] = "WB".to_string();
    board[7][3] = "WQ".to_string();
    board[7][4] = "WK".to_string();
    board[7][5] = "WB".to_string();
    board[7][6] = "WN".to_string();
    board[7][7] = "WR".to_string();
    for row in 0..8 {
        board[6][row] = "WP".to_string();
        board[1][row] = "BP".to_string();
    }
    board[0][0] = "BR".to_string();
    board[0][1] = "BN".to_string();
    board[0][2] = "BB".to_string();
    board[0][3] = "BQ".to_string();
    board[0][4] = "BK".to_string();
    board[0][5] = "BB".to_string();
    board[0][6] = "BN".to_string();
    board[0][7] = "BR".to_string();
    return board;
}

fn set_display(board: [[String; 8]; 8]) -> [[String; 8]; 8] {
    let mut display_board: [[String; 8]; 8] =
        array::from_fn(|_| array::from_fn(|_| String::from(".")));

    let wr = "|_|_|\n ( )\n[___]".to_string();
    let wn = "|\\\n ( \\\n[___]".to_string();
    let wb = " o\n ( )\n[___]".to_string();
    let wq = " www\n ( )\n[___]".to_string();
    let wk = " -+-\n ( )\n[___]".to_string();
    let bp = "[___]\n ( )".to_string();
    let wp = "\n ( )\n[___]".to_string();
    let br = "[___]\n ( )\n|-|-|".to_string();
    let bn = "[___]\n( /\n|/".to_string();
    let bb = "[___]\n ( )\n o".to_string();
    let bq = "[___]\n ( )\nmmm".to_string();
    let bk = "[___]\n ( )\n-+-".to_string();

    for row in 0..8 {
        for col in 0..8 {
            match board[row][col].as_str() {
                "WR" => display_board[row][col] = wr.clone(),
                "WN" => display_board[row][col] = wn.clone(),
                "WB" => display_board[row][col] = wb.clone(),
                "WQ" => display_board[row][col] = wq.clone(),
                "WK" => display_board[row][col] = wk.clone(),
                "WP" => display_board[row][col] = wp.clone(),
                "BR" => display_board[row][col] = br.clone(),
                "BN" => display_board[row][col] = bn.clone(),
                "BB" => display_board[row][col] = bb.clone(),
                "BQ" => display_board[row][col] = bq.clone(),
                "BK" => display_board[row][col] = bk.clone(),
                "BP" => display_board[row][col] = bp.clone(),
                _ => {}
            }
        }
    }

    return display_board;
}

fn set_color(board: [[String; 8]; 8]) -> [[Color; 8]; 8] {
    let mut color_board: [[Color; 8]; 8] = array::from_fn(|_| array::from_fn(|_| Color::Reset));
    let mut value;
    for i in 0..8 {
        for j in 0..8 {
            value = &board[i][j];
            if value.starts_with("W") {
                color_board[i][j] = Color::White;
            } else if value.starts_with("B") {
                color_board[i][j] = Color::Black;
            }
        }
    }

    return color_board;
}

pub fn check(board: [[String; 8]; 8], white: bool) -> bool {
    let mut bcheck = false;
    let mut row = 0;
    let mut col = 0;
    if white {
        for i in 0..8 {
            for j in 0..8 {
                if board[i][j] == "WK".to_string() {
                    row = i;
                    col = j;
                }
            }
        }
        for i in 0..8 {
            for j in 0..8 {
                match board[i][j].as_str() {
                    "BP" => {
                        let possility = black_pawn_cover(board.clone(), i, j);
                        if possility[row][col] == "PC" || possility[row][col] == "C" {
                            bcheck = true;
                        }
                    }
                    "BR" => {
                        let possility = rook_cover(board.clone(), i, j);
                        if possility[row][col] == "M" || possility[row][col] == "C" {
                            bcheck = true;
                        }
                    }
                    "BB" => {
                        let possility = bishop_cover(board.clone(), i, j);
                        if possility[row][col] == "M" || possility[row][col] == "C" {
                            bcheck = true;
                        }
                    }
                    "BQ" => {
                        let possility = queen_cover(board.clone(), i, j);
                        if possility[row][col] == "M" || possility[row][col] == "C" {
                            bcheck = true;
                        }
                    }
                    "BN" => {
                        let possility = knight_cover(board.clone(), i, j);
                        if possility[row][col] == "M" || possility[row][col] == "C" {
                            bcheck = true;
                        }
                    }
                    "BK" => {
                        let possility = king_cover(board.clone(), i, j);
                        if possility[row][col] == "M" || possility[row][col] == "C" {
                            bcheck = true;
                        }
                    }
                    _ => {}
                }
            }
        }
    } else {
        for i in 0..8 {
            for j in 0..8 {
                if board[i][j] == "BK".to_string() {
                    row = i;
                    col = j;
                }
            }
        }
        for i in 0..8 {
            for j in 0..8 {
                match board[i][j].as_str() {
                    "WP" => {
                        let possility = white_pawn_cover(board.clone(), i, j);
                        if possility[row][col] == "PC" || possility[row][col] == "C" {
                            bcheck = true;
                        }
                    }
                    "WR" => {
                        let possility = rook_cover(board.clone(), i, j);
                        if possility[row][col] == "M" || possility[row][col] == "C" {
                            bcheck = true;
                        }
                    }
                    "WB" => {
                        let possility = bishop_cover(board.clone(), i, j);
                        if possility[row][col] == "M" || possility[row][col] == "C" {
                            bcheck = true;
                        }
                    }
                    "WQ" => {
                        let possility = queen_cover(board.clone(), i, j);
                        if possility[row][col] == "M" || possility[row][col] == "C" {
                            bcheck = true;
                        }
                    }
                    "WN" => {
                        let possility = knight_cover(board.clone(), i, j);
                        if possility[row][col] == "M" || possility[row][col] == "C" {
                            bcheck = true;
                        }
                    }
                    "WK" => {
                        let possility = king_cover(board.clone(), i, j);
                        if possility[row][col] == "M" || possility[row][col] == "C" {
                            bcheck = true;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    return bcheck;
}
