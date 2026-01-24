// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! A propagator-based cryptocurrency: PropCoin (PRC)
//!
//! This module demonstrates how propagator networks can be used to implement
//! a distributed digital currency with unique properties.
//!
//! # Why Propagators for Cryptocurrency?
//!
//! Traditional blockchains use sequential transaction processing. Propagators offer:
//!
//! 1. **CRDT-like Balances**: Account balances merge conflict-free
//! 2. **Bidirectional Constraints**: "What transfer makes account A have exactly $1000?"
//! 3. **Natural Sharding**: Propagator networks partition naturally
//! 4. **Constraint-Based Smart Contracts**: Rules are propagators, not code execution
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                        PropCoin Network                                  │
//! │                                                                          │
//! │  ┌──────────────────────────────────────────────────────────────────┐   │
//! │  │                    Account State Layer                            │   │
//! │  │                                                                   │   │
//! │  │  Account Cells (monotonic balance bounds):                        │   │
//! │  │  ├─ alice: [1000, 1000]  (exact balance)                         │   │
//! │  │  ├─ bob:   [500, 500]    (exact balance)                         │   │
//! │  │  └─ carol: [0, ∞]        (unknown, can only increase)            │   │
//! │  │                                                                   │   │
//! │  │  Conservation Constraint (propagator):                            │   │
//! │  │  sum(all_accounts) = total_supply                                │   │
//! │  │                                                                   │   │
//! │  └──────────────────────────────────────────────────────────────────┘   │
//! │                                                                          │
//! │  ┌──────────────────────────────────────────────────────────────────┐   │
//! │  │                    Transaction Layer                              │   │
//! │  │                                                                   │
//! │  │  Pending Transaction Pool:                                        │   │
//! │  │  ├─ tx_001: alice → bob, 50 PRC, sig: 0x1234...                  │   │
//! │  │  └─ tx_002: bob → carol, 25 PRC, sig: 0x5678...                  │   │
//! │  │                                                                   │   │
//! │  │  Transaction Propagators:                                         │   │
//! │  │  ├─ ValidTransferChirho: sender.balance ≥ amount                 │   │
//! │  │  ├─ BalanceUpdateChirho: sender -= amount, receiver += amount    │   │
//! │  │  └─ NonceChirho: tx.nonce = account.nonce + 1                    │   │
//! │  │                                                                   │   │
//! │  └──────────────────────────────────────────────────────────────────┘   │
//! │                                                                          │
//! │  ┌──────────────────────────────────────────────────────────────────┐   │
//! │  │                    Consensus Layer                                │   │
//! │  │                                                                   │
//! │  │  Validator Nodes (each runs propagator network):                  │   │
//! │  │  ├─ Node A: proposes block, broadcasts state updates             │   │
//! │  │  ├─ Node B: validates via propagation, votes                     │   │
//! │  │  └─ Node C: merges states (CRDT-like, conflict-free)             │   │
//! │  │                                                                   │
//! │  │  Finality: When 2/3+ validators converge to same state           │   │
//! │  │                                                                   │
//! │  └──────────────────────────────────────────────────────────────────┘   │
//! │                                                                          │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Key Innovation: Interval Balances
//!
//! Instead of exact balances, accounts have *interval bounds*:
//!
//! - `[1000, 1000]` = exactly 1000 (fully known)
//! - `[100, ∞]` = at least 100 (lower bound only)
//! - `[0, 500]` = at most 500 (upper bound only)
//!
//! This enables:
//! - **Partial Knowledge**: Nodes don't need full state
//! - **Parallel Validation**: Transactions on disjoint accounts validate independently
//! - **Privacy**: Can prove "I have ≥ 100" without revealing exact balance
//!
//! # Feature Flag
//!
//! Enable with the `crypto` feature:
//! ```toml
//! propagators-chirho = { version = "0.1", features = ["crypto", "network"] }
//! ```

use crate::interval_chirho::NumericInfoChirho;
use crate::network_chirho::{CellIdChirho, CellUpdateChirho, DistributedCellChirho};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// CORE TYPES
// ============================================================================

/// A PropCoin address (simplified as a string for now).
/// In production, this would be a public key hash.
pub type AddressChirho = String;

/// Amount in PropCoin (using u64 for precision, represents smallest unit).
pub type AmountChirho = u64;

/// Transaction nonce for replay protection.
pub type NonceChirho = u64;

/// A transaction hash.
pub type TxHashChirho = String;

/// Block height.
pub type BlockHeightChirho = u64;

// ============================================================================
// ACCOUNT
// ============================================================================

/// An account in the PropCoin network.
///
/// Accounts use interval bounds for balances, enabling partial knowledge
/// and privacy-preserving proofs.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccountChirho {
    /// The account address.
    pub address_chirho: AddressChirho,
    /// Balance as an interval [lo, hi].
    /// Exact balance when lo == hi.
    pub balance_chirho: NumericInfoChirho,
    /// Nonce for replay protection.
    pub nonce_chirho: NonceChirho,
    /// Last update timestamp.
    pub updated_at_chirho: u64,
}

impl AccountChirho {
    /// Creates a new account with zero balance.
    pub fn new_chirho(address_chirho: AddressChirho) -> Self {
        Self {
            address_chirho,
            balance_chirho: NumericInfoChirho::exact_chirho(0.0),
            nonce_chirho: 0,
            updated_at_chirho: now_chirho(),
        }
    }

    /// Creates an account with an initial balance.
    pub fn with_balance_chirho(address_chirho: AddressChirho, balance_chirho: AmountChirho) -> Self {
        Self {
            address_chirho,
            balance_chirho: NumericInfoChirho::exact_chirho(balance_chirho as f64),
            nonce_chirho: 0,
            updated_at_chirho: now_chirho(),
        }
    }

    /// Gets the exact balance if known.
    pub fn exact_balance_chirho(&self) -> Option<AmountChirho> {
        self.balance_chirho.as_interval_chirho().and_then(|iv_chirho| {
            if (iv_chirho.hi_chirho - iv_chirho.lo_chirho).abs() < 0.001 {
                Some(iv_chirho.lo_chirho as AmountChirho)
            } else {
                None
            }
        })
    }

    /// Gets the minimum guaranteed balance.
    pub fn min_balance_chirho(&self) -> AmountChirho {
        self.balance_chirho
            .as_interval_chirho()
            .map(|iv_chirho| iv_chirho.lo_chirho.max(0.0) as AmountChirho)
            .unwrap_or(0)
    }

    /// Gets the maximum possible balance.
    pub fn max_balance_chirho(&self) -> AmountChirho {
        self.balance_chirho
            .as_interval_chirho()
            .map(|iv_chirho| iv_chirho.hi_chirho as AmountChirho)
            .unwrap_or(AmountChirho::MAX)
    }

    /// Checks if the account can definitely afford an amount.
    pub fn can_afford_chirho(&self, amount_chirho: AmountChirho) -> bool {
        self.min_balance_chirho() >= amount_chirho
    }

    /// Checks if the account might be able to afford an amount.
    pub fn might_afford_chirho(&self, amount_chirho: AmountChirho) -> bool {
        self.max_balance_chirho() >= amount_chirho
    }
}

// ============================================================================
// TRANSACTION
// ============================================================================

/// A PropCoin transaction.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransactionChirho {
    /// Transaction hash (computed from contents).
    pub hash_chirho: TxHashChirho,
    /// Sender address.
    pub from_chirho: AddressChirho,
    /// Receiver address.
    pub to_chirho: AddressChirho,
    /// Amount to transfer.
    pub amount_chirho: AmountChirho,
    /// Sender's nonce (for replay protection).
    pub nonce_chirho: NonceChirho,
    /// Transaction timestamp.
    pub timestamp_chirho: u64,
    /// Signature (simplified - in production use Ed25519).
    pub signature_chirho: String,
}

impl TransactionChirho {
    /// Creates a new transaction.
    pub fn new_chirho(
        from_chirho: AddressChirho,
        to_chirho: AddressChirho,
        amount_chirho: AmountChirho,
        nonce_chirho: NonceChirho,
    ) -> Self {
        let timestamp_chirho = now_chirho();
        let hash_chirho = compute_tx_hash_chirho(&from_chirho, &to_chirho, amount_chirho, nonce_chirho, timestamp_chirho);

        Self {
            hash_chirho,
            from_chirho,
            to_chirho,
            amount_chirho,
            nonce_chirho,
            timestamp_chirho,
            signature_chirho: String::new(),
        }
    }

    /// Signs the transaction (simplified - just stores the signature).
    pub fn sign_chirho(&mut self, signature_chirho: String) {
        self.signature_chirho = signature_chirho;
    }

    /// Verifies the signature (simplified - always returns true for now).
    pub fn verify_signature_chirho(&self) -> bool {
        !self.signature_chirho.is_empty()
    }
}

/// Transaction validation result.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TxValidationResultChirho {
    /// Transaction is valid.
    ValidChirho,
    /// Insufficient balance.
    InsufficientBalanceChirho,
    /// Invalid nonce.
    InvalidNonceChirho,
    /// Invalid signature.
    InvalidSignatureChirho,
    /// Self-transfer not allowed.
    SelfTransferChirho,
    /// Zero amount not allowed.
    ZeroAmountChirho,
    /// Sender account not found.
    SenderNotFoundChirho,
}

// ============================================================================
// BLOCK
// ============================================================================

/// A block in the PropCoin blockchain.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BlockChirho {
    /// Block height.
    pub height_chirho: BlockHeightChirho,
    /// Previous block hash.
    pub prev_hash_chirho: String,
    /// This block's hash.
    pub hash_chirho: String,
    /// Transactions in this block.
    pub transactions_chirho: Vec<TransactionChirho>,
    /// Block timestamp.
    pub timestamp_chirho: u64,
    /// Validator who proposed this block.
    pub proposer_chirho: AddressChirho,
    /// State root after applying transactions.
    pub state_root_chirho: String,
}

impl BlockChirho {
    /// Creates a new block.
    pub fn new_chirho(
        height_chirho: BlockHeightChirho,
        prev_hash_chirho: String,
        transactions_chirho: Vec<TransactionChirho>,
        proposer_chirho: AddressChirho,
    ) -> Self {
        let timestamp_chirho = now_chirho();
        let hash_chirho = compute_block_hash_chirho(height_chirho, &prev_hash_chirho, &transactions_chirho, timestamp_chirho);

        Self {
            height_chirho,
            prev_hash_chirho,
            hash_chirho,
            transactions_chirho,
            timestamp_chirho,
            proposer_chirho,
            state_root_chirho: String::new(),
        }
    }

    /// Returns the genesis block.
    pub fn genesis_chirho(initial_accounts_chirho: Vec<(AddressChirho, AmountChirho)>) -> Self {
        // Genesis block has special transactions that create initial balances
        let transactions_chirho: Vec<TransactionChirho> = initial_accounts_chirho
            .into_iter()
            .map(|(addr_chirho, amount_chirho)| {
                TransactionChirho::new_chirho(
                    "genesis".to_string(),
                    addr_chirho,
                    amount_chirho,
                    0,
                )
            })
            .collect();

        Self {
            height_chirho: 0,
            prev_hash_chirho: "0".repeat(64),
            hash_chirho: "genesis_block_hash".to_string(),
            transactions_chirho,
            timestamp_chirho: now_chirho(),
            proposer_chirho: "genesis".to_string(),
            state_root_chirho: String::new(),
        }
    }
}

// ============================================================================
// PROPAGATOR-BASED LEDGER
// ============================================================================

/// A PropCoin ledger using propagator networks.
///
/// This is the core innovation: account balances are cells in a propagator
/// network, and transactions are constraints that must be satisfied.
pub struct PropCoinLedgerChirho {
    /// Account states.
    accounts_chirho: HashMap<AddressChirho, AccountChirho>,
    /// Account balance cells (for distributed sync).
    balance_cells_chirho: HashMap<AddressChirho, Arc<DistributedCellChirho>>,
    /// Pending transactions.
    pending_txs_chirho: Vec<TransactionChirho>,
    /// Blockchain.
    blocks_chirho: Vec<BlockChirho>,
    /// Total supply (conservation constraint).
    total_supply_chirho: AmountChirho,
    /// This node's validator address.
    validator_address_chirho: AddressChirho,
}

impl PropCoinLedgerChirho {
    /// Creates a new ledger with genesis state.
    pub fn new_chirho(
        validator_address_chirho: AddressChirho,
        genesis_accounts_chirho: Vec<(AddressChirho, AmountChirho)>,
    ) -> Self {
        let total_supply_chirho: AmountChirho = genesis_accounts_chirho
            .iter()
            .map(|(_, amount_chirho)| amount_chirho)
            .sum();

        let mut accounts_chirho = HashMap::new();
        let mut balance_cells_chirho = HashMap::new();

        for (addr_chirho, amount_chirho) in &genesis_accounts_chirho {
            let account_chirho = AccountChirho::with_balance_chirho(addr_chirho.clone(), *amount_chirho);
            accounts_chirho.insert(addr_chirho.clone(), account_chirho);

            let cell_chirho = Arc::new(DistributedCellChirho::new_chirho(&format!("balance:{}", addr_chirho)));
            cell_chirho.merge_chirho(NumericInfoChirho::exact_chirho(*amount_chirho as f64), 0);
            balance_cells_chirho.insert(addr_chirho.clone(), cell_chirho);
        }

        let genesis_block_chirho = BlockChirho::genesis_chirho(genesis_accounts_chirho);

        Self {
            accounts_chirho,
            balance_cells_chirho,
            pending_txs_chirho: Vec::new(),
            blocks_chirho: vec![genesis_block_chirho],
            total_supply_chirho,
            validator_address_chirho,
        }
    }

    /// Gets an account by address.
    pub fn get_account_chirho(&self, address_chirho: &AddressChirho) -> Option<&AccountChirho> {
        self.accounts_chirho.get(address_chirho)
    }

    /// Gets account balance.
    pub fn get_balance_chirho(&self, address_chirho: &AddressChirho) -> Option<AmountChirho> {
        self.accounts_chirho
            .get(address_chirho)
            .and_then(|acc_chirho| acc_chirho.exact_balance_chirho())
    }

    /// Validates a transaction.
    pub fn validate_tx_chirho(&self, tx_chirho: &TransactionChirho) -> TxValidationResultChirho {
        // Check signature
        if !tx_chirho.verify_signature_chirho() {
            return TxValidationResultChirho::InvalidSignatureChirho;
        }

        // Check self-transfer
        if tx_chirho.from_chirho == tx_chirho.to_chirho {
            return TxValidationResultChirho::SelfTransferChirho;
        }

        // Check zero amount
        if tx_chirho.amount_chirho == 0 {
            return TxValidationResultChirho::ZeroAmountChirho;
        }

        // Check sender exists
        let sender_chirho = match self.accounts_chirho.get(&tx_chirho.from_chirho) {
            Some(acc_chirho) => acc_chirho,
            None => return TxValidationResultChirho::SenderNotFoundChirho,
        };

        // Check nonce
        if tx_chirho.nonce_chirho != sender_chirho.nonce_chirho + 1 {
            return TxValidationResultChirho::InvalidNonceChirho;
        }

        // Check balance using propagator interval bounds
        if !sender_chirho.can_afford_chirho(tx_chirho.amount_chirho) {
            return TxValidationResultChirho::InsufficientBalanceChirho;
        }

        TxValidationResultChirho::ValidChirho
    }

    /// Submits a transaction to the pending pool.
    ///
    /// # Errors
    ///
    /// Returns validation error if transaction is invalid.
    pub fn submit_tx_chirho(
        &mut self,
        tx_chirho: TransactionChirho,
    ) -> Result<TxHashChirho, TxValidationResultChirho> {
        let validation_chirho = self.validate_tx_chirho(&tx_chirho);
        if validation_chirho != TxValidationResultChirho::ValidChirho {
            return Err(validation_chirho);
        }

        let hash_chirho = tx_chirho.hash_chirho.clone();
        self.pending_txs_chirho.push(tx_chirho);
        Ok(hash_chirho)
    }

    /// Applies a transaction to the state.
    ///
    /// This is where propagators shine - we update balance cells and
    /// the network automatically propagates constraints.
    fn apply_tx_chirho(&mut self, tx_chirho: &TransactionChirho) -> bool {
        // Get or create sender account
        let sender_chirho = self
            .accounts_chirho
            .entry(tx_chirho.from_chirho.clone())
            .or_insert_with(|| AccountChirho::new_chirho(tx_chirho.from_chirho.clone()));

        // Deduct from sender
        if let Some(iv_chirho) = sender_chirho.balance_chirho.as_interval_chirho() {
            let new_lo_chirho = iv_chirho.lo_chirho - tx_chirho.amount_chirho as f64;
            let new_hi_chirho = iv_chirho.hi_chirho - tx_chirho.amount_chirho as f64;

            if new_lo_chirho < 0.0 {
                return false; // Insufficient balance
            }

            sender_chirho.balance_chirho = NumericInfoChirho::interval_chirho(new_lo_chirho, new_hi_chirho);
        }
        sender_chirho.nonce_chirho = tx_chirho.nonce_chirho;
        sender_chirho.updated_at_chirho = now_chirho();

        // Update sender balance cell
        if let Some(cell_chirho) = self.balance_cells_chirho.get(&tx_chirho.from_chirho) {
            cell_chirho.merge_chirho(sender_chirho.balance_chirho, now_chirho());
        }

        // Get or create receiver account
        let receiver_chirho = self
            .accounts_chirho
            .entry(tx_chirho.to_chirho.clone())
            .or_insert_with(|| AccountChirho::new_chirho(tx_chirho.to_chirho.clone()));

        // Credit to receiver
        if let Some(iv_chirho) = receiver_chirho.balance_chirho.as_interval_chirho() {
            let new_lo_chirho = iv_chirho.lo_chirho + tx_chirho.amount_chirho as f64;
            let new_hi_chirho = iv_chirho.hi_chirho + tx_chirho.amount_chirho as f64;

            receiver_chirho.balance_chirho = NumericInfoChirho::interval_chirho(new_lo_chirho, new_hi_chirho);
        }
        receiver_chirho.updated_at_chirho = now_chirho();

        // Update receiver balance cell
        if !self.balance_cells_chirho.contains_key(&tx_chirho.to_chirho) {
            let cell_chirho = Arc::new(DistributedCellChirho::new_chirho(&format!(
                "balance:{}",
                tx_chirho.to_chirho
            )));
            self.balance_cells_chirho
                .insert(tx_chirho.to_chirho.clone(), cell_chirho);
        }
        if let Some(cell_chirho) = self.balance_cells_chirho.get(&tx_chirho.to_chirho) {
            cell_chirho.merge_chirho(receiver_chirho.balance_chirho, now_chirho());
        }

        true
    }

    /// Creates a new block from pending transactions.
    pub fn create_block_chirho(&mut self) -> Option<BlockChirho> {
        if self.pending_txs_chirho.is_empty() {
            return None;
        }

        let prev_block_chirho = self.blocks_chirho.last()?;
        let height_chirho = prev_block_chirho.height_chirho + 1;
        let prev_hash_chirho = prev_block_chirho.hash_chirho.clone();

        // Take pending transactions
        let transactions_chirho = std::mem::take(&mut self.pending_txs_chirho);

        let block_chirho = BlockChirho::new_chirho(
            height_chirho,
            prev_hash_chirho,
            transactions_chirho,
            self.validator_address_chirho.clone(),
        );

        Some(block_chirho)
    }

    /// Applies a block to the ledger.
    pub fn apply_block_chirho(&mut self, block_chirho: BlockChirho) -> bool {
        // Validate block height
        let expected_height_chirho = self.blocks_chirho.len() as BlockHeightChirho;
        if block_chirho.height_chirho != expected_height_chirho {
            return false;
        }

        // Validate prev hash
        if let Some(prev_block_chirho) = self.blocks_chirho.last() {
            if block_chirho.prev_hash_chirho != prev_block_chirho.hash_chirho {
                return false;
            }
        }

        // Apply all transactions
        for tx_chirho in &block_chirho.transactions_chirho {
            // Skip genesis transactions (from "genesis" address)
            if tx_chirho.from_chirho == "genesis" {
                // Apply credit only
                let receiver_chirho = self
                    .accounts_chirho
                    .entry(tx_chirho.to_chirho.clone())
                    .or_insert_with(|| AccountChirho::new_chirho(tx_chirho.to_chirho.clone()));

                receiver_chirho.balance_chirho = NumericInfoChirho::exact_chirho(tx_chirho.amount_chirho as f64);
                continue;
            }

            if !self.apply_tx_chirho(tx_chirho) {
                return false;
            }
        }

        self.blocks_chirho.push(block_chirho);
        true
    }

    /// Gets cell updates for distributed sync.
    pub fn get_balance_updates_chirho(&self) -> Vec<CellUpdateChirho> {
        self.balance_cells_chirho
            .iter()
            .map(|(addr_chirho, cell_chirho)| CellUpdateChirho {
                cell_id_chirho: CellIdChirho::new_chirho(&format!("balance:{}", addr_chirho)),
                info_chirho: cell_chirho.content_chirho(),
                timestamp_chirho: cell_chirho.timestamp_chirho(),
                origin_chirho: self.validator_address_chirho.clone(),
            })
            .collect()
    }

    /// Merges balance updates from another node.
    ///
    /// This is where CRDT-like propagator merging shines - updates
    /// from different nodes merge conflict-free.
    pub fn merge_balance_updates_chirho(&mut self, updates_chirho: Vec<CellUpdateChirho>) {
        for update_chirho in updates_chirho {
            let addr_chirho = update_chirho
                .cell_id_chirho
                .name_chirho
                .strip_prefix("balance:")
                .unwrap_or(&update_chirho.cell_id_chirho.name_chirho)
                .to_string();

            // Merge into cell (CRDT-like)
            let cell_chirho = self
                .balance_cells_chirho
                .entry(addr_chirho.clone())
                .or_insert_with(|| Arc::new(DistributedCellChirho::new_chirho(&update_chirho.cell_id_chirho.name_chirho)));

            cell_chirho.merge_chirho(update_chirho.info_chirho, update_chirho.timestamp_chirho);

            // Update account
            let account_chirho = self
                .accounts_chirho
                .entry(addr_chirho)
                .or_insert_with(|| AccountChirho::new_chirho(update_chirho.cell_id_chirho.name_chirho.clone()));

            // Merge balance (intersection = more precise information)
            account_chirho.balance_chirho = account_chirho
                .balance_chirho
                .merge_chirho(&update_chirho.info_chirho);
        }
    }

    /// Gets the current block height.
    pub fn height_chirho(&self) -> BlockHeightChirho {
        self.blocks_chirho.len() as BlockHeightChirho - 1
    }

    /// Gets total supply.
    pub fn total_supply_chirho(&self) -> AmountChirho {
        self.total_supply_chirho
    }

    /// Gets number of accounts.
    pub fn account_count_chirho(&self) -> usize {
        self.accounts_chirho.len()
    }

    /// Gets pending transaction count.
    pub fn pending_tx_count_chirho(&self) -> usize {
        self.pending_txs_chirho.len()
    }
}

// ============================================================================
// HELPERS
// ============================================================================

fn now_chirho() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d_chirho| d_chirho.as_secs())
        .unwrap_or(0)
}

fn compute_tx_hash_chirho(
    from_chirho: &str,
    to_chirho: &str,
    amount_chirho: AmountChirho,
    nonce_chirho: NonceChirho,
    timestamp_chirho: u64,
) -> TxHashChirho {
    // Simplified hash - in production use SHA256 or similar
    format!(
        "tx_{:x}",
        from_chirho.len() as u64
            ^ to_chirho.len() as u64
            ^ amount_chirho
            ^ nonce_chirho
            ^ timestamp_chirho
    )
}

fn compute_block_hash_chirho(
    height_chirho: BlockHeightChirho,
    prev_hash_chirho: &str,
    transactions_chirho: &[TransactionChirho],
    timestamp_chirho: u64,
) -> String {
    // Simplified hash - in production use SHA256 or similar
    format!(
        "block_{:x}",
        height_chirho ^ prev_hash_chirho.len() as u64 ^ transactions_chirho.len() as u64 ^ timestamp_chirho
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_create_ledger_chirho() {
        let ledger_chirho = PropCoinLedgerChirho::new_chirho(
            "validator1".to_string(),
            vec![
                ("alice".to_string(), 1000),
                ("bob".to_string(), 500),
            ],
        );

        assert_eq!(ledger_chirho.total_supply_chirho(), 1500);
        assert_eq!(ledger_chirho.get_balance_chirho(&"alice".to_string()), Some(1000));
        assert_eq!(ledger_chirho.get_balance_chirho(&"bob".to_string()), Some(500));
    }

    #[test]
    fn test_transfer_chirho() {
        let mut ledger_chirho = PropCoinLedgerChirho::new_chirho(
            "validator1".to_string(),
            vec![
                ("alice".to_string(), 1000),
                ("bob".to_string(), 500),
            ],
        );

        // Create and submit transaction
        let mut tx_chirho = TransactionChirho::new_chirho(
            "alice".to_string(),
            "bob".to_string(),
            100,
            1, // nonce
        );
        tx_chirho.sign_chirho("sig".to_string());

        let result_chirho = ledger_chirho.submit_tx_chirho(tx_chirho);
        assert!(result_chirho.is_ok());

        // Create and apply block
        let block_chirho = ledger_chirho.create_block_chirho().unwrap();
        assert!(ledger_chirho.apply_block_chirho(block_chirho));

        // Check balances
        assert_eq!(ledger_chirho.get_balance_chirho(&"alice".to_string()), Some(900));
        assert_eq!(ledger_chirho.get_balance_chirho(&"bob".to_string()), Some(600));

        // Total supply unchanged
        assert_eq!(ledger_chirho.total_supply_chirho(), 1500);
    }

    #[test]
    fn test_insufficient_balance_chirho() {
        let mut ledger_chirho = PropCoinLedgerChirho::new_chirho(
            "validator1".to_string(),
            vec![("alice".to_string(), 100)],
        );

        let mut tx_chirho = TransactionChirho::new_chirho(
            "alice".to_string(),
            "bob".to_string(),
            200, // More than alice has
            1,
        );
        tx_chirho.sign_chirho("sig".to_string());

        let result_chirho = ledger_chirho.submit_tx_chirho(tx_chirho);
        assert_eq!(result_chirho, Err(TxValidationResultChirho::InsufficientBalanceChirho));
    }

    #[test]
    fn test_invalid_nonce_chirho() {
        let mut ledger_chirho = PropCoinLedgerChirho::new_chirho(
            "validator1".to_string(),
            vec![("alice".to_string(), 1000)],
        );

        let mut tx_chirho = TransactionChirho::new_chirho(
            "alice".to_string(),
            "bob".to_string(),
            100,
            5, // Wrong nonce (should be 1)
        );
        tx_chirho.sign_chirho("sig".to_string());

        let result_chirho = ledger_chirho.submit_tx_chirho(tx_chirho);
        assert_eq!(result_chirho, Err(TxValidationResultChirho::InvalidNonceChirho));
    }

    #[test]
    fn test_merge_balance_updates_chirho() {
        let mut ledger_a_chirho = PropCoinLedgerChirho::new_chirho(
            "node_a".to_string(),
            vec![("alice".to_string(), 1000)],
        );

        let mut ledger_b_chirho = PropCoinLedgerChirho::new_chirho(
            "node_b".to_string(),
            vec![("alice".to_string(), 1000)],
        );

        // Node A processes a transaction
        let mut tx_chirho = TransactionChirho::new_chirho(
            "alice".to_string(),
            "bob".to_string(),
            100,
            1,
        );
        tx_chirho.sign_chirho("sig".to_string());
        let _ = ledger_a_chirho.submit_tx_chirho(tx_chirho);
        let block_chirho = ledger_a_chirho.create_block_chirho().unwrap();
        ledger_a_chirho.apply_block_chirho(block_chirho);

        // Get updates from Node A
        let updates_chirho = ledger_a_chirho.get_balance_updates_chirho();

        // Merge into Node B (CRDT-like)
        ledger_b_chirho.merge_balance_updates_chirho(updates_chirho);

        // Node B should now have alice's updated balance
        let alice_b_chirho = ledger_b_chirho.get_account_chirho(&"alice".to_string()).unwrap();
        assert!(alice_b_chirho.min_balance_chirho() <= 900);
    }

    #[test]
    fn test_interval_balance_chirho() {
        let mut account_chirho = AccountChirho::new_chirho("test".to_string());

        // Set interval balance [100, 500]
        account_chirho.balance_chirho = NumericInfoChirho::interval_chirho(100.0, 500.0);

        assert!(account_chirho.can_afford_chirho(100)); // Definitely can
        assert!(!account_chirho.can_afford_chirho(200)); // Not certain
        assert!(account_chirho.might_afford_chirho(200)); // Might be able to
        assert!(account_chirho.might_afford_chirho(500)); // Might be able to
        assert!(!account_chirho.might_afford_chirho(600)); // Definitely can't
    }
}
