// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Temperature conversion example demonstrating bidirectional constraints.
//!
//! This example shows how propagators can run "backwards" - given Fahrenheit,
//! compute Celsius, or vice versa, using the same constraint network.
//!
//! Run with: `cargo run --example temperature_chirho`

use propagators_chirho::prelude_chirho::*;

fn main_chirho() {
    println!("=== Bidirectional Temperature Conversion ===\n");
    println!("Constraint: F = C × 9/5 + 32\n");

    // Create the constraint system
    let mut system_chirho = ConstraintSystemChirho::new_chirho();

    // Create cells
    let celsius_chirho = system_chirho.make_cell_chirho("celsius");
    let fahrenheit_chirho = system_chirho.make_cell_chirho("fahrenheit");
    let nine_fifths_chirho = system_chirho.make_cell_chirho("nine_fifths");
    let thirty_two_chirho = system_chirho.make_cell_chirho("thirty_two");
    let product_chirho = system_chirho.make_cell_chirho("product");

    // Set constants
    system_chirho.set_exact_chirho(&nine_fifths_chirho, 1.8);
    system_chirho.set_exact_chirho(&thirty_two_chirho, 32.0);

    // Set up constraints: F = C * 9/5 + 32
    system_chirho.add_multiplier_chirho(&celsius_chirho, &nine_fifths_chirho, &product_chirho);
    system_chirho.add_adder_chirho(&product_chirho, &thirty_two_chirho, &fahrenheit_chirho);

    // === Forward: Celsius → Fahrenheit ===
    println!("--- Forward Propagation ---");
    println!("Setting: Celsius = 100°C (boiling point of water)");

    system_chirho.set_exact_chirho(&celsius_chirho, 100.0);
    system_chirho.run_chirho();

    let f_value_chirho = system_chirho.get_chirho(&fahrenheit_chirho);
    println!("Result:  Fahrenheit = {}", f_value_chirho);
    println!();

    // === New system for backward propagation ===
    let mut system2_chirho = ConstraintSystemChirho::new_chirho();

    let celsius2_chirho = system2_chirho.make_cell_chirho("celsius");
    let fahrenheit2_chirho = system2_chirho.make_cell_chirho("fahrenheit");
    let nine_fifths2_chirho = system2_chirho.make_cell_chirho("nine_fifths");
    let thirty_two2_chirho = system2_chirho.make_cell_chirho("thirty_two");
    let product2_chirho = system2_chirho.make_cell_chirho("product");

    system2_chirho.set_exact_chirho(&nine_fifths2_chirho, 1.8);
    system2_chirho.set_exact_chirho(&thirty_two2_chirho, 32.0);

    system2_chirho.add_multiplier_chirho(&celsius2_chirho, &nine_fifths2_chirho, &product2_chirho);
    system2_chirho.add_adder_chirho(&product2_chirho, &thirty_two2_chirho, &fahrenheit2_chirho);

    // === Backward: Fahrenheit → Celsius ===
    println!("--- Backward Propagation ---");
    println!("Setting: Fahrenheit = 32°F (freezing point of water)");

    system2_chirho.set_exact_chirho(&fahrenheit2_chirho, 32.0);
    system2_chirho.run_chirho();

    let c_value_chirho = system2_chirho.get_chirho(&celsius2_chirho);
    println!("Result:  Celsius = {}", c_value_chirho);
    println!();

    // === Interval propagation ===
    let mut system3_chirho = ConstraintSystemChirho::new_chirho();

    let celsius3_chirho = system3_chirho.make_cell_chirho("celsius");
    let fahrenheit3_chirho = system3_chirho.make_cell_chirho("fahrenheit");
    let nine_fifths3_chirho = system3_chirho.make_cell_chirho("nine_fifths");
    let thirty_two3_chirho = system3_chirho.make_cell_chirho("thirty_two");
    let product3_chirho = system3_chirho.make_cell_chirho("product");

    system3_chirho.set_exact_chirho(&nine_fifths3_chirho, 1.8);
    system3_chirho.set_exact_chirho(&thirty_two3_chirho, 32.0);

    system3_chirho.add_multiplier_chirho(&celsius3_chirho, &nine_fifths3_chirho, &product3_chirho);
    system3_chirho.add_adder_chirho(&product3_chirho, &thirty_two3_chirho, &fahrenheit3_chirho);

    println!("--- Interval Propagation ---");
    println!("Setting: Celsius ∈ [20, 25] (comfortable room temperature)");

    system3_chirho.set_interval_chirho(&celsius3_chirho, 20.0, 25.0);
    system3_chirho.run_chirho();

    let f_interval_chirho = system3_chirho.get_chirho(&fahrenheit3_chirho);
    println!("Result:  Fahrenheit = {}", f_interval_chirho);
    println!();

    println!("✓ Propagators work in both directions!");
    println!("✓ Interval arithmetic preserves uncertainty!");
}

fn main() {
    main_chirho();
}
