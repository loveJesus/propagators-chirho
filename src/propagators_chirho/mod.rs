// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Propagator implementations for constraint propagation.
//!
//! This module contains all propagator types:
//! - Arithmetic propagators (Adder, Subtractor, Multiplier, Divider)
//! - Mathematical functions (Sqrt, Square, Abs, Exp, Ln, Power, Clamp)
//! - Control propagators (Conditional, Constant, Max, Min, Negater)

pub mod propagator_chirho;

// Re-exports for convenience
pub use propagator_chirho::{
    AbsoluterChirho, ClampChirho, ConditionalChirho, ConstantChirho, ExpChirho,
    IntervalAdderChirho, IntervalDividerChirho, IntervalMultiplierChirho, IntervalSubtractorChirho,
    LnChirho, MaxChirho, MinChirho, NegaterChirho, PowerChirho, PropagatorChirho, SqrterChirho,
    SquarerChirho,
};
