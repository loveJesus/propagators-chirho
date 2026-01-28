// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Search heuristics example demonstrating variable and value ordering.
//!
//! This example shows how to use the search heuristics added in GAP-007:
//! - FirstFailChirho: Select variable with smallest domain
//! - DomWdegChirho: Domain over weighted degree
//! - ImpactBasedChirho: Impact-based variable selection
//! - MinValueChirho, MaxValueChirho, MiddleOutChirho: Value orderings
//!
//! Run with: `cargo run --example search_heuristics_chirho`

use propagators_chirho::{
    AmbChirho, BacktrackingSearchChirho, DomWdegChirho, FirstFailChirho, ImpactBasedChirho,
    SearchResultChirho, VariableOrderingChirho,
};

/// Solve the N-Queens problem for a given board size.
///
/// N-Queens asks: place N queens on an NxN chessboard such that no two
/// queens attack each other (same row, column, or diagonal).
fn solve_n_queens_chirho(n_chirho: usize) -> Option<Vec<usize>> {
    // Each variable represents a column, value represents the row
    let ambs_chirho: Vec<AmbChirho> = (0..n_chirho)
        .map(|_| AmbChirho::range_chirho(0, (n_chirho - 1) as i32))
        .collect();

    let search_chirho = BacktrackingSearchChirho::with_limit_chirho(1_000_000);

    let result_chirho = search_chirho.search_simple_chirho(&ambs_chirho, |values_chirho| {
        // Check all pairs for conflicts
        for i_chirho in 0..values_chirho.len() {
            let row_i_chirho = values_chirho[i_chirho]
                .as_interval_chirho()
                .map(|iv_chirho| iv_chirho.lo_chirho as i32)
                .unwrap_or(-1);

            for j_chirho in (i_chirho + 1)..values_chirho.len() {
                let row_j_chirho = values_chirho[j_chirho]
                    .as_interval_chirho()
                    .map(|iv_chirho| iv_chirho.lo_chirho as i32)
                    .unwrap_or(-1);

                if row_i_chirho < 0 || row_j_chirho < 0 {
                    continue;
                }

                // Same row
                if row_i_chirho == row_j_chirho {
                    return false;
                }

                // Same diagonal
                let col_diff_chirho = (j_chirho - i_chirho) as i32;
                let row_diff_chirho = (row_j_chirho - row_i_chirho).abs();
                if col_diff_chirho == row_diff_chirho {
                    return false;
                }
            }
        }
        true
    });

    match result_chirho {
        SearchResultChirho::SolutionChirho(solution_chirho) => {
            let board_chirho: Vec<usize> = solution_chirho
                .iter()
                .map(|v_chirho| v_chirho.as_interval_chirho().unwrap().lo_chirho as usize)
                .collect();
            Some(board_chirho)
        }
        _ => None,
    }
}

/// Print an N-Queens solution.
fn print_queens_chirho(solution_chirho: &[usize]) {
    let n_chirho = solution_chirho.len();
    println!("  {}", (0..n_chirho).map(|c_chirho| format!("{}", c_chirho)).collect::<Vec<_>>().join(" "));
    for row_chirho in 0..n_chirho {
        print!("{} ", row_chirho);
        for col_chirho in 0..n_chirho {
            if solution_chirho[col_chirho] == row_chirho {
                print!("Q ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

/// Demonstrate variable ordering heuristics.
fn demonstrate_heuristics_chirho() {
    println!("=== Variable Ordering Heuristics Demo ===\n");

    // Setup: 5 variables with different domain sizes
    let domain_sizes_chirho = vec![5, 2, 8, 3, 1]; // Variable 4 has domain size 1
    let assigned_chirho = vec![false, false, false, false, false];

    println!("Domain sizes: {:?}", domain_sizes_chirho);
    println!("Assigned:     {:?}\n", assigned_chirho);

    // 1. First-Fail heuristic
    let first_fail_chirho = FirstFailChirho::new_chirho();
    let selected_ff_chirho = first_fail_chirho.select_variable_chirho(&domain_sizes_chirho, &assigned_chirho);
    println!("FirstFail selects: variable {} (domain size {})",
             selected_ff_chirho.unwrap(),
             domain_sizes_chirho[selected_ff_chirho.unwrap()]);

    // 2. Dom/Wdeg heuristic
    // Constraint topology: constraints involving pairs of variables
    let constraint_vars_chirho = vec![
        vec![0, 1], // Constraint 0 involves vars 0, 1
        vec![1, 2], // Constraint 1 involves vars 1, 2
        vec![2, 3], // Constraint 2 involves vars 2, 3
        vec![0, 4], // Constraint 3 involves vars 0, 4
    ];
    let dom_wdeg_chirho = DomWdegChirho::new_chirho(5, constraint_vars_chirho);

    // Simulate some failures on constraint 1 (involves vars 1, 2)
    dom_wdeg_chirho.record_failure_chirho(1);
    dom_wdeg_chirho.record_failure_chirho(1);
    dom_wdeg_chirho.record_failure_chirho(1);

    let selected_dw_chirho = dom_wdeg_chirho.select_variable_chirho(&domain_sizes_chirho, &assigned_chirho);
    println!(
        "Dom/Wdeg selects: variable {} (dom={}, wdeg={:.1})",
        selected_dw_chirho.unwrap(),
        domain_sizes_chirho[selected_dw_chirho.unwrap()],
        dom_wdeg_chirho.weighted_degree_chirho(selected_dw_chirho.unwrap())
    );

    // 3. Impact-Based heuristic
    let impact_chirho = ImpactBasedChirho::new_chirho(5);
    // Record high impacts for variable 2
    impact_chirho.record_impact_chirho(2, 0.9);
    impact_chirho.record_impact_chirho(2, 0.85);
    impact_chirho.record_impact_chirho(2, 0.88);

    let selected_ib_chirho = impact_chirho.select_variable_chirho(&domain_sizes_chirho, &assigned_chirho);
    println!(
        "Impact-Based selects: variable {} (impact={:.2})",
        selected_ib_chirho.unwrap(),
        impact_chirho.impact_chirho(selected_ib_chirho.unwrap())
    );
}

fn main_chirho() {
    // Demonstrate heuristics
    demonstrate_heuristics_chirho();

    println!("\n=== N-Queens Problem ===\n");

    // Solve 8-Queens
    println!("Solving 8-Queens...");
    match solve_n_queens_chirho(8) {
        Some(solution_chirho) => {
            println!("Solution found!\n");
            print_queens_chirho(&solution_chirho);
        }
        None => {
            println!("No solution found.");
        }
    }

    println!();

    // Count solutions for smaller boards
    for n_chirho in 4..=8 {
        let ambs_chirho: Vec<AmbChirho> = (0..n_chirho)
            .map(|_| AmbChirho::range_chirho(0, (n_chirho - 1) as i32))
            .collect();

        let search_chirho = BacktrackingSearchChirho::with_limit_chirho(1_000_000);
        let result_chirho = search_chirho.search_simple_chirho(&ambs_chirho, |values_chirho| {
            for i_chirho in 0..values_chirho.len() {
                let row_i_chirho = values_chirho[i_chirho]
                    .as_interval_chirho()
                    .map(|iv_chirho| iv_chirho.lo_chirho as i32)
                    .unwrap_or(-1);

                for j_chirho in (i_chirho + 1)..values_chirho.len() {
                    let row_j_chirho = values_chirho[j_chirho]
                        .as_interval_chirho()
                        .map(|iv_chirho| iv_chirho.lo_chirho as i32)
                        .unwrap_or(-1);

                    if row_i_chirho < 0 || row_j_chirho < 0 {
                        continue;
                    }

                    if row_i_chirho == row_j_chirho {
                        return false;
                    }

                    let col_diff_chirho = (j_chirho - i_chirho) as i32;
                    let row_diff_chirho = (row_j_chirho - row_i_chirho).abs();
                    if col_diff_chirho == row_diff_chirho {
                        return false;
                    }
                }
            }
            true
        });

        let has_solution_chirho = matches!(result_chirho, SearchResultChirho::SolutionChirho(_));
        let backtracks_chirho = search_chirho.backtrack_count_chirho();
        println!(
            "{}-Queens: {} (backtracks: {})",
            n_chirho,
            if has_solution_chirho { "Found" } else { "None" },
            backtracks_chirho
        );
    }

    println!("\nDone!");
}

fn main() {
    main_chirho();
}
