use std::str::FromStr;

use dashmap::DashMap;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use serde::{Deserialize, Deserializer, Serialize};
use tracing::info;

#[rpc(client, server, namespace = "eth")]
pub trait EthApi {
    #[method(name = "getBlockByNumber")]
    async fn block_by_number(
        &self,
        number: BlockNumberOrTag,
        full: bool,
    ) -> RpcResult<Option<Block>>;
}

#[derive(Debug, Clone)]
pub struct EthApiServerImpl {
    state: DashMap<String, Block>,
}

impl EthApiServerImpl {
    pub fn new(state: DashMap<String, Block>) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl EthApiServer for EthApiServerImpl {
    async fn block_by_number(
        &self,
        number: BlockNumberOrTag,
        full: bool,
    ) -> RpcResult<Option<Block>> {
        info!(
            "[rpc server] eth_getBlockByNumber params: {:?} {:?}, number: {:?}",
            number,
            full,
            number.as_number()
        );

        let block = match number {
            BlockNumberOrTag::Number(n) => {
                if let Some(block) = self.state.get(&n.to_string()) {
                    Some(block.clone())
                } else {
                    None
                }
            }
            BlockNumberOrTag::Latest => self.state.get(&"latest".to_string()).map(|b| b.clone()),
            _ => None,
        };

        Ok(block)
    }
}

/// Block information.
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Block {
    pub(crate) hash: String,
    pub(crate) number: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum BlockNumberOrTag {
    #[default]
    Latest,
    Finalized,
    Safe,
    Earliest,
    Pending,
    Number(u64),
}

impl BlockNumberOrTag {
    pub fn as_number(&self) -> Option<u64> {
        match *self {
            Self::Number(n) => Some(n),
            _ => None,
        }
    }
}

impl FromStr for BlockNumberOrTag {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "latest" => Ok(BlockNumberOrTag::Latest),
            "finalized" => Ok(BlockNumberOrTag::Finalized),
            "safe" => Ok(BlockNumberOrTag::Safe),
            "earliest" => Ok(BlockNumberOrTag::Earliest),
            "pending" => Ok(BlockNumberOrTag::Pending),
            _ => {
                if let Ok(number) = s.parse::<u64>() {
                    Ok(BlockNumberOrTag::Number(number))
                } else {
                    Err(anyhow::anyhow!("Invalid BlockNumberOrTag"))
                }
            }
        }
    }
}

impl<'de> Deserialize<'de> for BlockNumberOrTag {
    fn deserialize<D>(deserializer: D) -> Result<BlockNumberOrTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl Serialize for BlockNumberOrTag {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::Number(n) => serializer.serialize_str(&format!("0x{n:x}")),
            Self::Latest => serializer.serialize_str("latest"),
            Self::Finalized => serializer.serialize_str("finalized"),
            Self::Safe => serializer.serialize_str("safe"),
            Self::Earliest => serializer.serialize_str("earliest"),
            Self::Pending => serializer.serialize_str("pending"),
        }
    }
}
