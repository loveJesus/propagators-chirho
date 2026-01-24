// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Electrical circuit analysis using propagator networks.
//!
//! This example demonstrates Ohm's Law: V = I × R
//! and power: P = V × I
//!
//! Run with: `cargo run --example electrical_chirho`

use propagators_chirho::prelude_chirho::*;

fn main_chirho() {
    println!("=== Electrical Circuit Analysis ===\n");
    println!("Constraints:");
    println!("  Ohm's Law: V = I × R");
    println!("  Power:     P = V × I");
    println!();

    // Example 1: Given V and R, find I and P
    {
        println!("--- Example 1: LED Circuit ---");
        println!("Given: V = 5V, R = 220Ω");
        println!("Find:  I = ?, P = ?");

        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let v_chirho = system_chirho.make_cell_chirho("voltage");
        let i_chirho = system_chirho.make_cell_chirho("current");
        let r_chirho = system_chirho.make_cell_chirho("resistance");
        let p_chirho = system_chirho.make_cell_chirho("power");

        // V = I × R
        system_chirho.add_multiplier_chirho(&i_chirho, &r_chirho, &v_chirho);
        // P = V × I
        system_chirho.add_multiplier_chirho(&v_chirho, &i_chirho, &p_chirho);

        system_chirho.set_exact_chirho(&v_chirho, 5.0); // 5V
        system_chirho.set_exact_chirho(&r_chirho, 220.0); // 220Ω
        system_chirho.run_chirho();

        println!("Result: I = {} A", system_chirho.get_chirho(&i_chirho));
        println!("        P = {} W", system_chirho.get_chirho(&p_chirho));
        println!("        (≈ 22.7mA, ≈ 0.11W)");
        println!();
    }

    // Example 2: Given P and R, find V and I (backward!)
    {
        println!("--- Example 2: Power Dissipation ---");
        println!("Given: P = 1W (max power for resistor), R = 100Ω");
        println!("Find:  V = ?, I = ? (max safe values)");

        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let v_chirho = system_chirho.make_cell_chirho("voltage");
        let i_chirho = system_chirho.make_cell_chirho("current");
        let r_chirho = system_chirho.make_cell_chirho("resistance");
        let p_chirho = system_chirho.make_cell_chirho("power");

        // V = I × R
        system_chirho.add_multiplier_chirho(&i_chirho, &r_chirho, &v_chirho);
        // P = V × I
        system_chirho.add_multiplier_chirho(&v_chirho, &i_chirho, &p_chirho);

        // Constrain to positive values
        system_chirho.set_interval_chirho(&v_chirho, 0.0, f64::INFINITY);
        system_chirho.set_interval_chirho(&i_chirho, 0.0, f64::INFINITY);

        system_chirho.set_exact_chirho(&p_chirho, 1.0); // 1W max
        system_chirho.set_exact_chirho(&r_chirho, 100.0); // 100Ω
        system_chirho.run_chirho();

        println!("Result: V = {} V", system_chirho.get_chirho(&v_chirho));
        println!("        I = {} A", system_chirho.get_chirho(&i_chirho));
        println!("        (≈ 10V, ≈ 100mA)");
        println!();
    }

    // Example 3: Voltage divider
    {
        println!("--- Example 3: Voltage Divider ---");
        println!("Configuration: Vin → R1 → Vout → R2 → GND");
        println!("Formula: Vout = Vin × R2 / (R1 + R2)");
        println!("Given: Vin = 12V, R1 = 10kΩ, R2 = 10kΩ");
        println!("Find:  Vout = ?");

        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let vin_chirho = system_chirho.make_cell_chirho("vin");
        let vout_chirho = system_chirho.make_cell_chirho("vout");
        let r1_chirho = system_chirho.make_cell_chirho("r1");
        let r2_chirho = system_chirho.make_cell_chirho("r2");
        let r_total_chirho = system_chirho.make_cell_chirho("r_total");
        let ratio_chirho = system_chirho.make_cell_chirho("ratio");

        // R_total = R1 + R2
        system_chirho.add_adder_chirho(&r1_chirho, &r2_chirho, &r_total_chirho);
        // ratio = R2 / R_total
        system_chirho.add_divider_chirho(&r2_chirho, &r_total_chirho, &ratio_chirho);
        // Vout = Vin × ratio
        system_chirho.add_multiplier_chirho(&vin_chirho, &ratio_chirho, &vout_chirho);

        system_chirho.set_exact_chirho(&vin_chirho, 12.0); // 12V
        system_chirho.set_exact_chirho(&r1_chirho, 10000.0); // 10kΩ
        system_chirho.set_exact_chirho(&r2_chirho, 10000.0); // 10kΩ
        system_chirho.run_chirho();

        println!(
            "Result: Vout = {} V",
            system_chirho.get_chirho(&vout_chirho)
        );
        println!("        (Expected: 6V - half of 12V)");
        println!();
    }

    // Example 4: Component tolerance intervals
    {
        println!("--- Example 4: Resistor Tolerance ---");
        println!("Given: V = 5V, R = 100Ω ± 5% = [95, 105]Ω");
        println!("Find:  I = ? (range due to tolerance)");

        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let v_chirho = system_chirho.make_cell_chirho("voltage");
        let i_chirho = system_chirho.make_cell_chirho("current");
        let r_chirho = system_chirho.make_cell_chirho("resistance");

        system_chirho.add_multiplier_chirho(&i_chirho, &r_chirho, &v_chirho);

        system_chirho.set_exact_chirho(&v_chirho, 5.0);
        system_chirho.set_interval_chirho(&r_chirho, 95.0, 105.0); // 100Ω ± 5%
        system_chirho.run_chirho();

        println!("Result: I = {} A", system_chirho.get_chirho(&i_chirho));
        println!("        (≈ 47.6mA to 52.6mA)");
        println!();
    }

    println!("✓ Propagators enable bidirectional circuit analysis!");
    println!("✓ Intervals capture component tolerances!");
}

fn main() {
    main_chirho();
}
