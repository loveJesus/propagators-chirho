<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

# PropCoin Chirho

A propagator-based cryptocurrency using interval balances and CRDT-like distributed consensus.

## Overview

PropCoin demonstrates how propagator networks can be used to build a distributed ledger with unique properties:

- **Interval Balances**: Instead of exact balances, accounts have interval bounds `[min, max]` that can represent partial knowledge or privacy-preserving proofs
- **CRDT-like Merging**: Balance updates merge monotonically, enabling conflict-free distributed sync
- **Lamport Timestamps**: Distributed ordering for consistent state across nodes
- **Transaction Validation**: Nonce-based replay protection and balance verification

## Features

- `PropCoinLedgerChirho` - The main ledger that manages accounts and transactions
- `AccountChirho` - Accounts with interval balance bounds
- `TransactionChirho` - Transfer transactions with sender, receiver, amount, and nonce
- `BlockChirho` - Blockchain blocks containing batched transactions

## Usage

```rust
use propcoin_chirho::{PropCoinLedgerChirho, TransactionChirho};

// Create a new ledger with 1 million total supply
let mut ledger_chirho = PropCoinLedgerChirho::new_chirho(1_000_000, "validator_address");

// Create an account
let alice_chirho = "alice_address".to_string();
ledger_chirho.create_account_chirho(alice_chirho.clone(), 1000);

// Check balance (returns interval bounds!)
if let Some(account_chirho) = ledger_chirho.get_account_chirho(&alice_chirho) {
    println!("Alice's balance: {:?}", account_chirho.balance_chirho);
}

// Create a transfer
let bob_chirho = "bob_address".to_string();
ledger_chirho.create_account_chirho(bob_chirho.clone(), 0);

let tx_chirho = TransactionChirho {
    from_chirho: alice_chirho.clone(),
    to_chirho: bob_chirho.clone(),
    amount_chirho: 100,
    nonce_chirho: 0,
    timestamp_chirho: 1,
};

ledger_chirho.submit_tx_chirho(tx_chirho).expect("Transfer should succeed");
```

## Why Interval Balances?

Traditional blockchains use exact balances. PropCoin uses intervals for:

1. **Partial Knowledge**: A node might know "Alice has at least 100 tokens" without knowing the exact amount
2. **Privacy Proofs**: Prove you have enough for a transaction without revealing total balance
3. **Distributed Merging**: Intervals merge monotonically (narrowing), perfect for CRDT-style distributed systems

## Dependencies

This crate depends on `propagators-chirho` with the `network` and `serde` features enabled.

```toml
[dependencies]
propcoin-chirho = "0.1"
```

## License

MIT
