use std::{collections::{BinaryHeap, HashMap}, ops::Deref, rc::Rc};

use crate::{card_move::CardMove, Board, BoardNode, BoardNormalization};

#[repr(u8)]
#[derive(Debug)]
pub enum SolveResultStatus {
    Solved,
    ReachedMaxIterations,
    NoSolution,
}

#[derive(Debug)]
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
        next.normalize();

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

#[cfg(test)]
mod tests {
    use std::{collections::BinaryHeap, rc::Rc};

    use crate::{parse_board, Solver};

    #[test]
    pub fn solve_simple() {
        let board = parse_board("
            JG	-	-	-	-	-	-	-	2	-	-
            KG	-	-	-	-	-	-	-	1	-	-
            QG	-	-	-	-	-	-	-	0	-	-", None).unwrap();
        
        let solver = Solver::new(board);
        let result = solver.solve(20_000, 100, true);

        let b2 = parse_board("
        7R  3G  6G  QG  3B  6B  QB  5Y  10  13  21
        -   3R  4R  3Y  QR  8Y  7G   2  4Y  KG 10G
        -   11  KB  5B  2Y  JR  19   9  6R  5R   1
        -   QY  8G  8B 10Y  JG   0  4B  2B  9Y   4
        -   6Y  5G   8  7B  4G  2R   7  14  16  2G
        -   20  9R  18  KY   3  7Y  15  12  JY  8R
        -  10B   6  -  10R   5  JB  KR  9G  9B  17", None).unwrap();

        println!("{}", b2.score(1));
    }

    #[test]
    pub fn test_debug() {
        let board = parse_board("
       7B   -  3Y  6G  5B  KY 10R  QB   -  5G   -
        -   -  KR  7G  4B  QY  JB  KB   -  21   -
        -   -   -  8G   -  JY 10B   -   -  8B   -
        -   -   -  9G   - 10Y  9B   -   -  3B   -
        -   -   - 10G   -  9Y   -   -   -  2Y   -
        -   -   -  JG   -  8Y   -   -   -  6B   -
        -   -   -  QG   -  7Y   -   -   -  QR   -
        -   -   -  KG   -  6Y   -   -   -  JR   -
        -   -   -   -   -  5Y   -   -   -   -   -
        -   -   -   -   -  4Y   -   -   -   -   -", None).unwrap();

        println!("{}", board);
        let solver = Solver::new(board);
        let result = solver.solve(20_000, 100, true);
        assert!(result.solved());
    }

    #[test]
    pub fn test_priority_queue() {
        let mut bh = BinaryHeap::new();
        bh.push(Rc::new(100));
        bh.push(Rc::new(000));
        bh.push(Rc::new(001));
        bh.push(Rc::new(010));
        

        println!("{:?}", bh.pop());
        println!("{:?}", bh.pop());
        println!("{:?}", bh.pop());
        println!("{:?}", bh.pop());
    }


    #[test]
    pub fn solve() {
        let board = parse_board("
            13	5Y	3B	6G	QG	-	6B	21	QB	3G	10
            KG	2	QR	4R	3Y	-	8Y	10G	7G	3R	4Y
            5R	9	2Y	KB	5B	-	JR	1	19	11	6R
            9Y	4B	10Y	8G	8B	-	JG	4	0	QY	2B
            16	7	7B	5G	8	-	4G	2G	2R	6Y	14
            JY	15	KY	9R	18	-	3	8R	7Y	20	12
            9B	KR	10R	6	7R	-	5	17	JB	10B	9G", None).unwrap();
        
        let solver = Solver::new(board);
        let result = solver.solve(200_000, 500, true);

        println!("{result:?}");
    }

}