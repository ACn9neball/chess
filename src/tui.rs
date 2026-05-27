use color_eyre::eyre::Ok;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::array;

use crate::logic::{b_pawn, bishop, king, knight, queen, rook, w_pawn};

pub fn start() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();

    let result = run(&mut terminal);

    ratatui::restore();

    result
}

fn run(terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
    let mut board = set_board();
    let mut display = set_display();
    let mut color_display = set_color(board.clone());
    let mut highlight: [[bool; 8]; 8] = array::from_fn(|_| array::from_fn(|_| false));
    let mut row = 7;
    let mut col = 4;
    highlight[row][col] = true;
    let mut current_row = 8;
    let mut current_col = 8;
    let mut next_row;
    let mut next_col;

    loop {
        terminal.draw(|frame| {
            render(
                frame,
                &highlight,
                &current_row,
                &current_col,
                display.clone(),
                color_display.clone(),
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

                            match board[current_row][current_col].as_str() {
                                "WP" => {
                                    board =
                                        w_pawn(board, current_row, current_col, next_row, next_col);
                                    display = w_pawn(
                                        display,
                                        current_row,
                                        current_col,
                                        next_row,
                                        next_col,
                                    );
                                }
                                "BP" => {
                                    board =
                                        b_pawn(board, current_row, current_col, next_row, next_col);
                                    display = b_pawn(
                                        display,
                                        current_row,
                                        current_col,
                                        next_row,
                                        next_col,
                                    );
                                }
                                "WR" => {
                                    board =
                                        rook(board, current_row, current_col, next_row, next_col);
                                    display =
                                        rook(display, current_row, current_col, next_row, next_col);
                                }
                                "BR" => {
                                    board =
                                        rook(board, current_row, current_col, next_row, next_col);
                                    display =
                                        rook(display, current_row, current_col, next_row, next_col);
                                }
                                "BB" => {
                                    board =
                                        bishop(board, current_row, current_col, next_row, next_col);
                                    display = bishop(
                                        display,
                                        current_row,
                                        current_col,
                                        next_row,
                                        next_col,
                                    );
                                }
                                "WB" => {
                                    board =
                                        bishop(board, current_row, current_col, next_row, next_col);
                                    display = bishop(
                                        display,
                                        current_row,
                                        current_col,
                                        next_row,
                                        next_col,
                                    );
                                }
                                "WK" => {
                                    board =
                                        king(board, current_row, current_col, next_row, next_col);
                                    display =
                                        king(display, current_row, current_col, next_row, next_col);
                                }
                                "BK" => {
                                    board =
                                        king(board, current_row, current_col, next_row, next_col);
                                    display =
                                        king(display, current_row, current_col, next_row, next_col);
                                }
                                "WQ" => {
                                    board =
                                        queen(board, current_row, current_col, next_row, next_col);
                                    display = queen(
                                        display,
                                        current_row,
                                        current_col,
                                        next_row,
                                        next_col,
                                    );
                                }
                                "BQ" => {
                                    board =
                                        queen(board, current_row, current_col, next_row, next_col);
                                    display = queen(
                                        display,
                                        current_row,
                                        current_col,
                                        next_row,
                                        next_col,
                                    );
                                }
                                "BN" => {
                                    board =
                                        knight(board, current_row, current_col, next_row, next_col);
                                    display = knight(
                                        display,
                                        current_row,
                                        current_col,
                                        next_row,
                                        next_col,
                                    );
                                }
                                "WN" => {
                                    board =
                                        knight(board, current_row, current_col, next_row, next_col);
                                    display = knight(
                                        display,
                                        current_row,
                                        current_col,
                                        next_row,
                                        next_col,
                                    );
                                }
                                _ => {}
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
) {
    let screen = frame.area();
    let split = Layout::horizontal([Constraint::Ratio(8, 16), Constraint::Ratio(8, 16)]);
    let [left, right] = split.areas(screen);
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

fn set_display() -> [[String; 8]; 8] {
    let mut display_board: [[String; 8]; 8] =
        array::from_fn(|_| array::from_fn(|_| String::from(".")));
    display_board[7][0] = "|_|_|\n ( )\n[___]".to_string();
    display_board[7][1] = "|\\\n ( \\\n[___]".to_string();
    display_board[7][2] = " o\n ( )\n[___]".to_string();
    display_board[7][3] = " www\n ( )\n[___]".to_string();
    display_board[7][4] = " -+-\n ( )\n[___]".to_string();
    display_board[7][5] = " o\n ( )\n[___]".to_string();
    display_board[7][6] = "|\\\n ( \\\n[___]".to_string();
    display_board[7][7] = "|_|_|\n ( )\n[___]".to_string();
    for row in 0..8 {
        display_board[1][row] = "[___]\n ( )".to_string();
        display_board[6][row] = "\n ( )\n[___]".to_string();
    }
    display_board[0][0] = "[___]\n ( )\n|-|-|".to_string();
    display_board[0][1] = "[___]\n( /\n|/".to_string();
    display_board[0][2] = "[___]\n ( )\n o".to_string();
    display_board[0][3] = "[___]\n ( )\nmmm".to_string();
    display_board[0][4] = "[___]\n ( )\n-+-".to_string();
    display_board[0][5] = "[___]\n ( )\n o".to_string();
    display_board[0][6] = "[___]\n( /\n|/".to_string();
    display_board[0][7] = "[___]\n ( )\n|-|-|".to_string();
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
