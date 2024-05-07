use std::fs;
use clap::Parser;
use rand::{rngs::StdRng, SeedableRng};

use ff_solver_lib::*;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to input file
    #[arg(short='i', long="input")]
    input_file_path: String,

    /// Maximum number of iterations
    #[arg(short='m', long="max-iter", default_value_t = 100_000)]
    max_iter: u32,

    /// Maximum number of moves a solution may have
    #[arg(short='s', long="steps", default_value_t = 80)]
    max_steps: u32,

    /// Evaluate all iterations to find best solution
    #[arg(short='f', long="full", default_value_t = false)]
    full_iteration: bool,

    /// Display board for each move
    #[arg(short='h', long="hide-boards", default_value_t = false)]
    hide_boards: bool,
}

fn main() {
    let args = Args::parse();
    let text = fs::read_to_string(&args.input_file_path).unwrap();
    let board = parse_board(&text, None).unwrap();

    println!("Board state from {}:\n{}", &args.input_file_path, &board);

    let solver = Solver::new(&board);

    println!("Max iterations: {}", args.max_iter);
    println!("Max steps: {}", args.max_steps);
    println!("Evaluate all iterations: {}", args.full_iteration);

    println!("\nSolving...\n");

    let result = solver.solve(args.max_iter, args.max_steps, !args.full_iteration);

    match result.status {
        SolveResultStatus::Solved => println!("Found a solution with {} moves.", result.moves.len()),
        SolveResultStatus::ReachedMaxIterations => println!("No solution found."),
        SolveResultStatus::NoSolution => println!("No solution exists."),
    };

    if let SolveResultStatus::Solved = result.status {
        println!("");

        let mut b = board.clone();
        for (index, card_move) in result.moves.iter().enumerate() {
            if args.hide_boards {
                println!("Move {} - {}", index + 1, card_move);
            }
            else {
                b.apply_move(card_move);
                b.apply_auto_moves();
                println!("Move {} - {}:\n{}\n", index + 1, card_move, b);
            }
        }
    }
    // for seed in 0..10_000 {
    //     let mut rng = &mut StdRng::seed_from_u64(seed);
    //     let board = Board::random(&mut rng);

    //     let solver = Solver::new(&board);
    //     let result = solver.solve(100_000, 100, true);

    //     let result_str = match result.status {
    //         SolveResultStatus::Solved => format!("{:3}", result.moves.len()),
    //         SolveResultStatus::ReachedMaxIterations => "?".to_owned(),
    //         SolveResultStatus::NoSolution => "-".to_owned(),
    //     };
    //     println!("{:4} => {}", seed, result_str);
    // }
}
