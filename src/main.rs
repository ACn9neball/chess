mod logic;
mod possible;
mod tui;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Game {
    #[arg(short, long, value_enum)]
    display: Option<Commands>,
}

#[derive(ValueEnum, Clone)]
enum Commands {
    Cli,
    Tui,
}

fn main() {
    let game = Game::parse();
    if let Some(op) = game.display {
        match op {
            Commands::Cli => {}
            Commands::Tui => {
                tui::start().expect("");
            }
        }
    }
}
