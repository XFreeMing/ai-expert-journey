use axum::{routing::get, Router};
use clap::Parser;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Enterprise AI Platform Gateway — see docs/adr/009-platform.md
#[derive(Parser)]
#[command(name = "ai-platform-gateway", version = "0.1.0", about = "Enterprise AI Platform Gateway")]
struct Cli {
    /// Bind address
    #[arg(short, long, default_value = "0.0.0.0:8000")]
    bind: String,

    /// RAG API URL
    #[arg(long, default_value = "http://localhost:8001")]
    rag_api_url: String,

    /// Agent Orchestrator URL
    #[arg(long, default_value = "http://localhost:8002")]
    agent_api_url: String,

    /// RecSys API URL
    #[arg(long, default_value = "http://localhost:8003")]
    recsys_api_url: String,

    /// Multimodal API URL
    #[arg(long, default_value = "http://localhost:8005")]
    multimodal_api_url: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    let app = Router::new().route("/", get(|| async { "AI Platform Gateway v0.1.0" }));

    let addr: SocketAddr = cli
        .bind
        .parse()
        .expect("Invalid bind address");

    tracing::info!("Platform Gateway listening on {}", addr);
    tracing::info!("RAG API: {}", cli.rag_api_url);
    tracing::info!("Agent API: {}", cli.agent_api_url);

    // TODO: Full implementation pending — see docs/adr/009-platform.md
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
