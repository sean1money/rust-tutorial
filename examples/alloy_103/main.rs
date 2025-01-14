use alloy::hex::FromHex;
use alloy_primitives::{keccak256, Bytes, TxHash, U256};
use alloy_sol_types::{sol, SolCall};

use anyhow::Result;

fn main() -> Result<()> {
    println!("Hello, alloy sol types!");

    let hash =
        TxHash::from_hex("0xe26badb92d901794ae7efb028dab03e48a0ff22b2fbd5ac3ccb60bbef499c9e3")
            .unwrap();
    let call = builtinCancellationCall { hash };
    println!("{:?}", builtinCancellationCall::SIGNATURE);
    println!("{:?}", builtinCancellationCall::SELECTOR);
    println!("{:?}", hex::encode(call.abi_encode()));

    println!("{:?}", builtinAccountRecoveryCall::SIGNATURE);
    println!("{:?}", builtinAccountRecoveryCall::SELECTOR);

    println!("{:?}", keccak256(builtinAccountRecoveryCall::SIGNATURE));
    println!("0x{}", hex::encode(builtinAccountRecoveryCall::SELECTOR));

    let hash1 =
        TxHash::from_hex("0xe26badb92d901794ae7efb028dab03e48a0ff22b2fbd5ac3ccb60bbef499c9e3")
            .unwrap();
    let sig1 = Bytes::from_hex("0x000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000000e01").unwrap();
    let sig2 = Bytes::from_hex("0x000000000000000000000000000000000000000000000000000000000000000f000000000000000000000000000000000000000000000000000000000000000f01").unwrap();
    let half_tx1 = HalfTx {
        hash: hash1,
        signatures: vec![sig1, sig2],
    };

    let hash2 =
        TxHash::from_hex("0x97e48664d5efb42f1a48b5934770442767ac2381042871a77adb2b85ac32c9b0")
            .unwrap();
    let sig1 = Bytes::from_hex("0x00000000000000000000000000000000000000000000000000000000000000aa00000000000000000000000000000000000000000000000000000000000000cc01").unwrap();
    let sig2 = Bytes::from_hex("0x00000000000000000000000000000000000000000000000000000000000000bb00000000000000000000000000000000000000000000000000000000000000dd01").unwrap();
    let half_tx2 = HalfTx {
        hash: hash2,
        signatures: vec![sig1, sig2],
    };

    let call: builtinAccountRecoveryCall = builtinAccountRecoveryCall {
        halfTxs: vec![half_tx1, half_tx2],
    };

    let encoded = call.abi_encode();
    println!("{:?}", encoded);
    println!("{:?}", encoded.len());
    println!("{:?}", hex::encode(encoded));

    let abi_string = "625234b1000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000";
    let decoded = builtinAccountRecoveryCall::abi_decode(&hex::decode(abi_string)?, true);
    println!("{:?}", decoded);

    println!("{:?}", systemGovernanceCall::SIGNATURE);
    println!("{:?}", systemGovernanceCall::SELECTOR);
    println!("0x{}", hex::encode(systemGovernanceCall::SELECTOR));

    let balance = balanceOfRet(U256::from(1000));
    println!("{:?}", hex::encode(balance.abi_encode()));
    println!("{:?}", hex::encode(balance.abi_encode_packed()));

    Ok(())
}

sol!(
    #[allow(missing_docs)]
    #[derive(Debug, PartialEq, Eq)]
    function getRoundData(uint80 _roundId) external view returns (uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound);
);

sol! {
    type balanceOfRet is uint256;

    #[sol(rpc = true)]
    function balanceOf(address account) external view returns (balanceOfRet);
}

sol! {
    #[derive(Debug, PartialEq, Eq)]
    function builtinCancellation(bytes32 hash);
}

// Recovery function for a half-signed transaction.
sol! {
    #[allow(missing_docs)]
    #[derive(Debug, PartialEq, Eq)]
    struct HalfTx {
        bytes32 hash;
        bytes[] signatures;
    }

    #[allow(missing_docs)]
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc = true, abi = true)]
    function builtinAccountRecovery(HalfTx[] halfTxs);
}

// Governance function.
sol! {
    struct Governance {
        uint64 epoch;
        bytes operator;
        address mint;

        Parameters parameters;
        string version;
        Validator[] validators;
        string banned;
    }

    struct Parameters {
        uint128 txFee;
        uint128 mintFee;
        uint128 burnFee;
        uint32 minTxToCheckpoint;
        uint32 operatorFeeShare;
    }

    struct Validator {
        bytes publicKey;
        address account;
        string ip;
        bool archive;
    }

    function systemGovernance(Governance governance);
    function systemSetParameters(Parameters parameters);

    function systemAppendValidator(Validator validator);
    function systemReplaceAllValidators(Validator[] validators);

    function systemUpdateBanned(string newBanned, bool clearExisting);
    function systemAddBannedAccounts(address[] accounts);
}
