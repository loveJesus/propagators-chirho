// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Language bindings for propagators.
//!
//! This module contains:
//! - Python bindings via PyO3
//! - WebAssembly bindings

#[cfg(all(feature = "python", not(feature = "no-std")))]
pub mod python_chirho;

#[cfg(all(feature = "wasm", feature = "arena", not(feature = "no-std")))]
pub mod wasm_chirho;
