use alloy::{
    network::EthereumWallet, providers::ProviderBuilder, signers::local::PrivateKeySigner, sol,
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let bytes =
        hex::decode("8178ebd5ed69413f50fda1710254350be874b3020319ce7b9b8e38810f358b1b").unwrap();
    let signer = PrivateKeySigner::from_slice(bytes.as_slice()).unwrap();

    let sender = signer.address();
    eprintln!("Sender address: {sender}");

    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .on_http("http://127.0.0.1:18545".parse().unwrap());

    let _counter = Counter::deploy(&provider).await.unwrap();
    // println!("Counter deployed: {counter}");

    Ok(())
}

// Codegen from embedded Solidity code and precompiled bytecode.
sol! {
    #[allow(missing_docs)]
    // solc v0.8.26; solc Counter.sol --via-ir --optimize --bin
    #[sol(rpc, bytecode="6080806040523460135760df908160198239f35b600080fdfe6080806040526004361015601257600080fd5b60003560e01c9081633fb5c1cb1460925781638381f58a146079575063d09de08a14603c57600080fd5b3460745760003660031901126074576000546000198114605e57600101600055005b634e487b7160e01b600052601160045260246000fd5b600080fd5b3460745760003660031901126074576020906000548152f35b34607457602036600319011260745760043560005500fea2646970667358221220e978270883b7baed10810c4079c941512e93a7ba1cd1108c781d4bc738d9090564736f6c634300081a0033")]
    contract Counter {
        uint256 public number;

        function setNumber(uint256 newNumber) public {
            number = newNumber;
        }

        function increment() public {
            number++;
        }
    }
}
