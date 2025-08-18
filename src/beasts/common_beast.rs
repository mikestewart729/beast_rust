use crate::{Coord, beasts::Beast, board::Board};

#[derive(Debug)]
pub struct CommonBeast {
    pub position: Coord,
}

impl Beast for CommonBeast {
    fn new(position: Coord) -> Self {
        Self { position }
    }

    fn advance(
        &mut self,
        board: &Board,
        player_position: &Coord,
    ) -> Option<Coord> {
        let mut next_position = self.position;
        if next_position.column > 0 {
            next_position.column -= 1;
            return Some(next_position);
        }

        None
    }
}