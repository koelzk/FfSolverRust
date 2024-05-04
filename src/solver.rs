use std::{collections::{BinaryHeap, HashMap}, ops::Deref, rc::Rc};

use crate::{card_move::CardMove, Board, BoardNode};

#[repr(u8)]
pub enum SolveResultStatus {
    Solved,
    ReachedMaxIterations,
    NoSolution,
}

pub struct SolveResult {
    pub moves: Vec<CardMove>,
    pub iteration: usize,
    pub status: SolveResultStatus
}

impl SolveResult {
    pub fn solved(&self) -> bool {
        matches!(self.status, SolveResultStatus::Solved)
    }
}

pub struct Solver {
    start: Board,
    visited: HashMap<Rc<Board>, Rc<BoardNode>>,
    queue: BinaryHeap<Rc<BoardNode>>,
}

impl Solver {
    pub fn new(board: Board) -> Self {
        Solver {
            start: board,
            visited: Default::default(),
            queue: Default::default()
        }
    }

    pub fn solve(mut self, max_iter: usize, max_steps: usize, return_on_solve: bool) -> SolveResult {
        let mut board = self.start.clone();
        board.apply_auto_moves();

        let start_node = BoardNode {
            board: Rc::new(board),
            card_move: None,
            previous: None,
            step: 0,
            score: 0,
        };
        self.queue.push(Rc::new(start_node));

        let mut current_max_steps = max_steps;
        let mut solution_node: Option<Rc<BoardNode>> = None;

        for iteration in 0..max_iter {
            if self.queue.is_empty() {
                return SolveResult {
                    iteration,
                    moves: vec![],
                    status: SolveResultStatus::NoSolution
                }
            }

            let current_node = self.queue.pop().unwrap();
            let current = current_node.board.deref();

            if current.is_game_won() {
                if matches!(&solution_node, None) || matches!(&solution_node, Some(n) if solution_node.unwrap().step > current_node.step) {
                    solution_node = Some(current_node);
                }
            }
        }


        SolveResult {
            iteration: 0,
            moves: vec![],
            status: SolveResultStatus::NoSolution
        }

    }
}