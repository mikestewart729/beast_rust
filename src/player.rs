use rand::Rng;

use crate::{BOARD_HEIGHT, BOARD_WIDTH, Coord, Direction, Tile, board::Board};

#[derive(Debug)]
pub struct Player {
    pub position: Coord,
    pub lives: usize,
}

impl Player {
    pub fn new() -> Self {
        Self { 
            position: Coord { column: 0, row: 0 },
            lives: 3,
        }
    }

    fn get_next_position(position: Coord, direction: &Direction) -> Option<Coord> {
        let mut next_position = position;

        match direction {
            Direction::Up => {
                if next_position.row > 0 {
                    next_position.row -= 1
                } else {
                    return None;
                }
            }, // 0, 0 is in upper left corner
            Direction::Right => {
                if next_position.column < BOARD_WIDTH - 1 {
                    next_position.column += 1
                } else {
                    return None;
                }
            },
            Direction::Down => {
                if next_position.row < BOARD_HEIGHT - 1 {
                    next_position.row += 1
                } else {
                    return None;
                }
            },
            Direction::Left => {
                if next_position.column > 0 {
                    next_position.column -= 1
                } else {
                    return None;
                }
            },
        }

        Some(next_position)
    }

    pub fn advance(&mut self, board: &mut Board, direction: &Direction) {
        if let Some(first_position) = 
            Self::get_next_position(self.position, direction) 
        {
            match board[&first_position] {
                Tile::Empty => {
                    board[&self.position] = Tile::Empty;
                    self.position = first_position;
                    board[&first_position] = Tile::Player;
                },
                Tile::Block => {
                    let mut current_tile = Tile::Block;
                    let mut current_position = first_position;

                    while current_tile == Tile::Block {
                        if let Some(next_position) = 
                            Self::get_next_position(current_position, direction)
                        {
                            current_position = next_position;
                            current_tile = board[&current_position];

                            match current_tile {
                                Tile::Block => { /* Continue Looking */},
                                Tile::Empty => {
                                    board[&self.position] = Tile::Empty;
                                    self.position = first_position;
                                    board[&first_position] = Tile::Player;
                                    board[&current_position] = Tile::Block;
                                },
                                Tile::StaticBlock | Tile::Player | Tile::CommonBeast => { break; },
                            }
                        } else {
                            break;
                        }
                    }
                },
                Tile::Player | Tile::StaticBlock => {},
                Tile::CommonBeast => {
                    todo!("The player ran into a beast and died");
                },
            }
        }
    }

    pub fn respawn(&mut self, board: &mut Board) {
        let mut new_position = self.position;

        let mut rng = rand::rng();
        while board[&new_position] != Tile::Empty {
            new_position = Coord {
                column: rng.random_range(0..BOARD_WIDTH),
                row: rng.random_range(0..BOARD_HEIGHT),
            };
        }

        self.position = new_position;
        board[&new_position] = Tile::Player;
    }
}