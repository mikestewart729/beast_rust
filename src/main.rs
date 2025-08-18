const BOARD_WIDTH: usize = 39;
const BOARD_HEIGHT: usize = 20;
const TILE_SIZE: usize = 2;

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
        let mut output = format!("▛{}▜\n", "▀".repeat(BOARD_WIDTH * TILE_SIZE));

        for rows in self.buffer {
            output.push_str("▌");
            for tile in rows {
                match tile {
                    Tile::Empty => output.push_str("  "),
                    Tile::Player => output.push_str("◀▶"),
                    Tile::Block => output.push_str("░░"),
                    Tile::StaticBlock => output.push_str("▓▓"),
                }
            }
            output.push_str("▐\n");
        }
        output.push_str(&format!("▙{}▟", "▄".repeat(BOARD_WIDTH * TILE_SIZE)));

        output
    }
}

fn main() {
    let board = Board::new();
    println!("{}", board.render());
}
