use std::rc::Rc;
use crate::{card_move::CardMove, Board};

/// Represents a card move and the resulting board state in the
/// context of a path finding strategy
pub struct BoardNode {
    /// Current board state
    pub board: Rc<Board>,

    /// Board state before [card_move] was applied
    pub previous: Option<Rc<Board>>,

    /// Card move from previous to current board
    pub card_move: Option<CardMove>,

    /// Move number
    pub step: u32,

    /// Score of current board
    pub score: i32
}

impl Ord for BoardNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for BoardNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for BoardNode {}

impl PartialEq for BoardNode {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::{Board, BoardNode};


    #[test]
    pub fn comparison() {
        let bn1 = BoardNode {
            board: Rc::new(Board::random(&mut rand::thread_rng())),
            previous: None,
            card_move: None,
            step: 0,
            score: 1
        };
        let bn0 = BoardNode {
            board: Rc::new(Board::random(&mut rand::thread_rng())),
            previous: None,
            card_move: None,
            step: 0,
            score: 0
        };
        let bnneg1 = BoardNode {
            board: Rc::new(Board::random(&mut rand::thread_rng())),
            previous: None,
            card_move: None,
            step: 0,
            score: -1
        };
        let bn0_2 = BoardNode {
            board: Rc::new(Board::random(&mut rand::thread_rng())),
            previous: None,
            card_move: None,
            step: 0,
            score: 0
        };

        assert!(bn1 > bn0);
        assert!(bn0 > bnneg1);
        assert!(bn0_2 >= bn0_2);
        assert!(bn0_2 <= bn0_2);
        assert!(bn0_2 == bn0_2);
        assert_eq!(bn0_2 > bn0_2, false);
        assert_eq!(bn0_2 < bn0_2, false);
    }
}
