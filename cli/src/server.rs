use std::net::SocketAddr;

use anyhow::Result;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::Response;
use axum::routing::get;
use axum::{Json, Router};
use clap::Args;
use libp2p::Multiaddr;
use lumina_node::network::canonical_network_bootnodes;
use rust_embed::RustEmbed;
use serde::Serialize;
use tokio::net::TcpListener;
use tracing::info;

use crate::common::ArgNetwork;

const SERVER_DEFAULT_BIND_ADDR: &str = "127.0.0.1:9876";

#[derive(Debug, Clone, Serialize)]
struct WasmNodeArgs {
    pub network: ArgNetwork,
    pub bootnodes: Vec<Multiaddr>,
}

#[derive(RustEmbed)]
#[folder = "$WASM_NODE_OUT_DIR"]
struct WasmPackage;

#[derive(RustEmbed)]
#[folder = "static"]
struct StaticResources;

#[derive(Debug, Args)]
pub(crate) struct Params {
    /// Network to connect.
    #[arg(short, long, value_enum, default_value_t)]
    pub(crate) network: ArgNetwork,

    /// Listening addresses. Can be used multiple times.
    #[arg(short, long = "listen", default_value = SERVER_DEFAULT_BIND_ADDR)]
    pub(crate) listen_addr: SocketAddr,

    /// Bootnode multiaddr, including peer id. Can be used multiple times.
    #[arg(short, long = "bootnode")]
    pub(crate) bootnodes: Vec<Multiaddr>,
}

pub(crate) async fn run(args: Params) -> Result<()> {
    let network = args.network.into();
    let bootnodes = if args.bootnodes.is_empty() {
        canonical_network_bootnodes(network).collect()
    } else {
        args.bootnodes
    };

    let state = WasmNodeArgs {
        network: args.network,
        bootnodes,
    };

    let app = Router::new()
        .route("/", get(serve_index_html))
        .route("/js/*path", get(serve_embedded_path::<StaticResources>))
        .route("/wasm/*path", get(serve_embedded_path::<WasmPackage>))
        .route("/cfg.json", get(serve_config))
        .with_state(state);

    info!("listening on {}", args.listen_addr);
    let listener = TcpListener::bind(&args.listen_addr).await?;

    Ok(axum::serve(listener, app.into_make_service()).await?)
}

async fn serve_index_html() -> Result<Response, StatusCode> {
    serve_embedded_path::<StaticResources>(Path("index.html".to_string())).await
}

async fn serve_embedded_path<Source: RustEmbed>(
    Path(path): Path<String>,
) -> Result<Response, StatusCode> {
    if let Some(content) = Source::get(&path) {
        let mime = mime_guess::from_path(&path).first_or_octet_stream();
        Ok(Response::builder()
            .header(header::CONTENT_TYPE, mime.as_ref())
            .body(Body::from(content.data))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn serve_config(state: State<WasmNodeArgs>) -> Json<WasmNodeArgs> {
    Json(state.0)
}
