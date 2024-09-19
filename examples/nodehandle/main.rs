use std::{
    future::Future,
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::Result;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    let node_handle = NodeHandle {
        node_exit_future: NodeExitFuture { terminate: false },
    };

    node_handle.wait_for_node_exit().await?;

    Ok(())
}

#[allow(dead_code)]
async fn exit_node(_node_handle: Arc<Mutex<NodeHandle>>) {
    sleep(Duration::from_secs(5)).await;
    // node_handle.lock().unwrap().node_exit_future.terminate = true;
}

#[derive(Debug)]
struct NodeHandle {
    node_exit_future: NodeExitFuture,
}

impl NodeHandle {
    pub async fn wait_for_node_exit(self) -> Result<()> {
        self.node_exit_future.await
    }
}

#[derive(Debug, Clone, Copy)]
struct NodeExitFuture {
    terminate: bool,
}

impl Future for NodeExitFuture {
    type Output = Result<()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.terminate {
            std::task::Poll::Ready(Ok(()))
        } else {
            std::task::Poll::Pending
        }
    }
}
