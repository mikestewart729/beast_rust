use std::io::{Read, stdin};

use crate::{BOARD_HEIGHT, Direction, board::Board, player::Player};

#[derive(Debug)]
pub struct Game{
    board: Board,
    player: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            player: Player::new(),
        }
    }

    pub fn play(&mut self) {
        let stdin = stdin();
        let mut lock = stdin.lock();
        let mut buffer = [0_u8; 1];
        println!("{}", self.board.render());

        while lock.read_exact(&mut buffer).is_ok() {
            match buffer[0] as char {
                'w' => {
                    self.player.advance(&mut self.board, Direction::Up);
                },
                'd' => {
                    self.player.advance(&mut self.board, Direction::Right);
                },
                's' => {
                    self.player.advance(&mut self.board, Direction::Down);
                },
                'a' => {
                    self.player.advance(&mut self.board, Direction::Left);
                },
                'q' => {
                    println!("Goodbye!");
                    break;
                },
                _ => {},
            }

            println!("\x1B[{}F{}", BOARD_HEIGHT + 1 + 1, self.board.render());
        }
    }
}