// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Sudoku solver using propagator-based constraint satisfaction.
//!
//! This example demonstrates using amb (nondeterministic choice) with
//! backtracking search to solve constraint satisfaction problems.
//!
//! Run with: `cargo run --example sudoku_chirho`

use propagators_chirho::{AmbChirho, BacktrackingSearchChirho, SearchResultChirho};

/// Checks if a partial Sudoku assignment is consistent.
fn is_consistent_chirho(board_chirho: &[Option<u8>]) -> bool {
    // Check rows
    for row_chirho in 0..9 {
        let mut seen_chirho = [false; 10];
        for col_chirho in 0..9 {
            if let Some(val_chirho) = board_chirho[row_chirho * 9 + col_chirho] {
                if seen_chirho[val_chirho as usize] {
                    return false;
                }
                seen_chirho[val_chirho as usize] = true;
            }
        }
    }

    // Check columns
    for col_chirho in 0..9 {
        let mut seen_chirho = [false; 10];
        for row_chirho in 0..9 {
            if let Some(val_chirho) = board_chirho[row_chirho * 9 + col_chirho] {
                if seen_chirho[val_chirho as usize] {
                    return false;
                }
                seen_chirho[val_chirho as usize] = true;
            }
        }
    }

    // Check 3x3 boxes
    for box_row_chirho in 0..3 {
        for box_col_chirho in 0..3 {
            let mut seen_chirho = [false; 10];
            for i_chirho in 0..3 {
                for j_chirho in 0..3 {
                    let row_chirho = box_row_chirho * 3 + i_chirho;
                    let col_chirho = box_col_chirho * 3 + j_chirho;
                    if let Some(val_chirho) = board_chirho[row_chirho * 9 + col_chirho] {
                        if seen_chirho[val_chirho as usize] {
                            return false;
                        }
                        seen_chirho[val_chirho as usize] = true;
                    }
                }
            }
        }
    }

    true
}

fn solve_sudoku_chirho(initial_chirho: &[Option<u8>]) -> Option<[u8; 81]> {
    // Find empty cells and create ambs for them
    let mut ambs_chirho = Vec::new();
    let mut cell_indices_chirho = Vec::new();

    for (i_chirho, cell_chirho) in initial_chirho.iter().enumerate() {
        if cell_chirho.is_none() {
            ambs_chirho.push(AmbChirho::range_chirho(1, 9));
            cell_indices_chirho.push(i_chirho);
        }
    }

    if ambs_chirho.is_empty() {
        // Already solved
        let mut result_chirho = [0u8; 81];
        for (i_chirho, cell_chirho) in initial_chirho.iter().enumerate() {
            result_chirho[i_chirho] = cell_chirho.unwrap();
        }
        return Some(result_chirho);
    }

    let initial_board_chirho = initial_chirho.to_vec();
    let cell_indices_copy_chirho = cell_indices_chirho.clone();

    let search_chirho = BacktrackingSearchChirho::with_limit_chirho(100000);

    let result_chirho = search_chirho.search_simple_chirho(&ambs_chirho, |values_chirho| {
        let mut board_chirho = initial_board_chirho.clone();

        for (i_chirho, val_chirho) in values_chirho.iter().enumerate() {
            let cell_idx_chirho = cell_indices_copy_chirho[i_chirho];
            let num_chirho = val_chirho.as_interval_chirho().unwrap().lo_chirho as u8;
            board_chirho[cell_idx_chirho] = Some(num_chirho);
        }

        is_consistent_chirho(&board_chirho)
    });

    match result_chirho {
        SearchResultChirho::SolutionChirho(solution_chirho) => {
            let mut result_chirho = [0u8; 81];
            for (i_chirho, cell_chirho) in initial_chirho.iter().enumerate() {
                if let Some(val_chirho) = cell_chirho {
                    result_chirho[i_chirho] = *val_chirho;
                }
            }
            for (i_chirho, val_chirho) in solution_chirho.iter().enumerate() {
                let cell_idx_chirho = cell_indices_chirho[i_chirho];
                result_chirho[cell_idx_chirho] =
                    val_chirho.as_interval_chirho().unwrap().lo_chirho as u8;
            }
            Some(result_chirho)
        }
        _ => None,
    }
}

fn print_board_chirho(board_chirho: &[u8]) {
    for row_chirho in 0..9 {
        if row_chirho % 3 == 0 && row_chirho > 0 {
            println!("------+-------+------");
        }
        for col_chirho in 0..9 {
            if col_chirho % 3 == 0 && col_chirho > 0 {
                print!("| ");
            }
            print!("{} ", board_chirho[row_chirho * 9 + col_chirho]);
        }
        println!();
    }
}

fn main_chirho() {
    println!("=== Sudoku Solver with Propagators ===\n");

    // A simple Sudoku puzzle (0 = empty)
    // This is an easy puzzle for demonstration
    #[rustfmt::skip]
    let puzzle_chirho: [u8; 81] = [
        5, 3, 0,  0, 7, 0,  0, 0, 0,
        6, 0, 0,  1, 9, 5,  0, 0, 0,
        0, 9, 8,  0, 0, 0,  0, 6, 0,

        8, 0, 0,  0, 6, 0,  0, 0, 3,
        4, 0, 0,  8, 0, 3,  0, 0, 1,
        7, 0, 0,  0, 2, 0,  0, 0, 6,

        0, 6, 0,  0, 0, 0,  2, 8, 0,
        0, 0, 0,  4, 1, 9,  0, 0, 5,
        0, 0, 0,  0, 8, 0,  0, 7, 9,
    ];

    println!("Puzzle:");
    print_board_chirho(&puzzle_chirho);
    println!();

    // Convert to Option<u8> format
    let initial_chirho: Vec<Option<u8>> = puzzle_chirho
        .iter()
        .map(|&x_chirho| if x_chirho == 0 { None } else { Some(x_chirho) })
        .collect();

    println!("Solving...");

    match solve_sudoku_chirho(&initial_chirho) {
        Some(solution_chirho) => {
            println!("\nSolution:");
            print_board_chirho(&solution_chirho);
            println!("\n✓ Solved using backtracking search with amb!");
        }
        None => {
            println!("\n✗ No solution found");
        }
    }
}

fn main() {
    main_chirho();
}
