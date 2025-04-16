use alloy::primitives::{Bytes, ChainId, TxHash};
use alloy_primitives::{Address, U256};
use alloy_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

fn main() {
    println!("Alloy 102 example!");
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RlpEncodable, RlpDecodable)]
#[rlp(trailing)]
pub struct RecoveryMessage {
    pub nonce: u64,
    pub incomplete_signatures: Vec<IncompleteSignature>,
    pub address: Address,
    pub value: U256,
    pub chain_id: Option<ChainId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RlpEncodable, RlpDecodable)]
pub struct IncompleteSignature {
    pub tx_hash: TxHash,
    pub signatures: Vec<Bytes>,
}
