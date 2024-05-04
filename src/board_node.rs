use std::rc::Rc;
use crate::{card_move::CardMove, Board};


pub struct BoardNode {
    pub board: Rc<Board>,
    pub previous: Option<Rc<Board>>,
    pub card_move: Option<CardMove>,
    pub step: u32,
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
