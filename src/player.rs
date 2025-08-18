use crate::{BOARD_HEIGHT, BOARD_WIDTH, Coord, Direction, Tile, board::Board};

#[derive(Debug)]
pub struct Player {
    position: Coord,
}

impl Player {
    pub fn new() -> Self {
        Self { position: Coord { column: 0, row: 0 } }
    }

    pub fn advance(&mut self, board: &mut Board, direction: Direction) {
        board[&self.position] = Tile::Empty;

        match direction {
            Direction::Up => {
                if self.position.row > 0 {
                    self.position.row -= 1
                }
            }, // 0, 0 is in upper left corner
            Direction::Right => {
                if self.position.column < BOARD_WIDTH - 1 {
                    self.position.column += 1
                }
            },
            Direction::Down => {
                if self.position.row < BOARD_HEIGHT - 1 {
                    self.position.row += 1
                }
            },
            Direction::Left => {
                if self.position.column > 0 {
                    self.position.column -= 1
                }
            },
        }

        board[&self.position] = Tile::Player;
    }
}