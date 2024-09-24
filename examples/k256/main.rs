use anyhow::Result;
use hex_literal::hex;
use k256::ecdsa::signature::Verifier;
use k256::ecdsa::RecoveryId;
use k256::ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey};
use k256::{PublicKey, SecretKey};
// use sha2::Sha256;
use sha3::{Digest, Keccak256};

use rand_core::OsRng;

fn main() -> Result<()> {
    public_key_serialize()?;
    sign_and_verify()?;
    sign_and_verify_with_recid()?;
    Ok(())
}

fn sign_and_verify() -> Result<()> {
    let sk = SigningKey::random(&mut OsRng);
    let message = b"ECDSA proves knowledge of a secret number in the context of a single message";

    let signature: Signature = sk.sign(message);

    let verifying_key = VerifyingKey::from(&sk);
    assert!(verifying_key.verify(message, &signature).is_ok());

    Ok(())
}

fn sign_and_verify_with_recid() -> Result<()> {
    // let sk = SecretKey::random(&mut OsRng);
    // let pk = sk.public_key();

    let signing_key = SigningKey::from_bytes(
        &hex!("4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318").into(),
    )?;

    let msg = hex!(
        "e9808504e3b29200831e848094f0109fc8df283027b6285cc889f5aa624eac1f55843b9aca0080018080"
    );

    let digest = Keccak256::new_with_prefix(msg);

    let (signature, recid): (Signature, RecoveryId) =
        signing_key.sign_digest_recoverable(digest)?;
    println!("recid: {:?}", recid);
    println!("signature: {:?}", signature);

    println!("{:?}", signature.to_bytes().as_slice());

    let sig = Signature::try_from(signature.to_bytes().as_slice())?;
    let recid_recovered = RecoveryId::try_from(recid.to_byte())?;

    let vk_recovered: VerifyingKey =
        VerifyingKey::recover_from_digest(Keccak256::new_with_prefix(msg), &sig, recid_recovered)?;
    println!("vk_recovered: {:?}", vk_recovered);

    let sec1_bytes = vk_recovered.to_sec1_bytes();
    println!("sec1_bytes: {:?}", hex::encode(sec1_bytes));

    let public_key = PublicKey::from(vk_recovered);
    println!("public_key: {:?}", hex::encode(public_key.to_sec1_bytes()));

    let vk_excepted = signing_key.verifying_key();
    assert_eq!(&vk_recovered, vk_excepted);

    Ok(())
}

fn public_key_serialize() -> Result<()> {
    // SingingKey from bytes and to bytes.
    let signing_key = SigningKey::from_bytes(
        &hex!("6d60143e929f5498d94e2c2c20fb8bafbbfa7fa94dc36ac35a9ffb37798605b8").into(),
    )?;
    let signing_key_bytes = signing_key.to_bytes();
    println!(
        ">> signing key bytes: {:?}, hex format: {}",
        signing_key_bytes,
        hex::encode(signing_key_bytes)
    );

    // SecretKey from bytes and to bytes.
    let secret_key = SecretKey::from_bytes(&signing_key_bytes)?;
    let secret_key_bytes = secret_key.to_bytes();
    println!(
        ">> secret key bytes: {:?}, hex format: {}",
        secret_key_bytes,
        hex::encode(secret_key_bytes)
    );

    // VerifyingKey to encoded point.
    let verifying_key: &VerifyingKey = signing_key.verifying_key();
    let compressed_vk = verifying_key.to_encoded_point(true);
    let uncompressed_vk = verifying_key.to_encoded_point(false);

    println!(
        ">> compressed verifying key: {:?}, hex format: {}",
        compressed_vk.as_bytes(),
        hex::encode(compressed_vk.as_bytes())
    );

    println!(
        ">> uncompressed verifying key: {:?}, hex format: {}",
        uncompressed_vk.as_bytes(),
        hex::encode(uncompressed_vk.as_bytes())
    );

    // VerifyingKey from sec1 bytes.
    let vk_compressed_bytes =
        &hex!("02c0e6a94ba9346c8fcc67f7a6bf77a374e986a65160557b11d96abb6298f16240");
    let recovered_vk = VerifyingKey::from_sec1_bytes(vk_compressed_bytes)?;
    println!(">> recovered verifying key: {:?}", recovered_vk);
    assert_eq!(&recovered_vk, verifying_key);

    let recoverd_pk = PublicKey::from_sec1_bytes(vk_compressed_bytes)?;
    println!(">> recovered public key: {:?}", recoverd_pk);

    println!("--------------------------------------");

    Ok(())
}
