#!/bin/bash
# For God so loved the world that he gave his only begotten Son,
#     that whoever believes in him should not perish but have eternal life.
#     John 3:16

# Test script for propagators-chirho
# Tests various feature combinations to ensure compatibility

set -e

echo "=============================================="
echo "Propagators Chirho - Feature Combination Tests"
echo "=============================================="

# Colors for output
RED_CHIRHO='\033[0;31m'
GREEN_CHIRHO='\033[0;32m'
YELLOW_CHIRHO='\033[1;33m'
NC_CHIRHO='\033[0m' # No Color

pass_chirho() {
    echo -e "${GREEN_CHIRHO}✓ PASS:${NC_CHIRHO} $1"
}

fail_chirho() {
    echo -e "${RED_CHIRHO}✗ FAIL:${NC_CHIRHO} $1"
    exit 1
}

info_chirho() {
    echo -e "${YELLOW_CHIRHO}→${NC_CHIRHO} $1"
}

# Track test results
TESTS_RUN_CHIRHO=0
TESTS_PASSED_CHIRHO=0

run_test_chirho() {
    local name_chirho="$1"
    local cmd_chirho="$2"
    TESTS_RUN_CHIRHO=$((TESTS_RUN_CHIRHO + 1))
    info_chirho "Testing: $name_chirho"
    if eval "$cmd_chirho" > /dev/null 2>&1; then
        pass_chirho "$name_chirho"
        TESTS_PASSED_CHIRHO=$((TESTS_PASSED_CHIRHO + 1))
    else
        fail_chirho "$name_chirho"
    fi
}

echo ""
echo "1. Default features (no features enabled)"
echo "-------------------------------------------"
run_test_chirho "cargo check (default)" "cargo check --lib"
run_test_chirho "cargo test (default)" "cargo test"

echo ""
echo "2. Individual features"
echo "----------------------"
run_test_chirho "arena feature" "cargo check --lib --features arena"
run_test_chirho "parallel feature" "cargo check --lib --features parallel"
run_test_chirho "serde feature" "cargo check --lib --features serde"
run_test_chirho "tracing feature" "cargo check --lib --features tracing"
run_test_chirho "tms-full feature" "cargo check --lib --features tms-full"
run_test_chirho "backtrack feature" "cargo check --lib --features backtrack"
run_test_chirho "network feature" "cargo check --lib --features network"
run_test_chirho "kani feature" "cargo check --lib --features kani"

echo ""
echo "3. no-std feature (embedded target)"
echo "------------------------------------"
run_test_chirho "no-std (thumbv7m-none-eabi)" "cargo check --lib --features no-std --target thumbv7m-none-eabi"

echo ""
echo "4. Feature combinations"
echo "-----------------------"
run_test_chirho "arena + parallel" "cargo check --lib --features 'arena,parallel'"
run_test_chirho "arena + serde" "cargo check --lib --features 'arena,serde'"
run_test_chirho "arena + network" "cargo check --lib --features 'arena,network'"
run_test_chirho "parallel + network" "cargo check --lib --features 'parallel,network'"
run_test_chirho "tms-full + backtrack" "cargo check --lib --features 'tms-full,backtrack'"
run_test_chirho "arena + parallel + serde + tracing" "cargo check --lib --features 'arena,parallel,serde,tracing'"

echo ""
echo "5. All std features combined"
echo "----------------------------"
run_test_chirho "all std features (check)" "cargo check --lib --features 'arena,parallel,serde,tracing,tms-full,backtrack,network'"
run_test_chirho "all std features (test)" "cargo test --features 'arena,parallel,serde,tracing,tms-full,backtrack,network'"

echo ""
echo "6. Examples"
echo "-----------"
run_test_chirho "temperature example" "cargo build --example temperature_chirho"
run_test_chirho "pythagorean example" "cargo build --example pythagorean_chirho"
run_test_chirho "sudoku example" "cargo build --example sudoku_chirho"
run_test_chirho "electrical example" "cargo build --example electrical_chirho"
run_test_chirho "scheduling example" "cargo build --example scheduling_chirho --features arena"

echo ""
echo "7. Documentation"
echo "----------------"
run_test_chirho "cargo doc" "cargo doc --no-deps"
run_test_chirho "cargo doc (all features)" "cargo doc --no-deps --features 'arena,parallel,serde,tracing,tms-full,backtrack,network'"

echo ""
echo "8. Clippy lints"
echo "---------------"
run_test_chirho "clippy (default)" "cargo clippy -- -D warnings"
run_test_chirho "clippy (all std features)" "cargo clippy --features 'arena,parallel,serde,tracing,tms-full,backtrack,network' -- -D warnings"

echo ""
echo "=============================================="
echo -e "${GREEN_CHIRHO}All tests passed: $TESTS_PASSED_CHIRHO / $TESTS_RUN_CHIRHO${NC_CHIRHO}"
echo "=============================================="
