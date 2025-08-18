use std::{
    io::{Read, stdin},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crate::{
    BOARD_HEIGHT, 
    BOARD_WIDTH, 
    Direction, 
    TILE_SIZE, 
    beasts::CommonBeast,
    board::Board, 
    levels::Level, 
    player::Player
};

#[derive(Debug)]
pub struct Game{
    board: Board,
    player: Player,
    level: Level,
    beasts: Vec<CommonBeast>,
    input_receiver: mpsc::Receiver<u8>,
}

impl Game {
    pub fn new() -> Self {
        let (board, beasts) = Board::new();
        let (input_sender, input_receiver) = mpsc::channel::<u8>();
        let stdin = stdin();
        thread::spawn(move || {
            let mut lock = stdin.lock();
            let mut buffer = [0_u8; 1];
            while lock.read_exact(&mut buffer).is_ok() {
                if input_sender.send(buffer[0]).is_err() {
                    break;
                }
            }
        });

        Self {
            board,
            player: Player::new(),
            level: Level::One,
            beasts,
            input_receiver,
        }
    }

    pub fn play(&mut self) {
        let stdin = stdin();
        let mut lock = stdin.lock();
        let mut buffer = [0_u8; 1];
        println!("{}", self.render(false));

        loop {
            if let Ok(byte) = self.input_receiver.try_recv() {
                match byte as char {
                    'w' => {
                        self.player.advance(&mut self.board, &Direction::Up);
                    },
                    'd' => {
                        self.player.advance(&mut self.board, &Direction::Right);
                    },
                    's' => {
                        self.player.advance(&mut self.board, &Direction::Down);
                    },
                    'a' => {
                        self.player.advance(&mut self.board, &Direction::Left);
                    },
                    'q' => {
                        println!("Goodbye!");
                        break;
                    },
                    _ => {},
                }

                println!("{}", self.render(true));
            }
        }
    }

    fn render(&self, reset: bool) -> String {
        const BORDER_SIZE: usize = 1;
        const FOOTER_SIZE: usize = 1;

        let mut board = if reset {
            format!(
                "\x1B[{}F", 
                BORDER_SIZE + BOARD_HEIGHT + BORDER_SIZE + FOOTER_SIZE 
            )
        } 
        else { 
            String::new() 
        };
        board.push_str(&format!(
            "{board}\n{footer:>width$}{level}",
            board = self.board.render(),
            footer = "Level: ",
            level = self.level,
            width = BORDER_SIZE + BOARD_WIDTH * TILE_SIZE + BORDER_SIZE - FOOTER_SIZE,
        ));

        board
    }
}