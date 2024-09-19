use std::sync::Arc;

use anyhow::Result;

fn main() -> Result<()> {
    let inner = EthApiInner::default();
    let eth_api = EthApi::new(inner);

    let _chain_info = EthApiSpec::chain_info(&eth_api)?;
    Ok(())
}

#[auto_impl::auto_impl(&, Arc)]
pub trait EthApiSpec: Send + Sync {
    fn chain_info(&self) -> Result<ChainInfo>;
}

impl EthApiSpec for EthApi {
    fn chain_info(&self) -> Result<ChainInfo> {
        Ok(ChainInfo {
            best_number: 42,
            best_hash: "0xdeadbeef".to_string(),
        })
    }
}

pub struct EthApi {
    pub inner: Arc<EthApiInner>,
}

impl EthApi {
    pub fn new(inner: EthApiInner) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

#[derive(Debug, Default)]
pub struct EthApiInner {}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct ChainInfo {
    pub best_number: u64,
    pub best_hash: String,
}
