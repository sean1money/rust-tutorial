use anyhow::Result;
use secp256k1::{
    ecdsa::Signature,
    hashes::{sha256, Hash},
    rand::{self, rngs::OsRng},
    Message, PublicKey, Secp256k1, SecretKey, Signing,
};

fn main() -> Result<()> {
    generate_key()?;

    sign_and_verify()?;

    Ok(())
}

/// Generate key.
fn generate_key() -> Result<()> {
    let secp = Secp256k1::new();

    // Option 1
    // rand 1
    let mut rng = rand::thread_rng();
    let (sk, pk) = secp.generate_keypair(&mut rng);
    assert_eq!(pk, PublicKey::from_secret_key(&secp, &sk));

    // rand 2
    let (sk2, pk2) = secp.generate_keypair(&mut OsRng);
    assert_eq!(pk2, PublicKey::from_secret_key(&secp, &sk2));

    // Option 2
    let sk = SecretKey::new(&mut rng);
    let _ = PublicKey::from_secret_key(&secp, &sk);
    Ok(())
}

fn sign_and_verify() -> Result<()> {
    let secp = Secp256k1::new();

    let seckey = [
        59, 148, 11, 85, 134, 130, 61, 253, 2, 174, 59, 70, 27, 180, 51, 107, 94, 203, 174, 253,
        102, 39, 170, 146, 46, 252, 4, 143, 236, 12, 136, 28,
    ];
    let _pubkey = [
        2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41, 91, 141,
        134, 245, 114, 45, 63, 82, 19, 251, 210, 57, 79, 54,
    ];
    let message = b"hello world";

    let sig = sign_message(&secp, message, seckey)?;

    let compact_sig = sig.serialize_compact();
    let der_sig = sig.serialize_der();

    println!("compact sig: {:?}", compact_sig);
    println!("der sig: {:?}", der_sig);

    Ok(())
}

fn sign_message<C: Signing>(
    secp: &Secp256k1<C>,
    message: &[u8],
    sk: [u8; 32],
) -> Result<Signature> {
    let message = sha256::Hash::hash(message);
    let message = Message::from_digest_slice(message.as_ref())?;
    let secret_key = SecretKey::from_slice(&sk)?;
    let r = secp.sign_ecdsa(&message, &secret_key);
    Ok(r)
}
