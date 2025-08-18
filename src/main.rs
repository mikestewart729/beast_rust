mod beasts;
mod board;
mod game;
mod levels;
mod player;
mod raw_mode;

use crate::{game::Game, raw_mode::RawMode};

// Board Size constants
pub const BOARD_WIDTH: usize = 39;
pub const BOARD_HEIGHT: usize = 20;
pub const TILE_SIZE: usize = 2;
// ANSI color constants
pub const ANSI_YELLOW: &str = "\x1B[33m";
pub const ANSI_GREEN: &str = "\x1B[32m";
pub const ANSI_CYAN: &str = "\x1B[36m";
pub const ANSI_RED: &str = "\x1B[31m";
pub const ANSI_RESET: &str = "\x1B[39m";

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Empty,       // There will be empty spaces on our board "  "
    Player,      // We will need the player "◀▶"
    Block,       // Some tiles will be blocks "░░"
    StaticBlock, // Others will be blocks that can't be moved "▓▓"
    CommonBeast,
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug)]
pub struct Coord {
    column: usize,
    row: usize,
}

fn main() {
    let _raw_mode = RawMode::enter();

    let mut game = Game::new();
    game.play();
}
