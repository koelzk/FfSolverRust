
use ff_solver_lib::*;

fn main() {
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
