
use ff_solver_lib::*;
use rand::{rngs::StdRng, SeedableRng};

fn main() {
    for seed in 0..10_000 {
        let mut rng = &mut StdRng::seed_from_u64(seed);
        let board = Board::random(&mut rng);

        let solver = Solver::new(&board);
        let result = solver.solve(100_000, 100, true);

        let result_str = match result.status {
            SolveResultStatus::Solved => format!("{:3}", result.moves.len()),
            SolveResultStatus::ReachedMaxIterations => "?".to_owned(),
            SolveResultStatus::NoSolution => "-".to_owned(),
        };
        println!("{:4} => {}", seed, result_str);
    }
}
