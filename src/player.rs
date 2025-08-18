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
        let mut next_position = self.position;

        match direction {
            Direction::Up => {
                if next_position.row > 0 {
                    next_position.row -= 1
                }
            }, // 0, 0 is in upper left corner
            Direction::Right => {
                if next_position.column < BOARD_WIDTH - 1 {
                    next_position.column += 1
                }
            },
            Direction::Down => {
                if next_position.row < BOARD_HEIGHT - 1 {
                    next_position.row += 1
                }
            },
            Direction::Left => {
                if next_position.column > 0 {
                    next_position.column -= 1
                }
            },
        }

        match board[&next_position] {
            Tile::Empty => {
                board[&self.position] = Tile::Empty;
                self.position = next_position;
                board[&next_position] = Tile::Player;
            },
            Tile::Block => {
                // TODO: Need to move the block and any behind it
            },
            Tile::Player | Tile::StaticBlock => {},
        }
    }
}