use alloy::rpc::types::Transaction;
use alloy_primitives::B256;

fn main() {
    println!("Hello, world!");
    let transaction = Transaction::default();
    let full = BlockTransactions::Full(vec![transaction]);
    let json = serde_json::to_string(&full).unwrap();
    println!("{}", json);

    let deserialized: BlockTransactions = serde_json::from_str(&json).unwrap();
    println!("{:?}", deserialized);
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
// #[serde(untagged)]
#[serde(tag = "type", content = "txs")]
pub enum BlockTransactions {
    /// Full transactions
    Full(Vec<Transaction>),
    /// Only hashes
    Hashes(Vec<B256>),
}
