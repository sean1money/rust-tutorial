// [package]
// name = "reth-codecs-test"
// version = "0.1.0"
// publish = false
// edition.workspace = true
// license.workspace = true

// [dependencies]
// reth.workspace = true
// reth-codecs-derive = { workspace = true }
// reth-codecs = { workspace = true }
// serde = { workspace = true }
// alloy-rlp = { workspace = true }
// derive_more = { workspace = true }
// proptest-arbitrary-interop = { workspace = true }
// arbitrary = { workspace = true }
// proptest = { workspace = true }
// rand = { workspace = true }

use derive_more::Deref;

use alloy_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

// use alloy_rlp::Encodable;
// use reth::primitives::{Block, Header};

fn main() {
    println!("Hello, world!");

    // let block = Block { header: Header { number: 1, ..Default::default() }, ..Default::default() };
    // println!("{:?}", block);
    // println!("{:?}", block.length());
}

#[reth_codecs::add_arbitrary_tests(rlp, 25)]
// #[cfg_attr(any(test, feature = "reth-codec"), reth_codecs::add_arbitrary_tests(rlp, 25))]
#[derive(
    Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, Deref, RlpEncodable, RlpDecodable,
)]
struct Block {
    pub body: Vec<String>,
}

impl<'a> arbitrary::Arbitrary<'a> for Block {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        // first generate up to 100 txs
        let transactions = (0..100)
            .map(|_| String::arbitrary(u))
            .collect::<arbitrary::Result<Vec<_>>>()?;

        Ok(Self { body: transactions })
    }
}
