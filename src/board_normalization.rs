use itertools::Itertools;

use crate::{card_move::CardMove, Board, Card, CASCADE_COUNT};


pub struct BoardNormalization {
    cascade_indices: [u8; CASCADE_COUNT as usize]
}

impl BoardNormalization {
    pub fn advance(&mut self, board: &Board) {
        let new_indices = board.cascades()
            .iter()
            .zip(self.cascade_indices)
            .map(|(cc, ci)| (cc, ci, Self::cascade_rank(cc)))
            .sorted_by_key(|&(_, _, si)| si)
            .map(|(_, ci, _)| ci)
            .collect::<Vec<u8>>();

        self.cascade_indices = new_indices.try_into().unwrap();
    }

    pub fn translate(&self, card_move: &CardMove) -> CardMove {
        CardMove::new(self.translate_index(card_move.from()), self.translate_index(card_move.to()), card_move.count())
    }
    
    fn translate_index(&self, index: u8) -> u8 {
        match index < CASCADE_COUNT {
            true => self.cascade_indices[index as usize],
            false => index,
        }
     }

     pub fn cascade_rank(cascade: &[Card]) -> u32 {
        match cascade.first() {
            Some(c) => u8::from(c) as u32,
            None => u32::MAX,
        }
    }
}

impl Default for BoardNormalization {
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
        let mut bn = BoardNormalization::default();
        assert_eq!(bn.cascade_indices, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let card_move = CardMove::new(0, 5, 1);
        assert_eq!(card_move, bn.translate(&card_move));

        let board = Board::random(&mut StdRng::seed_from_u64(1337));
        bn.advance(&board);
        
        assert_eq!(bn.cascade_indices, [8, 3, 7, 1, 4, 10, 0, 9, 2, 6, 5]);
        assert_eq!(CardMove::new(8, 10, 1), bn.translate(&card_move));
        assert_eq!(CardMove::new(CELL, FOUNDATION, 2), bn.translate(&CardMove::new(CELL, FOUNDATION, 2)));
    }
}