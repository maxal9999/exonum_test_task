// Copyright 2018 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Cryptocurrency wallet.

use exonum::crypto::{Hash, PublicKey};

use super::proto;

/// Wallet information stored in the database.
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Wallet", serde_pb_convert)]
pub struct Wallet {
    /// `PublicKey` of the wallet.
    pub pub_key: PublicKey,
    /// Linked multisig wallet.
    /// TODO. Some decision to link single wallet and multisign wallet
    //pub multisig_wallet: PublicKey,
    /// Name of the wallet.
    pub name: String,
    /// Current balance of the wallet.
    pub balance: u64,
    /// Current pending balance
    pub pending_balance: u64,
    /// Pending txs
    pub pending_txs: Vec<Hash>,
    /// Length of the transactions history.
    pub history_len: u64,
    /// `Hash` of the transactions history.
    pub history_hash: Hash,
}

impl Wallet {
    /// Create new Wallet.
    pub fn new(
        &pub_key: &PublicKey,
        name: &str,
        balance: u64,
        pending_balance: u64,
        pending_txs_list: &[Hash],
        history_len: u64,
        &history_hash: &Hash,
    ) -> Self {
        let pending_txs = pending_txs_list.to_vec();
        Self {
            pub_key,
            name: name.to_owned(),
            balance,
            pending_balance,
            pending_txs,
            history_len,
            history_hash,
        }
    }
    /// Returns a copy of this wallet with updated balance.
    pub fn set_balance(self, balance: u64, history_hash: &Hash) -> Self {
        Self::new(
            &self.pub_key,
            &self.name,
            balance,
            self.pending_balance,
            &self.pending_txs,
            self.history_len + 1,
            history_hash,
        )
    }
    /// Returns a copy of this wallet with updated pending balance.
    pub fn set_pending_balance(self, balance: u64) -> Self {
        Self::new(
            &self.pub_key,
            &self.name,
            self.balance,
            balance,
            &self.pending_txs,
            self.history_len,
            &self.history_hash,
        )
    }
    /// Returns a copy of this wallet with updated pending_txs.
    pub fn add_pending_tx(self, tx_hash: &Hash) -> Self {
        let mut pending_txs = self.pending_txs;
        pending_txs.push(*tx_hash);
        Self::new(
            &self.pub_key,
            &self.name,
            self.balance,
            self.pending_balance,
            &pending_txs,
            self.history_len,
            &self.history_hash,
        )
    }
    /// Returns a copy of this wallet with updated pending_txs.
    pub fn delete_pending_tx(self, tx_hash: &Hash) -> Self {
        let mut pending_txs = self.pending_txs;
        if let Some(index) = pending_txs.iter().position(|x| *x == *tx_hash) {
            pending_txs.remove(index);
        }
        Self::new(
            &self.pub_key,
            &self.name,
            self.balance,
            self.pending_balance,
            &pending_txs,
            self.history_len,
            &self.history_hash,
        )
    }
}
