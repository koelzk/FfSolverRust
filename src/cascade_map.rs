use itertools::Itertools;

use crate::{card_move::{CardMove, CELL, FOUNDATION}, Board, Card, CASCADE_COUNT};

/// Tracks reorderings of cascades in order to nomalize the cascade order of boards
/// to reduce the number of possible board states for solving.
pub struct CascadeMap {
    cascade_indices: [u8; CASCADE_COUNT as usize]
}

impl CascadeMap {
    /// Tracks the cascade ranks of the specified board
    pub fn advance(&mut self, board: &Board) {
        let new_indices = board.cascades()
            .iter()
            .zip(self.cascade_indices)
            .map(|(cc, cc_index)| (cc, cc_index, Self::cascade_rank(cc)))
            .sorted_by_key(|&(_, _, cc_rank)| cc_rank)
            .map(|(_, cc_index, _)| cc_index)
            .collect::<Vec<u8>>();

        self.cascade_indices = new_indices.try_into().unwrap();
    }

    /// Returns the specified card move with remapped cascade indices
    pub fn translate(&self, card_move: &CardMove) -> CardMove {
        CardMove::new(self.translate_index(card_move.from()), self.translate_index(card_move.to()), card_move.count())
    }
    
    fn translate_index(&self, index: u8) -> u8 {
        match index {
            _ if index < CASCADE_COUNT => self.cascade_indices[index as usize],
            CELL | FOUNDATION => index,
            _ => panic!("Unexpected index {index}.")
        }
     }

     /// Returns the rank of the specified cascade
     pub fn cascade_rank(cascade: &[Card]) -> u32 {
        match cascade.first() {
            Some(c) => u8::from(c) as u32,
            None => u32::MAX,
        }
    }
}

impl Default for CascadeMap {
    fn default() -> Self {
        Self {
            cascade_indices: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, SeedableRng};

    use crate::{card_move::{CardMove, CELL, FOUNDATION}, *};


    #[test]
    pub fn test() {
        let mut cascade_map = CascadeMap::default();
        assert_eq!(cascade_map.cascade_indices, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let card_move = CardMove::new(0, 5, 1);
        assert_eq!(card_move, cascade_map.translate(&card_move));

        let board = Board::random(&mut StdRng::seed_from_u64(1337));
        cascade_map.advance(&board);
        
        assert_eq!(cascade_map.cascade_indices, [8, 3, 7, 1, 4, 10, 0, 9, 2, 6, 5]);
        assert_eq!(CardMove::new(8, 10, 1), cascade_map.translate(&card_move));
        assert_eq!(CardMove::new(CELL, FOUNDATION, 2), cascade_map.translate(&CardMove::new(CELL, FOUNDATION, 2)));
    }
}