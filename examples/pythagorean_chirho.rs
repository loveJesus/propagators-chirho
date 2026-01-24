// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Pythagorean theorem example: a² + b² = c²
//!
//! Demonstrates running the constraint "backwards" to find missing sides.
//!
//! Run with: `cargo run --example pythagorean_chirho`

use propagators_chirho::prelude_chirho::*;

fn main_chirho() {
    println!("=== Pythagorean Theorem: a² + b² = c² ===\n");

    // Example 1: Given a and b, find c
    {
        println!("--- Example 1: Forward ---");
        println!("Given: a = 3, b = 4");
        println!("Find:  c = ?");

        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");
        let c_chirho = system_chirho.make_cell_chirho("c");

        system_chirho.add_pythagorean_chirho(&a_chirho, &b_chirho, &c_chirho);

        // Constrain c to be positive
        system_chirho.set_interval_chirho(&c_chirho, 0.0, f64::INFINITY);

        system_chirho.set_exact_chirho(&a_chirho, 3.0);
        system_chirho.set_exact_chirho(&b_chirho, 4.0);
        system_chirho.run_chirho();

        println!("Result: c = {}", system_chirho.get_chirho(&c_chirho));
        println!();
    }

    // Example 2: Given a and c, find b (backward!)
    {
        println!("--- Example 2: Backward ---");
        println!("Given: a = 5, c = 13");
        println!("Find:  b = ?");

        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");
        let c_chirho = system_chirho.make_cell_chirho("c");

        system_chirho.add_pythagorean_chirho(&a_chirho, &b_chirho, &c_chirho);

        // Constrain b to be positive
        system_chirho.set_interval_chirho(&b_chirho, 0.0, f64::INFINITY);

        system_chirho.set_exact_chirho(&a_chirho, 5.0);
        system_chirho.set_exact_chirho(&c_chirho, 13.0);
        system_chirho.run_chirho();

        println!("Result: b = {}", system_chirho.get_chirho(&b_chirho));
        println!("(Expected: 12, since 5² + 12² = 13²)");
        println!();
    }

    // Example 3: Interval arithmetic
    {
        println!("--- Example 3: Intervals ---");
        println!("Given: a ∈ [3, 4], b ∈ [4, 5]");
        println!("Find:  c = ?");

        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");
        let c_chirho = system_chirho.make_cell_chirho("c");

        system_chirho.add_pythagorean_chirho(&a_chirho, &b_chirho, &c_chirho);
        system_chirho.set_interval_chirho(&c_chirho, 0.0, f64::INFINITY);

        system_chirho.set_interval_chirho(&a_chirho, 3.0, 4.0);
        system_chirho.set_interval_chirho(&b_chirho, 4.0, 5.0);
        system_chirho.run_chirho();

        println!("Result: c = {}", system_chirho.get_chirho(&c_chirho));
        println!("(c² ∈ [25, 41], so c ∈ [5, 6.4])");
        println!();
    }

    println!("✓ Constraints work bidirectionally!");
}

fn main() {
    main_chirho();
}
