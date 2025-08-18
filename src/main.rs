#[derive(Copy, Clone, Debug)]
enum Tile {
    Empty,       // There will be empty spaces on our board "  "
    Player,      // We will need the player "◀▶"
    Block,       // Some tiles will be blocks "░░"
    StaticBlock, // Others will be blocks that can't be moved "▓▓"
}

#[derive(Debug)]
struct Board {
    buffer: [[Tile; 39]; 20],
}

impl Board {
    fn new() -> Self {
        Self {
            buffer: [[Tile::Empty; 39]; 20],
        }
    }

    fn render(&self) -> String {
        let mut output = String::new();

        for rows in self.buffer {
            for tile in rows {
                match tile {
                    Tile::Empty => output.push_str("  "),
                    Tile::Player => output.push_str("◀▶"),
                    Tile::Block => output.push_str("░░"),
                    Tile::StaticBlock => output.push_str("▓▓"),
                }
            }
            output.push('\n');
        }

        output
    }
}

fn main() {
    let board = Board::new();
    println!("{}", board.render());
}
