use std::{collections::{BinaryHeap, HashMap}, ops::Deref, rc::Rc};

use crate::{card_move::CardMove, Board, BoardNode, BoardNormalization};

#[repr(u8)]
pub enum SolveResultStatus {
    Solved,
    ReachedMaxIterations,
    NoSolution,
}

pub struct SolveResult {
    pub moves: Vec<CardMove>,
    pub iteration: u32,
    pub status: SolveResultStatus
}

impl SolveResult {
    pub fn new(moves: Vec<CardMove>, iteration: u32, status: SolveResultStatus) -> Self {
        SolveResult {
            moves,
            iteration,
            status
        }
    }

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

    pub fn solve(mut self, max_iter: u32, max_steps: u32, return_on_solve: bool) -> SolveResult {
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
        let mut iteration = 0u32;

        while iteration < max_iter {
            if self.queue.is_empty() {
                return SolveResult::new(vec![], iteration, SolveResultStatus::NoSolution);
            }

            let current_node = self.queue.pop().unwrap();
            let current = current_node.board.deref();

            if current.is_game_won() {
                if matches!(&solution_node, None) ||
                    matches!(&solution_node, Some(n) if (*n).step > current_node.step) {
                    solution_node = Some(current_node.clone());
                    current_max_steps = solution_node.as_ref().unwrap().as_ref().step - 1;
                }

                if return_on_solve {
                    break;
                }
            }

            iteration += 1;

            if current_node.step >= current_max_steps {
                continue;
            }

            let mut card_moves = Vec::<CardMove>::new();
            current.enumerate_moves(&mut card_moves);

            for card_move in card_moves {
                self.add_node(current_node.as_ref(), card_move);
            }
        }

        match solution_node {
            Some(node) => SolveResult::new(self.assemble_moves(&node.board), iteration, SolveResultStatus::Solved),
            None => SolveResult::new(vec![], iteration, SolveResultStatus::ReachedMaxIterations),
        }
    }
    
    fn add_node(&mut self, current_node: &BoardNode, card_move: CardMove) {
        let mut next = current_node.board.as_ref().clone();
        next.apply_move(&card_move);
        next.apply_auto_moves(); //TODO Normalization?

        let step = current_node.step + 1;
        let score = next.score(step);        
        let next_ref = Rc::new(next);

        let next_node = Rc::new(BoardNode {
            board: next_ref.clone(),
            previous: Some(current_node.board.clone()),
            card_move: Some(card_move),
            step,
            score
        });

        self.visited.entry(next_ref).
            and_modify(|n| if step < n.as_ref().step {
                *n = next_node.clone();
            }).or_insert_with(|| {
                self.queue.push(next_node.clone());
                next_node
            });
    }

    fn assemble_moves(&self, end: &Rc<Board>) -> Vec<CardMove> {
        let mut node_stack = Vec::<&BoardNode>::new();

        let mut b = end.clone();

        loop {
            match self.visited.get(&b) {
                Some(node) if node.previous.as_deref().is_some() => {
                    node_stack.push(node);
                    b = node.previous.clone().unwrap();
                }
                _ => break
            }
        }

        let mut norm = BoardNormalization::new();

        node_stack.iter().rev()
        .map(|&node| {
            assert!(node.card_move.is_some() && node.previous.is_some());

            let mut current_no_norm = node.previous.as_deref().unwrap().clone();
            current_no_norm.apply_move(&node.card_move.as_ref().unwrap());
            current_no_norm.apply_auto_moves();

            let translated_move = norm.translate(node.card_move.as_ref().unwrap());
            norm.advance(&current_no_norm);
            translated_move
        })
        .collect::<Vec<CardMove>>()
    }
}