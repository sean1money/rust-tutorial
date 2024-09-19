use std::{fmt::Debug, future::Future, sync::Arc};

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}

struct TxSigned {
    hash: String,
    sender: String,
}

impl TxSigned {
    fn new(hash: String, sender: String) -> Self {
        Self { hash, sender }
    }
}

impl From<TxSigned> for PooledTransaction {
    fn from(tx: TxSigned) -> Self {
        PooledTransaction::new(tx.hash, tx.sender)
    }
}

struct EthApi<Pool> {
    inner: Arc<EthApiInner<Pool>>,
}

impl<Pool> EthApi<Pool>
where
    Pool: TransactionPool + 'static,
{
    async fn send_raw_transaction(&self, tx: TxSigned) -> Result<()> {
        let inner = self.inner.clone();

        let pooled_tx =
            <Pool as TransactionPool>::Transaction::from_recovered_pooled_transaction(tx);
        inner.pool.add_transaction(pooled_tx).await?;
        Ok(())
    }
}

struct EthApiInner<Pool> {
    pool: Pool,
}

trait TransactionPool {
    type Transaction: PoolTransaction;

    fn add_transaction(&self, tx: Self::Transaction) -> impl Future<Output = Result<()>> + Send;
}

struct Pool<V> {
    inner: Arc<PoolInner<V>>,
}

impl<V> Pool<V>
where
    V: TransactionValidator,
{
    fn new(validator: V) -> Self {
        Self {
            inner: Arc::new(PoolInner::new(validator)),
        }
    }
}

impl<V> TransactionPool for Pool<V>
where
    V: TransactionValidator,
{
    type Transaction = V::Transaction;

    fn add_transaction(&self, tx: Self::Transaction) -> impl Future<Output = Result<()>> + Send {
        let inner = self.inner.clone();
        async move {
            let validator = &inner.validator;
            validator.validate(tx).await?;
            Ok(())
        }
    }
}

struct PoolInner<V> {
    validator: V,
}

impl<V> PoolInner<V> {
    pub fn new(validator: V) -> Self {
        Self { validator }
    }
}

trait PoolTransaction: Debug + Send + Sync + FromRecoveredPooledTransaction {
    fn hash(&self) -> String;
    fn sender(&self) -> String;
}

trait FromRecoveredPooledTransaction {
    fn from_recovered_pooled_transaction(tx: TxSigned) -> Self;
}

impl FromRecoveredPooledTransaction for PooledTransaction {
    fn from_recovered_pooled_transaction(tx: TxSigned) -> Self {
        PooledTransaction::new(tx.hash, tx.sender)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PooledTransaction {
    hash: String,
    sender: String,
}

impl PooledTransaction {
    fn new(hash: String, sender: String) -> Self {
        Self { hash, sender }
    }
}

impl PoolTransaction for PooledTransaction {
    fn hash(&self) -> String {
        self.hash.clone()
    }

    fn sender(&self) -> String {
        self.sender.clone()
    }
}

/// Validator
trait TransactionValidator: Send + Sync {
    type Transaction: PoolTransaction;

    fn validate(&self, tx: Self::Transaction) -> impl Future<Output = Result<bool>> + Send;
}

struct DefaultTxValidator<S> {
    inner: Arc<DefaultTxValidatorInner<S>>,
}

impl<S> DefaultTxValidator<S> {
    fn new(state: S) -> Self {
        Self {
            inner: Arc::new(DefaultTxValidatorInner { state }),
        }
    }
}

struct DefaultTxValidatorInner<S> {
    state: S,
}

impl<S> TransactionValidator for DefaultTxValidator<S>
where
    S: AccountState,
{
    type Transaction = PooledTransaction;

    fn validate(&self, tx: Self::Transaction) -> impl Future<Output = Result<bool>> + Send {
        async move {
            println!("tx: {:?}", tx);
            Ok(true)
        }
    }
}

trait AccountState: Sync + Send {
    fn get_balance(&self, account: &str) -> u64;
}

struct DefaultAccountState;

impl AccountState for DefaultAccountState {
    fn get_balance(&self, account: &str) -> u64 {
        0
    }
}
