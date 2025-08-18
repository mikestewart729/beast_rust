// Board Size constants
const BOARD_WIDTH: usize = 39;
const BOARD_HEIGHT: usize = 20;
const TILE_SIZE: usize = 2;
// ANSI color constants
const ANSI_YELLOW: &str = "\x1B[33m";
const ANSI_GREEN: &str = "\x1B[32m";
const ANSI_CYAN: &str = "\x1B[36m";
const ANSI_RESET: &str = "\x1B[39m";

#[derive(Copy, Clone, Debug)]
enum Tile {
    Empty,       // There will be empty spaces on our board "  "
    Player,      // We will need the player "◀▶"
    Block,       // Some tiles will be blocks "░░"
    StaticBlock, // Others will be blocks that can't be moved "▓▓"
}

#[derive(Debug)]
struct Board {
    buffer: [[Tile; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    fn new() -> Self {
        let mut buffer = [[Tile::Empty; BOARD_WIDTH]; BOARD_HEIGHT];

        buffer[0][0] = Tile::Player;
		buffer[2][5] = Tile::Block;
		buffer[2][6] = Tile::Block;
		buffer[2][7] = Tile::Block;
		buffer[3][6] = Tile::StaticBlock;

		Self { buffer }
    }

    fn render(&self) -> String {
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

fn main() {
    let board = Board::new();
    println!("{}", board.render());
}
