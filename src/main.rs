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
}

fn main() {
    println!("{:?}", Board::new());
}
