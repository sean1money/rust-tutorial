use alloy_rlp::{Decodable, Encodable, RlpDecodable, RlpEncodable};

#[derive(Debug, RlpEncodable, RlpDecodable)]
pub struct Payment {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

#[derive(Debug, RlpEncodable, RlpDecodable)]
pub struct AccountRecovery {
    pub address: String,
    pub nonce: u64,
}

#[derive(Debug)]
pub enum Transaction {
    Payment(Payment),
    AccountRecovery(AccountRecovery),
}

impl Encodable for Transaction {
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        match self {
            Transaction::Payment(payment) => {
                out.put_u8(0 as u8);
                payment.encode(out);
            }
            Transaction::AccountRecovery(account_recovery) => {
                out.put_u8(1 as u8);
                account_recovery.encode(out);
            }
        }
    }
}

impl Decodable for Transaction {
    fn decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let tx_type = buf[0];
        *buf = &buf[1..];

        match tx_type {
            0 => {
                let payment = Payment::decode(buf)?;
                Ok(Transaction::Payment(payment))
            }
            1 => {
                let account_recovery = AccountRecovery::decode(buf)?;
                Ok(Transaction::AccountRecovery(account_recovery))
            }
            _ => Err(alloy_rlp::Error::Custom("invalid tx type")),
        }
    }
}

#[derive(Debug, RlpEncodable, RlpDecodable)]
pub struct Signed {
    pub tx: Transaction,
    pub verified_sig: String,
}

#[derive(Debug, RlpEncodable, RlpDecodable)]
pub struct Certificate {
    pub tx: Signed,
    pub sigs: Vec<String>,
}

fn main() {
    let certificate = Certificate {
        tx: Signed {
            tx: Transaction::Payment(Payment {
                from: "0x123".to_string(),
                to: "0x456".to_string(),
                amount: 100,
            }),
            verified_sig: "0x789".to_string(),
        },
        sigs: vec!["0xabc".to_string(), "0xdef".to_string()],
    };

    let mut buffer = Vec::<u8>::new();

    certificate.encode(&mut buffer);
    let hex_cert = hex::encode(buffer.as_slice());
    println!("encoded certificate hex: {:?}", buffer.as_slice());
    println!("encoded certificate hex: {}", hex_cert);

    let res = Certificate::decode(&mut buffer.as_slice());
    match res {
        Ok(decoded_cert) => {
            println!("decoded certificate: {:?}", decoded_cert);
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }

    let certificate = Certificate {
        tx: Signed {
            tx: Transaction::AccountRecovery(AccountRecovery {
                address: "0x123".to_string(),
                nonce: 100,
            }),
            verified_sig: "0x789".to_string(),
        },
        sigs: vec!["0xabc".to_string(), "0xdef".to_string()],
    };

    let mut buffer = Vec::<u8>::new();
    certificate.encode(&mut buffer);
    let hex_cert = hex::encode(buffer.as_slice());
    println!("encoded certificate hex: {:?}", buffer.as_slice());
    println!("encoded certificate hex: {}", hex_cert);

    let res = Certificate::decode(&mut buffer.as_slice());
    match res {
        Ok(decoded_cert) => {
            println!("decoded certificate: {:?}", decoded_cert);
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }
}
