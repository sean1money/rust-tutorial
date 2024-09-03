use std::net::SocketAddr;
use std::time::Duration;

use anyhow::Result;
use dashmap::DashMap;
use eth::{Block, EthApiServer, EthApiServerImpl};
use hyper::body::Bytes;
use hyper::Method;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::server::Server;
use jsonrpsee::{core::client::ClientT, rpc_params};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;

mod eth;

#[tokio::main]
async fn main() -> Result<()> {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()?
        .add_directive("jsonrpsee[method_call{name = \"eth_getBlockByNumber\"}]=DEBUG".parse()?);
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .finish()
        .try_init()?;

    let server_addr = run_server().await?;
    info!("[rpc server] Server started at {}\n\n", server_addr);

    // Test the server with a client.
    let url = format!("http://{}", server_addr);
    let middleware = tower::ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .on_request(|request: &hyper::Request<_>, _span: &tracing::Span| tracing::info!(?request, "on_request"))
            .on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
                tracing::info!(size_bytes = chunk.len(), latency = ?latency, "on_body_chunk")
            })
            .make_span_with(DefaultMakeSpan::new().include_headers(true))
            .on_response(DefaultOnResponse::new().include_headers(true).latency_unit(LatencyUnit::Micros)),
    );
    let client = HttpClient::builder()
        .set_http_middleware(middleware)
        .build(url)?;

    let params = rpc_params!("latest", false);
    let resp: Result<Option<Block>, _> = client.request("eth_getBlockByNumber", params).await;
    info!("===> [rpc client] Received response: {:?}\n\n", resp);

    let params = rpc_params!("safe", false);
    let resp: Result<Option<Block>, _> = client.request("eth_getBlockByNumber", params).await;
    info!("===> [rpc client] Received response: {:?}\n\n", resp);

    let params = rpc_params!("10000", false);
    let resp: Result<Option<Block>, _> = client.request("eth_getBlockByNumber", params).await;
    info!("===> [rpc client] Received response: {:?}\n\n", resp);

    Ok(())
}

async fn run_server() -> Result<SocketAddr> {
    let cors = CorsLayer::new()
        // Allow only POST requests.
        .allow_methods([Method::POST])
        // Allow requests from any origin.
        .allow_origin(Any)
        // .allow_headers([hyper::header::CONTENT_TYPE]);
        .allow_headers(Any);
    let middleware = tower::ServiceBuilder::new().layer(cors);

    let server = Server::builder()
        .set_http_middleware(middleware)
        .build("127.0.0.1:3000".parse::<SocketAddr>()?)
        .await?;

    // Dummy data.
    let state = build_state();
    let module = EthApiServerImpl::new(state).into_rpc();

    let addr = server.local_addr()?;
    let srv_handle = server.start(module);

    tokio::spawn(srv_handle.stopped());

    Ok(addr)
}

/// Build a dummy state.
fn build_state() -> DashMap<String, Block> {
    let state = DashMap::new();
    state.insert(
        "10000".to_string(),
        Block {
            hash: "0x10000hash".to_string(),
            number: 10000,
        },
    );

    state.insert(
        "latest".to_string(),
        Block {
            hash: "0xlatesthash".to_string(),
            number: 10001,
        },
    );

    state
}
