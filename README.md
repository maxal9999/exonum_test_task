# Exonum test task
 My test case solution for multisignature transfer.
 
## Wallet
 Wallet information stored in the database.
 
 pub struct Wallet {
    
    pub pub_key: PublicKey,  /// `PublicKey` of the wallet.
 
    pub name: String,  /// Name of the wallet.
 
    pub balance: u64,  /// Current balance of the wallet.

    pub pending_balance: u64,/// Current pending balance
                         /// TODO. This field is not thread safe.

    pub pending_txs: Vec<Hash>, /// Pending txs
                        /// TODO. This field is not thread safe.
    
    pub history_len: u64, /// Length of the transactions history.
    
    pub history_hash: Hash, /// `Hash` of the transactions history.
}
 
Field "pending_balance" is the pending balance after multisign transaction confirmation.
Field "pending_txs" is the list of multisign transaction's hashes, which are pending confirmation.

## Multisignature transfer
Multisignature transfer `amount` of the currency from one multisig wallet to another.

pub struct TransferMultisign {
    
    pub from: PublicKey, /// `PublicKey` of multisign sender's wallet.
    
    pub to: PublicKey, /// `PublicKey` of receiver's wallet.
    
    pub approvers: Vec<PublicKey>, /// Approvers of this transfer.
    
    pub amount: u64, /// Amount of currency to transfer.
    
    pub seed: u64, /// Auxiliary number to guarantee [non-idempotence][idempotence] of transactions.
}

Field "approvers" is the list of the users public keys, which must to approve the multisign transfer.

This transaction initiates the transfer of money from the multisign wallet to another wallet. It is also added to the list of transactions for confirmation.

## Accept multisign transfer
Accept transfer for multisignature transfer.

pub struct AcceptMultisign {
    
    pub tx_hash: Hash, /// Hash of the accepted transfer.
    
    pub from: PublicKey, /// `PublicKey` of multisign sender's wallet.
    
    pub to: PublicKey, /// `PublicKey` of receiver's wallet.
    
    pub approvers: Vec<PublicKey>, /// Approvers of this transfer
    
    pub seed: u64, /// Auxiliary number to guarantee [non-idempotence][idempotence] of transactions.
}

 
