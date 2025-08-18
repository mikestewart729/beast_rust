use rand::seq::SliceRandom;

use crate::{
    ANSI_CYAN, 
    ANSI_GREEN, 
    ANSI_RESET, 
    ANSI_YELLOW, 
    BOARD_HEIGHT, 
    BOARD_WIDTH, 
    TILE_SIZE, 
    Tile,
};

#[derive(Debug)]
pub struct Board {
    pub buffer: [[Tile; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        let mut buffer = [[Tile::Empty; BOARD_WIDTH]; BOARD_HEIGHT];

        let mut all_coords = (0..BOARD_HEIGHT)
            .flat_map(
                |row| (0..BOARD_WIDTH).map(move |column| (column, row))
            )
            .filter(|coord| !(coord.0 == 0 && coord.1 == 0))
            .collect::<Vec<(usize, usize)>>();

        let mut rng = rand::rng();
        all_coords.shuffle(&mut rng);

        buffer[0][0] = Tile::Player;

        for _ in 0..50 {
            let coord = all_coords.pop().expect(
                "We tried to place more blocks than there are available spaces on the board",
            );
            buffer[coord.1][coord.0] = Tile::Block;
        }

        for _ in 0..5 {
            let coord = all_coords.pop().expect(
                "We tried to place more static blocks than there are available spaces on the board",
            );
            buffer[coord.1][coord.0] = Tile::StaticBlock;
        }

		Self { buffer }
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