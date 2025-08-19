use rand::seq::SliceRandom;
use std::ops::{Index, IndexMut};

use crate::{
    ANSI_CYAN, 
    ANSI_GREEN, 
    ANSI_RED,
    ANSI_RESET, 
    ANSI_YELLOW, 
    BOARD_HEIGHT, 
    BOARD_WIDTH, 
    Coord,
    TILE_SIZE, 
    Tile,
    beasts::{Beast, CommonBeast},
    levels::{LevelConfig, Level},
};

#[derive(Debug)]
pub struct Board {
    pub buffer: [[Tile; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Index<&Coord> for Board {
    type Output = Tile;

    fn index(&self, coord: &Coord) -> &Self::Output {
        &self.buffer[coord.row][coord.column]
    }
}

impl IndexMut<&Coord> for Board {
    fn index_mut(&mut self, coord: &Coord) -> &mut Self::Output {
        &mut self.buffer[coord.row][coord.column]
    }
}

impl Board {
    pub fn new(level: &Level) -> (Self, Vec<CommonBeast>) {
        let mut buffer = [[Tile::Empty; BOARD_WIDTH]; BOARD_HEIGHT];

        let mut all_coords = (0..BOARD_HEIGHT)
            .flat_map(
                |row| (0..BOARD_WIDTH).map(
                    move |column| Coord { column, row }
                )
            )
            .filter(|coord| !(coord.column == 0 && coord.row == 0))
            .collect::<Vec<Coord>>();

        let mut rng = rand::rng();
        all_coords.shuffle(&mut rng);

        buffer[0][0] = Tile::Player;

        let LevelConfig {
            block_count,
            static_block_count,
            common_beast_count,
        } = level.get_level_config();

        for _ in 0..block_count {
            let coord = all_coords.pop().expect(
                "We tried to place more blocks than there are available spaces on the board",
            );
            buffer[coord.row][coord.column] = Tile::Block;
        }

        for _ in 0..static_block_count {
            let coord = all_coords.pop().expect(
                "We tried to place more static blocks than there are available spaces on the board",
            );
            buffer[coord.row][coord.column] = Tile::StaticBlock;
        }

        let mut beasts = Vec::with_capacity(common_beast_count);
        for _ in 0..common_beast_count {
            let coord = all_coords.pop().expect(
                "We tried to place more beasts than there were available spaces on the board",
            );
            buffer[coord.row][coord.column] = Tile::CommonBeast;
            beasts.push(CommonBeast::new(coord));
        }

		(Self { buffer }, beasts)
    }

    pub fn render(&self) -> String {
        let mut output = format!(
            "{ANSI_YELLOW}▛{}▜{ANSI_RESET}\n", 
            "▀".repeat(BOARD_WIDTH * TILE_SIZE)
        );

        for rows in self.buffer {
            output.push_str(&format!("{ANSI_YELLOW}▌{ANSI_RESET}"));
            for tile in rows {
                match tile {
                    Tile::Empty => output.push_str("  "),
                    Tile::Player => {
                        output.push_str(&format!("{ANSI_CYAN}◀▶{ANSI_RESET}"))
                    },
                    Tile::Block => {
                        output.push_str(&format!("{ANSI_GREEN}░░{ANSI_RESET}"))
                    },
                    Tile::StaticBlock => {
                        output.push_str(&format!("{ANSI_YELLOW}▓▓{ANSI_RESET}"))
                    },
                    Tile::CommonBeast => {
                        output.push_str(&format!("{ANSI_RED}├┤{ANSI_RESET}"))
                    },
                }
            }
            output.push_str(&format!("{ANSI_YELLOW}▐{ANSI_RESET}\n"));
        }
        output.push_str(&format!(
            "{ANSI_YELLOW}▙{}▟{ANSI_RESET}", 
            "▄".repeat(BOARD_WIDTH * TILE_SIZE)
        ));

        output
    }
}