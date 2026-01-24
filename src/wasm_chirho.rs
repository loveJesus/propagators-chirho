// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! WebAssembly bindings for propagator networks.
//!
//! This module provides JavaScript-friendly bindings for using propagator
//! networks in the browser or Node.js.
//!
//! # Feature Flag
//!
//! This module requires the `wasm` feature:
//!
//! ```toml
//! [dependencies]
//! propagators-chirho = { version = "0.1", features = ["wasm"] }
//! ```
//!
//! # Building for WASM
//!
//! ```bash
//! wasm-pack build --target web --features wasm
//! ```
//!
//! # JavaScript Usage
//!
//! ```javascript
//! import init, { WasmNetworkChirho } from 'propagators-chirho';
//!
//! await init();
//!
//! const networkChirho = new WasmNetworkChirho();
//! const aChirho = networkChirho.makeCellChirho();
//! const bChirho = networkChirho.makeCellChirho();
//! const cChirho = networkChirho.makeCellChirho();
//!
//! networkChirho.addAdderChirho(aChirho, bChirho, cChirho);
//! networkChirho.setExactChirho(aChirho, 3.0);
//! networkChirho.setExactChirho(bChirho, 4.0);
//! networkChirho.propagateChirho();
//!
//! console.log(networkChirho.getExactChirho(cChirho)); // 7.0
//! ```

use wasm_bindgen::prelude::*;

use crate::arena_chirho::{ArenaNetworkChirho, CellIdChirho};
use crate::interval_chirho::NumericInfoChirho;

/// A WebAssembly-compatible propagator network.
///
/// This wraps the arena-based network for efficient WASM execution.
#[wasm_bindgen]
pub struct WasmNetworkChirho {
    network_chirho: ArenaNetworkChirho,
}

#[wasm_bindgen]
impl WasmNetworkChirho {
    /// Creates a new empty network.
    #[wasm_bindgen(constructor)]
    pub fn new_chirho() -> Self {
        Self {
            network_chirho: ArenaNetworkChirho::new_chirho(),
        }
    }

    /// Creates a new cell and returns its ID.
    #[wasm_bindgen(js_name = "makeCellChirho")]
    pub fn make_cell_chirho(&mut self) -> usize {
        self.network_chirho.make_cell_chirho().index_chirho()
    }

    /// Sets a cell to an exact value.
    #[wasm_bindgen(js_name = "setExactChirho")]
    pub fn set_exact_chirho(&mut self, cell_id_chirho: usize, value_chirho: f64) {
        let cell_chirho = CellIdChirho::from_index_chirho(cell_id_chirho);
        self.network_chirho
            .set_exact_chirho(cell_chirho, value_chirho);
    }

    /// Sets a cell to an interval.
    #[wasm_bindgen(js_name = "setIntervalChirho")]
    pub fn set_interval_chirho(&mut self, cell_id_chirho: usize, lo_chirho: f64, hi_chirho: f64) {
        let cell_chirho = CellIdChirho::from_index_chirho(cell_id_chirho);
        self.network_chirho
            .set_interval_chirho(cell_chirho, lo_chirho, hi_chirho);
    }

    /// Adds an addition constraint: a + b = c
    #[wasm_bindgen(js_name = "addAdderChirho")]
    pub fn add_adder_chirho(&mut self, a_chirho: usize, b_chirho: usize, c_chirho: usize) {
        let a_cell_chirho = CellIdChirho::from_index_chirho(a_chirho);
        let b_cell_chirho = CellIdChirho::from_index_chirho(b_chirho);
        let c_cell_chirho = CellIdChirho::from_index_chirho(c_chirho);
        self.network_chirho
            .add_adder_chirho(a_cell_chirho, b_cell_chirho, c_cell_chirho);
    }

    /// Adds a multiplication constraint: a * b = c
    #[wasm_bindgen(js_name = "addMultiplierChirho")]
    pub fn add_multiplier_chirho(&mut self, a_chirho: usize, b_chirho: usize, c_chirho: usize) {
        let a_cell_chirho = CellIdChirho::from_index_chirho(a_chirho);
        let b_cell_chirho = CellIdChirho::from_index_chirho(b_chirho);
        let c_cell_chirho = CellIdChirho::from_index_chirho(c_chirho);
        self.network_chirho
            .add_multiplier_chirho(a_cell_chirho, b_cell_chirho, c_cell_chirho);
    }

    /// Adds a square constraint: a² = b
    #[wasm_bindgen(js_name = "addSquarerChirho")]
    pub fn add_squarer_chirho(&mut self, a_chirho: usize, b_chirho: usize) {
        let a_cell_chirho = CellIdChirho::from_index_chirho(a_chirho);
        let b_cell_chirho = CellIdChirho::from_index_chirho(b_chirho);
        self.network_chirho
            .add_squarer_chirho(a_cell_chirho, b_cell_chirho);
    }

    /// Runs propagation until fixpoint.
    #[wasm_bindgen(js_name = "propagateChirho")]
    pub fn propagate_chirho(&mut self) {
        self.network_chirho.propagate_chirho();
    }

    /// Gets the low bound of a cell's interval.
    #[wasm_bindgen(js_name = "getLoChirho")]
    pub fn get_lo_chirho(&self, cell_id_chirho: usize) -> Option<f64> {
        let cell_chirho = CellIdChirho::from_index_chirho(cell_id_chirho);
        match self.network_chirho.get_content_chirho(cell_chirho) {
            NumericInfoChirho::IntervalChirho(iv_chirho) => Some(iv_chirho.lo_chirho),
            _ => None,
        }
    }

    /// Gets the high bound of a cell's interval.
    #[wasm_bindgen(js_name = "getHiChirho")]
    pub fn get_hi_chirho(&self, cell_id_chirho: usize) -> Option<f64> {
        let cell_chirho = CellIdChirho::from_index_chirho(cell_id_chirho);
        match self.network_chirho.get_content_chirho(cell_chirho) {
            NumericInfoChirho::IntervalChirho(iv_chirho) => Some(iv_chirho.hi_chirho),
            _ => None,
        }
    }

    /// Gets the exact value if the cell is exact (lo == hi).
    #[wasm_bindgen(js_name = "getExactChirho")]
    pub fn get_exact_chirho(&self, cell_id_chirho: usize) -> Option<f64> {
        let cell_chirho = CellIdChirho::from_index_chirho(cell_id_chirho);
        self.network_chirho.get_exact_chirho(cell_chirho)
    }

    /// Returns true if the cell is in contradiction.
    #[wasm_bindgen(js_name = "isContradictionChirho")]
    pub fn is_contradiction_chirho(&self, cell_id_chirho: usize) -> bool {
        let cell_chirho = CellIdChirho::from_index_chirho(cell_id_chirho);
        matches!(
            self.network_chirho.get_content_chirho(cell_chirho),
            NumericInfoChirho::ContradictionChirho
        )
    }

    /// Returns true if any cell has a contradiction.
    #[wasm_bindgen(js_name = "hasContradictionChirho")]
    pub fn has_contradiction_chirho(&self) -> bool {
        self.network_chirho.has_contradiction_chirho()
    }

    /// Returns the number of cells.
    #[wasm_bindgen(js_name = "cellCountChirho")]
    pub fn cell_count_chirho(&self) -> usize {
        self.network_chirho.cell_count_chirho()
    }

    /// Returns the number of propagators.
    #[wasm_bindgen(js_name = "propagatorCountChirho")]
    pub fn propagator_count_chirho(&self) -> usize {
        self.network_chirho.propagator_count_chirho()
    }

    /// Returns the number of propagation steps.
    #[wasm_bindgen(js_name = "propagationCountChirho")]
    pub fn propagation_count_chirho(&self) -> usize {
        self.network_chirho.propagation_count_chirho()
    }
}

/// A standalone interval for use in JavaScript.
#[wasm_bindgen]
pub struct WasmIntervalChirho {
    lo_chirho: f64,
    hi_chirho: f64,
}

#[wasm_bindgen]
impl WasmIntervalChirho {
    /// Creates a new interval.
    #[wasm_bindgen(constructor)]
    pub fn new_chirho(lo_chirho: f64, hi_chirho: f64) -> Self {
        Self {
            lo_chirho,
            hi_chirho,
        }
    }

    /// Creates an exact value interval.
    #[wasm_bindgen(js_name = "exactChirho")]
    pub fn exact_chirho(value_chirho: f64) -> Self {
        Self {
            lo_chirho: value_chirho,
            hi_chirho: value_chirho,
        }
    }

    /// Gets the low bound.
    #[wasm_bindgen(getter)]
    pub fn lo_chirho(&self) -> f64 {
        self.lo_chirho
    }

    /// Gets the high bound.
    #[wasm_bindgen(getter)]
    pub fn hi_chirho(&self) -> f64 {
        self.hi_chirho
    }

    /// Returns the width (hi - lo).
    #[wasm_bindgen(js_name = "widthChirho")]
    pub fn width_chirho(&self) -> f64 {
        self.hi_chirho - self.lo_chirho
    }

    /// Returns true if this is an exact value.
    #[wasm_bindgen(js_name = "isExactChirho")]
    pub fn is_exact_chirho(&self) -> bool {
        (self.hi_chirho - self.lo_chirho).abs() < 1e-10
    }

    /// Adds two intervals.
    #[wasm_bindgen(js_name = "addChirho")]
    pub fn add_chirho(&self, other_chirho: &WasmIntervalChirho) -> WasmIntervalChirho {
        WasmIntervalChirho {
            lo_chirho: self.lo_chirho + other_chirho.lo_chirho,
            hi_chirho: self.hi_chirho + other_chirho.hi_chirho,
        }
    }

    /// Subtracts two intervals.
    #[wasm_bindgen(js_name = "subChirho")]
    pub fn sub_chirho(&self, other_chirho: &WasmIntervalChirho) -> WasmIntervalChirho {
        WasmIntervalChirho {
            lo_chirho: self.lo_chirho - other_chirho.hi_chirho,
            hi_chirho: self.hi_chirho - other_chirho.lo_chirho,
        }
    }

    /// Multiplies two intervals.
    #[wasm_bindgen(js_name = "mulChirho")]
    pub fn mul_chirho(&self, other_chirho: &WasmIntervalChirho) -> WasmIntervalChirho {
        let products_chirho = [
            self.lo_chirho * other_chirho.lo_chirho,
            self.lo_chirho * other_chirho.hi_chirho,
            self.hi_chirho * other_chirho.lo_chirho,
            self.hi_chirho * other_chirho.hi_chirho,
        ];

        let lo_chirho = products_chirho
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min);
        let hi_chirho = products_chirho
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        WasmIntervalChirho {
            lo_chirho,
            hi_chirho,
        }
    }

    /// Intersects two intervals.
    #[wasm_bindgen(js_name = "intersectChirho")]
    pub fn intersect_chirho(&self, other_chirho: &WasmIntervalChirho) -> WasmIntervalChirho {
        WasmIntervalChirho {
            lo_chirho: self.lo_chirho.max(other_chirho.lo_chirho),
            hi_chirho: self.hi_chirho.min(other_chirho.hi_chirho),
        }
    }

    /// Returns true if the interval is empty (contradiction).
    #[wasm_bindgen(js_name = "isEmptyChirho")]
    pub fn is_empty_chirho(&self) -> bool {
        self.lo_chirho > self.hi_chirho
    }

    /// Returns true if the interval contains a value.
    #[wasm_bindgen(js_name = "containsChirho")]
    pub fn contains_chirho(&self, value_chirho: f64) -> bool {
        self.lo_chirho <= value_chirho && value_chirho <= self.hi_chirho
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_wasm_network_chirho() {
        let mut network_chirho = WasmNetworkChirho::new_chirho();

        let a_chirho = network_chirho.make_cell_chirho();
        let b_chirho = network_chirho.make_cell_chirho();
        let c_chirho = network_chirho.make_cell_chirho();

        network_chirho.add_adder_chirho(a_chirho, b_chirho, c_chirho);
        network_chirho.set_exact_chirho(a_chirho, 3.0);
        network_chirho.set_exact_chirho(b_chirho, 4.0);
        network_chirho.propagate_chirho();

        assert_eq!(network_chirho.get_exact_chirho(c_chirho), Some(7.0));
    }

    #[test]
    fn test_wasm_interval_chirho() {
        let a_chirho = WasmIntervalChirho::new_chirho(1.0, 5.0);
        let b_chirho = WasmIntervalChirho::new_chirho(2.0, 4.0);

        let sum_chirho = a_chirho.add_chirho(&b_chirho);
        assert_eq!(sum_chirho.lo_chirho, 3.0);
        assert_eq!(sum_chirho.hi_chirho, 9.0);

        let intersection_chirho = a_chirho.intersect_chirho(&b_chirho);
        assert_eq!(intersection_chirho.lo_chirho, 2.0);
        assert_eq!(intersection_chirho.hi_chirho, 4.0);
    }
}
