use alloy::{
    consensus::TxEnvelope,
    network::{EthereumWallet, TransactionBuilder},
    primitives::{address, U256},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};

#[tokio::main]
async fn main() {
    println!("Alloy 101 example!");
    reocver_sender_address().await;
}

async fn reocver_sender_address() {
    let to = address!("d8da6bf26964af9d7eed9e03e53415d37aa96045");

    let tx = TransactionRequest::default()
        .with_to(to)
        .with_value(U256::from(10_000))
        .with_nonce(0)
        .with_chain_id(1)
        .with_gas_limit(21_000)
        .with_gas_price(10_000_000_000);

    let bytes =
        hex::decode("8178ebd5ed69413f50fda1710254350be874b3020319ce7b9b8e38810f358b1b").unwrap();
    let signer = PrivateKeySigner::from_slice(bytes.as_slice()).unwrap();

    let sender = signer.address();
    println!("Sender address: {sender}");

    let wallet = EthereumWallet::from(signer);
    let tx_envelop = tx.build(&wallet).await.unwrap();

    let sender = tx_envelop.recover_signer().unwrap();
    println!("Reocvered Sender address: {sender}");

    match tx_envelop {
        TxEnvelope::Legacy(signed_tx) => {
            let signature_hash = signed_tx.signature_hash();
            println!("Recovered signature hash: {signature_hash}");

            let tx_hash = signed_tx.hash();
            println!("Recovered tx hash: {tx_hash}");

            let signature = signed_tx.clone().signature().to_owned();
            println!("Recovered signature: {signature:?}");

            let recovered_sender = signature.recover_address_from_msg(tx_hash).unwrap();
            println!("Recovered sender using tx hash: {recovered_sender}");

            let recovered_sender_from_prehash = signature
                .recover_address_from_prehash(&signature_hash)
                .unwrap();
            println!("Recovered sender using signature hash: {recovered_sender_from_prehash}");
        }
        _ => {
            println!("Unsupported envelope: {:?}", tx_envelop);
        }
    }
}

// impl TryFrom<TxEnvelope> for SignedTransaction<Signature> {
//     type Error = InvalidTransactionError;

//     fn try_from(envelope: TxEnvelope) -> Result<Self, Self::Error> {
//         let sender_signature = signature_from_envelope(envelope.clone())?;
//         let vk = sender_signature.recover_from_prehash(envelope.tx_hash())?;
//         let compressed_vk = vk.to_encoded_point(true).as_bytes().to_vec();

//         let sender = envelope.recover_signer()?;
//         let tx_hash = envelope.tx_hash().to_owned();

//         let TxEnvelope::Legacy(recovered) = envelope.clone() else {
//             return Err(InvalidTransactionError::TxTypeNotUnsupported);
//         };

//         let recipient = match recovered.tx().to().to() {
//             Some(recipient) => recipient.to_owned(),
//             None => return Err(PrimitivesError::InvalidToAddress),
//         };

//         let nonce = recovered.tx().nonce();
//         let amount = recovered.tx().value();
//         let money = Money::from(amount);
//         let input = recovered.tx().input();

//         let chain_id = recovered
//             .tx()
//             .chain_id()
//             .ok_or(InvalidTransactionError::ChainIdNotFound)?;

//         let payment = Payment::new(sender, chain_id, nonce, recipient, money);
//         let payment = payment.with_input_data(input.to_owned());
//         let tx = Message::from(payment);

//         let unverified_transaction = UnverifiedTransaction::new_with_eth_tx(chain_id, envelope)?;

//         let signed = SignedTransaction::try_new(unverified_transaction)?;
//         Ok(signed)
//     }
// }

// impl TryFrom<TxEnvelope> for Message {
//     type Error = InvalidTransactionError;

//     fn try_from(envelope: TxEnvelope) -> Result<Self, Self::Error> {
//         let sender_signature = signature_from_envelope(envelope.clone())?;
//         let vk = sender_signature.recover_from_prehash(envelope.tx_hash())?;
//         let compressed_vk = vk.to_encoded_point(true).as_bytes().to_vec();

//         let sender = envelope.recover_signer()?;

//         let TxEnvelope::Legacy(recovered) = envelope else {
//             return Err(InvalidTransactionError::TxTypeNotUnsupported);
//         };

//         let recipient = match recovered.tx().to().to() {
//             Some(recipient) => recipient.to_owned(),
//             None => return Err(InvalidTransactionError::InvalidToAddress),
//         };

//         let nonce = recovered.tx().nonce();
//         let amount = recovered.tx().value();
//         let money = Money::from(amount);
//         let input = recovered.tx().input();

//         let chain_id = recovered
//             .tx()
//             .chain_id()
//             .ok_or(InvalidTransactionError::ChainIdNotFound)?;

//         let payment = Payment::new(sender, chain_id, nonce, recipient, money);
//         let payment = payment.with_input_data(input.to_owned());

//         Ok(Message::from(payment))
//     }
// }
