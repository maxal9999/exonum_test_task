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

//! Cryptocurrency transactions.

// Workaround for `failure` see https://github.com/rust-lang-nursery/failure/issues/223 and
// ECR-1771 for the details.
#![allow(bare_trait_objects)]

use exonum::{
    blockchain::{ExecutionError, ExecutionResult, Transaction, TransactionContext},
    crypto::{Hash, PublicKey, SecretKey},
    messages::{Message, RawTransaction, Signed},
};

use super::proto;
use schema::Schema;
use CRYPTOCURRENCY_SERVICE_ID;

const ERROR_SENDER_SAME_AS_RECEIVER: u8 = 0;

/// Error codes emitted by wallet transactions during execution.
#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    /// Wallet already exists.
    ///
    /// Can be emitted by `CreateWallet`.
    #[fail(display = "Wallet already exists")]
    WalletAlreadyExists = 0,

    /// Sender doesn't exist.
    ///
    /// Can be emitted by `Transfer`.
    #[fail(display = "Sender doesn't exist")]
    SenderNotFound = 1,

    /// Receiver doesn't exist.
    ///
    /// Can be emitted by `Transfer` or `Issue`.
    #[fail(display = "Receiver doesn't exist")]
    ReceiverNotFound = 2,

    /// Insufficient currency amount.
    ///
    /// Can be emitted by `Transfer`.
    #[fail(display = "Insufficient currency amount")]
    InsufficientCurrencyAmount = 3,
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}

/// Transfer `amount` of the currency from one wallet to another.
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Transfer", serde_pb_convert)]
pub struct Transfer {
    /// `PublicKey` of receiver's wallet.
    pub to: PublicKey,
    /// Amount of currency to transfer.
    pub amount: u64,
    /// Auxiliary number to guarantee [non-idempotence][idempotence] of transactions.
    ///
    /// [idempotence]: https://en.wikipedia.org/wiki/Idempotence
    pub seed: u64,
}

/// Multisignature transfer `amount` of the currency from one multisig wallet to another.
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::TransferMultisign", serde_pb_convert)]
pub struct TransferMultisign {
    /// `PublicKey` of multisign sender's wallet.
    pub from: PublicKey,
    /// `PublicKey` of receiver's wallet.
    pub to: PublicKey,
    /// Approvers of this transfer.
    pub approvers: Vec<PublicKey>,
    /// Amount of currency to transfer.
    pub amount: u64,
    /// Auxiliary number to guarantee [non-idempotence][idempotence] of transactions.
    ///
    /// [idempotence]: https://en.wikipedia.org/wiki/Idempotence
    pub seed: u64,
}

/// Accept transfer for multisignature transfer.
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::AcceptMultisign", serde_pb_convert)]
pub struct AcceptMultisign {
    /// Hash of the accepted transfer.
    pub tx_hash: Hash,
    /// `PublicKey` of multisign sender's wallet.
    pub from: PublicKey,
    /// `PublicKey` of receiver's wallet.
    pub to: PublicKey,
    /// Approvers of this transfer
    pub approvers: Vec<PublicKey>,
    /// Auxiliary number to guarantee [non-idempotence][idempotence] of transactions.
    ///
    /// [idempotence]: https://en.wikipedia.org/wiki/Idempotence
    pub seed: u64,
}

/// Issue `amount` of the currency to the `wallet`.
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Issue")]
pub struct Issue {
    /// Issued amount of currency.
    pub amount: u64,
    /// Auxiliary number to guarantee [non-idempotence][idempotence] of transactions.
    ///
    /// [idempotence]: https://en.wikipedia.org/wiki/Idempotence
    pub seed: u64,
}

/// Create wallet with the given `name`.
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::CreateWallet")]
pub struct CreateWallet {
    /// Name of the new wallet.
    pub name: String,
}

/// Transaction group.
#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum WalletTransactions {
    /// Transfer tx.
    Transfer(Transfer),
    /// Multisign transfer tx.
    TransferMultisign(TransferMultisign),
    /// Accept multisign transfer
    AcceptMultisign(AcceptMultisign),
    /// Issue tx.
    Issue(Issue),
    /// CreateWallet tx.
    CreateWallet(CreateWallet),
}

impl CreateWallet {
    #[doc(hidden)]
    pub fn sign(name: &str, pk: &PublicKey, sk: &SecretKey) -> Signed<RawTransaction> {
        Message::sign_transaction(
            Self {
                name: name.to_owned(),
            },
            CRYPTOCURRENCY_SERVICE_ID,
            *pk,
            sk,
        )
    }
}

impl Transfer {
    #[doc(hidden)]
    pub fn sign(
        pk: &PublicKey,
        &to: &PublicKey,
        amount: u64,
        seed: u64,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        Message::sign_transaction(
            Self { to, amount, seed },
            CRYPTOCURRENCY_SERVICE_ID,
            *pk,
            sk,
        )
    }
}

impl TransferMultisign {
    #[doc(hidden)]
    pub fn sign(
        pk: &PublicKey,
        &from: &PublicKey,
        &to: &PublicKey,
        ref users: &Vec<PublicKey>,
        amount: u64,
        seed: u64,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        let approvers = users.to_vec();
        Message::sign_transaction(
            Self { from, to, approvers, amount, seed },
            CRYPTOCURRENCY_SERVICE_ID,
            *pk,
            sk,
        )
    }
}

impl AcceptMultisign {
    #[doc(hidden)]
    pub fn sign(
        pk: &PublicKey,
        &tx_hash: &Hash,
        &from: &PublicKey,
        &to: &PublicKey,
        ref users: &Vec<PublicKey>,
        seed: u64,
        sk: &SecretKey,
    ) -> Signed<RawTransaction> {
        let approvers = users.to_vec();
        Message::sign_transaction(
            Self { tx_hash, from, to, approvers, seed },
            CRYPTOCURRENCY_SERVICE_ID,
            *pk,
            sk,
        )
    }
}

impl Transaction for Transfer {
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        let from = &context.author();
        let hash = context.tx_hash();

        let mut schema = Schema::new(context.fork());

        let to = &self.to;
        let amount = self.amount;

        if from == to {
            return Err(ExecutionError::new(ERROR_SENDER_SAME_AS_RECEIVER));
        }

        let sender = schema.wallet(from).ok_or(Error::SenderNotFound)?;

        let receiver = schema.wallet(to).ok_or(Error::ReceiverNotFound)?;

        if sender.balance < amount {
            Err(Error::InsufficientCurrencyAmount)?
        }

        schema.decrease_wallet_balance(sender, amount, &hash);
        schema.increase_wallet_balance(receiver, amount, &hash);

        Ok(())
    }
}

impl Transaction for TransferMultisign {
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        let significant = &context.author();
        let hash = context.tx_hash();

        let mut schema = Schema::new(context.fork());

        let from = &self.from;
        let to = &self.to;
        let amount = self.amount;

        if from == to {
            return Err(ExecutionError::new(ERROR_SENDER_SAME_AS_RECEIVER));
        }

        self.approvers.iter().find(|&&x| x == *significant).ok_or(Error::SenderNotFound)?;

        let sender = schema.wallet(from).ok_or(Error::SenderNotFound)?;

        schema.wallet(to).ok_or(Error::ReceiverNotFound)?;

        if sender.balance < amount {
            Err(Error::InsufficientCurrencyAmount)?
        }

        let sender = schema.add_tx_to_wallet(sender, &hash);
        schema.decrease_wallet_pending_balance(sender, amount);

        Ok(())
    }
}

impl Transaction for AcceptMultisign {
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        let significant = &context.author();

        let mut schema = Schema::new(context.fork());

        let hash = &self.tx_hash;
        let from = &self.from;
        let to = &self.to;

        if from == to {
            return Err(ExecutionError::new(ERROR_SENDER_SAME_AS_RECEIVER));
        }

        let sender = schema.wallet(from).ok_or(Error::SenderNotFound)?;

        let receiver = schema.wallet(to).ok_or(Error::ReceiverNotFound)?;

        let pending_txs = sender.pending_txs.clone();

        if let Some(tx_hash) = pending_txs.iter().find(|&&x| x == *hash) {
            if let Some(_pub_key) = self.approvers.iter().find(|&&x| x == *significant) {
                let sender = schema.remove_tx_from_wallet(sender, &tx_hash);
                let new_amount = sender.balance - sender.pending_balance;
                schema.decrease_wallet_balance(sender, new_amount, &tx_hash);
                schema.increase_wallet_balance(receiver, new_amount, &tx_hash);
                return Ok(());
            }
            else {
                Err(Error::SenderNotFound)?
            }
        }

        Ok(())
    }
}

impl Transaction for Issue {
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        let pub_key = &context.author();
        let hash = context.tx_hash();

        let mut schema = Schema::new(context.fork());

        if let Some(wallet) = schema.wallet(pub_key) {
            let amount = self.amount;
            schema.increase_wallet_balance(wallet, amount, &hash);
            Ok(())
        } else {
            Err(Error::ReceiverNotFound)?
        }
    }
}

impl Transaction for CreateWallet {
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        let pub_key = &context.author();
        let hash = context.tx_hash();

        let mut schema = Schema::new(context.fork());

        if schema.wallet(pub_key).is_none() {
            let name = &self.name;
            schema.create_wallet(pub_key, name, &hash);
            Ok(())
        } else {
            Err(Error::WalletAlreadyExists)?
        }
    }
}
