// /// Handler for `fp_getValidatorSignature`.
// async fn get_validator_signature(&self, raw_tx: Bytes) -> RpcResult<Signature> {
//     debug!(target: "rpc::fp", ?raw_tx, "Serving fp_getValidatorSignature");

//     // Recover the transaction. tx.slice(..)
//     let tx_signed = recover_raw_tx(&raw_tx)?;
//     info!("tx_signed: {:?}", tx_signed);

//     let pooled_tx =
//         <Pool as TransactionPool>::Transaction::from_recovered_transaction(tx_signed.clone());

//     let hash = self
//         .pool
//         .add_transaction(TransactionOrigin::Local, pooled_tx)
//         .await
//         .map_err(|e| {
//             error!("Failed to add transaction to pool: {e}");
//             // TODO: optimize the error response.
//             ErrorCode::InternalError
//         })?;
//     debug!("Transaction added to pool: {:?}", hash);

//     // Generate the signature using the validator key.
//     let sig = match &self.keystore {
//         Some(keystore) => keystore.sign_message(&raw_tx).map_err(|e| {
//             error!("Failed to sign message: {e}");
//             RpcApiError::SigningError(e.to_string())
//         })?,
//         None => {
//             return Err(RpcApiError::KeystoreNotFound.into());
//         }
//     };

//     // TODO: get epoch id and checkpoint id from the newest state.
//     let _epoch_id = EpochId(0);
//     let _checkpoint_id = CheckpointId(0);

//     Ok(sig)
// }

// /// Handler for `fp_sendRawCertificate`.
// async fn send_raw_certificate(&self, raw_certificate: Bytes) -> RpcResult<()> {
//     debug!(target: "rpc::fp", ?raw_certificate, "Serving fp_sendRawCertificate");

//     let mut raw_bytes = raw_certificate.as_ref();
//     let certificate = TransactionCertificate::decode(&mut raw_bytes).map_err(|e| {
//         error!("Failed to decode certificate: {e}");
//         RpcApiError::TransactionCertificateDecodeError(e.to_string())
//     })?;

//     info!("certificate: {:?}", certificate);

//     let governace_cert = self
//         .datastore
//         .provider_factory()
//         .get_governance_certificate()
//         .map_err(|e| {
//             error!("Failed to get governance certificate: {e}");
//             ProviderErr::GovernaceNotFound
//         })?;
//     let agg_sig = AggregateSignature::List(certificate.countersignatures);
//     let keys = governace_cert.proposal.message.validators;
//     let () = agg_sig
//         .verify_quorum(&keys, &raw_bytes.to_vec())
//         .map_err(|e| {
//             error!("Failed to verify the certificate: {e}");
//             RpcApiError::CertificateInsufficientSignatures(e.to_string())
//         })?;

//     // TODO: apply the certificate to the state: send certificate by the channel
//     Ok(())
// }

// #[allow(unused)]
// pub async fn client_get_validator_signature(url: &str, tx: Bytes) -> Result<Signature> {
//     let client = HttpClient::builder()
//         .request_timeout(Duration::from_secs(10))
//         .build(url)?;
//     let res = client.get_validator_signature(tx).await?;
//     info!("res: {:?}", res);
//     Ok(res)
// }

// #[allow(unused)]
// pub async fn client_send_raw_certificate(url: &str, certificate: Bytes) -> Result<()> {
//     let client = HttpClient::builder()
//         .request_timeout(Duration::from_secs(10))
//         .build(url)?;
//     client.send_raw_certificate(certificate).await?;
//     Ok(())
// }

// /// The relayer should call this method to get the validator signature.
// #[method(name = "getValidatorSignature")]
// async fn get_validator_signature(&self, raw_tx: Bytes) -> RpcResult<Signature>;

// /// The relayer should call this method to send the certificate to the validator.
// ///
// /// raw_certificate is the raw bytes of the certificate encoded by rlp.
// #[method(name = "sendRawCertificate")]
// async fn send_raw_certificate(&self, raw_certificate: Bytes) -> RpcResult<()>;

#[cfg(test)]
mod tests {
    // use std::str::FromStr;

    // use super::*;
    // use alloy_rlp::Encodable;
    // use anyhow::Result;

    // #[tokio::test]
    // #[ignore]
    // async fn test_get_validator_signature() -> Result<()> {
    //     let s = "0xf8708085174876e800830186a09421dd7e6361a8edfd251ee3188b800f2a9f5c1541884563918244f4000080830191fca026f163d345b43359f00bd12085f05d4ee9ddf300611aff8d40484c081550f537a027929c7d359dd0ae0fdcecba081cfc6488d77b846780a5f40c46a033bd1aeda0";
    //     let tx = Bytes::from_str(s)?;
    //     let _res = client_get_validator_signature("http://localhost:18545", tx).await?;
    //     // println!("res: {:?}", _res);
    //     Ok(())
    // }

    // #[tokio::test]
    // #[ignore]
    // async fn test_send_raw_certificate() -> Result<()> {
    //     let s = "0xf8708085174876e800830186a09421dd7e6361a8edfd251ee3188b800f2a9f5c1541884563918244f4000080830191fca026f163d345b43359f00bd12085f05d4ee9ddf300611aff8d40484c081550f537a027929c7d359dd0ae0fdcecba081cfc6488d77b846780a5f40c46a033bd1aeda0";
    //     let raw_bytes = Bytes::from_str(s)?;
    //     let sig =
    //         client_get_validator_signature("http://localhost:18545", raw_bytes.slice(..)).await?;

    //     let tx_signed = recover_raw_tx(&raw_bytes)?;
    //     let certificate = TransactionCertificate::new(tx_signed, vec![sig]);
    //     let mut buffer = Vec::<u8>::new();
    //     certificate.encode(&mut buffer);
    //     let certificate_bytes = Bytes::copy_from_slice(&buffer);
    //     client_send_raw_certificate("http://localhost:18545", certificate_bytes).await?;
    //     // println!("res: {:?}", _res);

    //     Ok(())
    // }
}

// For debug.
// let tx_dup = self.pool.get_pooled_transaction(tx_hash);
// if let Some(tx_dup) = tx_dup {
//     let agg_sigs = tx_dup.get_aggregation_signature();

//     match agg_sigs {
//         AggregateSignature::List(agg_sigs) => {
//             assert_eq!(agg_sigs.len(), signatures.len());
//             agg_sigs.iter().for_each(|s| {
//                 info!(target: "relayer:handle", "agg_sig item: {s:?}");
//             });
//         }
//     }
// }

// let sig_verify_result = tx
//     .get_aggregation_signature()
//     .verify_quorum_with_message_hash(
//         &governance_certificate.proposal.message.validators,
//         sign_message,
//         sig_quorum,
//     );
// if let Err(e) = sig_verify_result {
//     warn!(target: "relayer:handle", "Failed to verify the certificate: {e}");
//     continue;
// }

// let mut out = Vec::new();
// message.encode(&mut out);
// let pooled_tx = PooledTransaction::decode(&mut out.as_slice())
//     .expect("should decode to PooledTransactioin");
// let signed_tx = pooled_tx.0;

/// RocksDB

// #[derive(Debug)]
// pub struct DB {
//     db: RwLock<RocksDB>,
// }

// impl DB {
//     pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DBError> {
//         let mut options = Options::default();
//         options.create_if_missing(true);
//         let db = RocksDB::open(&options, path)?;
//         info!("database initialized...");
//         Ok(DB {
//             db: RwLock::new(db),
//         })
//     }

//     pub fn put<K, V>(&self, key: K, value: V) -> Result<(), DBError>
//     where
//         K: AsRef<[u8]>,
//         V: AsRef<[u8]>,
//     {
//         match self.db.write() {
//             Ok(db) => db.put(key, value).map_err(DBError::from),
//             Err(e) => {
//                 error!("Failed to acquire write lock: {}", e);
//                 Err(DBError::RWLockError(e.to_string()))
//             }
//         }
//     }

//     pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Vec<u8>>, DBError> {
//         match self.db.read() {
//             Ok(db) => db.get(key).map_err(DBError::from),
//             Err(e) => {
//                 error!("Failed to acquire read lock: {}", e);
//                 Err(DBError::RWLockError(e.to_string()))
//             }
//         }
//     }

//     #[allow(unused)]
//     pub fn delete<K: AsRef<[u8]>>(&self, key: K) -> Result<(), DBError> {
//         match self.db.write() {
//             Ok(db) => db.delete(key).map_err(DBError::from),
//             Err(e) => {
//                 error!("Failed to acquire write lock: {}", e);
//                 Err(DBError::RWLockError(e.to_string()))
//             }
//         }
//     }
// }

// impl Drop for DB {
//     fn drop(&mut self) {
//         match self.db.write() {
//             Ok(db) => {
//                 if let Err(e) = db.flush() {
//                     info!("Failed to flush database: {}", e);
//                 }
//             }
//             Err(e) => {
//                 error!("Failed to acquire write lock for database flush: {}", e);
//             }
//         }
//     }
// }
