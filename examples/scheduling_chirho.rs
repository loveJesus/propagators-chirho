// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Job Scheduling Example
//!
//! This example demonstrates using finite domain propagators for
//! job shop scheduling problems.
//!
//! Problem: Schedule 3 jobs on 2 machines with constraints:
//! - Each job has a duration
//! - Jobs on the same machine cannot overlap
//! - Some jobs depend on others (must complete first)

use propagators_chirho::finite_domain_chirho::{FiniteDomainChirho, LessThanChirho, NotEqualsChirho};
use propagators_chirho::generic_cell_chirho::GenericNetworkChirho;
use propagators_chirho::lattice_chirho::BoundedLatticeChirho;

fn main_chirho() {
    println!("Job Shop Scheduling with Propagators\n");
    println!("====================================\n");

    // Time slots: 0-9 (each slot is 1 hour)
    let _time_slots_chirho = 10;

    // Create a network for finite domain (discrete) values
    let mut network_chirho: GenericNetworkChirho<FiniteDomainChirho> =
        GenericNetworkChirho::new_chirho();

    // Jobs: each represented by their start time
    // Job A: duration 2, on machine 1
    // Job B: duration 3, on machine 1
    // Job C: duration 2, on machine 2

    let job_a_start_chirho = network_chirho.make_named_cell_chirho("job_a_start");
    let job_b_start_chirho = network_chirho.make_named_cell_chirho("job_b_start");
    let job_c_start_chirho = network_chirho.make_named_cell_chirho("job_c_start");

    // Initialize domains: jobs can start at any time that allows completion
    // Job A (duration 2) can start 0-8 to finish by slot 9
    // Job B (duration 3) can start 0-7 to finish by slot 9
    // Job C (duration 2) can start 0-8 to finish by slot 9
    network_chirho.set_cell_chirho(job_a_start_chirho, FiniteDomainChirho::range_chirho(0, 8));
    network_chirho.set_cell_chirho(job_b_start_chirho, FiniteDomainChirho::range_chirho(0, 7));
    network_chirho.set_cell_chirho(job_c_start_chirho, FiniteDomainChirho::range_chirho(0, 8));

    println!("Initial domains:");
    print_domain_chirho(&network_chirho, job_a_start_chirho, "Job A");
    print_domain_chirho(&network_chirho, job_b_start_chirho, "Job B");
    print_domain_chirho(&network_chirho, job_c_start_chirho, "Job C");

    // Constraint 1: Job A must finish before Job B starts
    // A.start + 2 <= B.start, i.e., A.start < B.start - 1
    // We'll represent this as: A.start must be at least 2 less than B.start
    println!("\nAdding constraint: Job A must complete before Job B starts...");

    // For simplicity, we'll add a "less than" constraint
    // A < B (A's domain values must be less than B's minimum)
    network_chirho.add_propagator_chirho(LessThanChirho, vec![job_a_start_chirho, job_b_start_chirho]);

    network_chirho.propagate_chirho();

    println!("\nAfter precedence constraint (A before B):");
    print_domain_chirho(&network_chirho, job_a_start_chirho, "Job A");
    print_domain_chirho(&network_chirho, job_b_start_chirho, "Job B");

    // Constraint 2: Jobs A and B are on the same machine - they can't have the same start time
    // (In reality, we'd need to check for overlap, but != is a simple approximation)
    println!("\nAdding constraint: Jobs A and B cannot have same start time...");
    network_chirho.add_propagator_chirho(NotEqualsChirho, vec![job_a_start_chirho, job_b_start_chirho]);

    network_chirho.propagate_chirho();

    println!("\nAfter non-overlap constraint:");
    print_domain_chirho(&network_chirho, job_a_start_chirho, "Job A");
    print_domain_chirho(&network_chirho, job_b_start_chirho, "Job B");

    // Constraint 3: Fix Job A to start at time 0
    println!("\nFixing Job A to start at time 0...");
    network_chirho.set_cell_chirho(job_a_start_chirho, FiniteDomainChirho::singleton_chirho(0));

    network_chirho.propagate_chirho();

    println!("\nFinal schedule:");
    print_domain_chirho(&network_chirho, job_a_start_chirho, "Job A");
    print_domain_chirho(&network_chirho, job_b_start_chirho, "Job B");
    print_domain_chirho(&network_chirho, job_c_start_chirho, "Job C");

    // Check for contradictions
    if network_chirho.has_contradiction_chirho() {
        println!("\n⚠ No valid schedule exists!");
    } else {
        println!("\n✓ Valid schedule found!");
    }

    println!("\nPropagation steps: {}", network_chirho.propagation_count_chirho());
}

fn print_domain_chirho(
    network_chirho: &GenericNetworkChirho<FiniteDomainChirho>,
    cell_idx_chirho: usize,
    name_chirho: &str,
) {
    if let Some(cell_chirho) = network_chirho.get_cell_chirho(cell_idx_chirho) {
        let domain_chirho = cell_chirho.get_chirho();
        if domain_chirho.is_top_chirho() {
            println!("  {}: CONTRADICTION", name_chirho);
        } else if domain_chirho.is_bottom_chirho() {
            println!("  {}: (no constraint)", name_chirho);
        } else {
            let values_chirho: Vec<i64> = domain_chirho.iter_chirho().collect();
            if values_chirho.len() == 1 {
                println!("  {}: fixed at time {}", name_chirho, values_chirho[0]);
            } else {
                println!("  {}: possible times {:?}", name_chirho, values_chirho);
            }
        }
    }
}

fn main() {
    main_chirho();
}
