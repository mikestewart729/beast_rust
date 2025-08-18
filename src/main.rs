use std::io::{Read, stdin};

mod board;
mod raw_mode;

use crate::{board::Board, raw_mode::RawMode};

// Board Size constants
pub const BOARD_WIDTH: usize = 39;
pub const BOARD_HEIGHT: usize = 20;
pub const TILE_SIZE: usize = 2;
// ANSI color constants
pub const ANSI_YELLOW: &str = "\x1B[33m";
pub const ANSI_GREEN: &str = "\x1B[32m";
pub const ANSI_CYAN: &str = "\x1B[36m";
pub const ANSI_RESET: &str = "\x1B[39m";

#[derive(Copy, Clone, Debug)]
enum Tile {
    Empty,       // There will be empty spaces on our board "  "
    Player,      // We will need the player "◀▶"
    Block,       // Some tiles will be blocks "░░"
    StaticBlock, // Others will be blocks that can't be moved "▓▓"
}

#[derive(Debug)]
struct Game{
    board: Board,
}

impl Game {
    fn new() -> Self {
        Self {
            board: Board::new(),
        }
    }

    fn play(&self) {
        let stdin = stdin();
        let mut lock = stdin.lock();
        let mut buffer = [0_u8; 1];

        while lock.read_exact(&mut buffer).is_ok() {
            match buffer[0] as char {
                'w' => {
                    println!("Go up!");
                },
                'd' => {
                    println!("Go right!");
                },
                's' => {
                    println!("Go down!");
                },
                'a' => {
                    println!("Go left!");
                },
                'q' => {
                    println!("Goodbye!");
                    break;
                },
                _ => {},
            }
        }
    }
}

fn main() {
    let _raw_mode = RawMode::enter();

    let game = Game::new();
    game.play();
}
