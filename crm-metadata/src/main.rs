use anyhow::Result;
use crm_metadata::{AppConfig, MetadataService};
use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::{
    filter::LevelFilter, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _,
};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load()?;
    let port = config.server.port;
    let addr = format!("[::1]:{}", port).parse().unwrap();
    info!("Metadata Server listening on {}", addr);

    let metadata_server = MetadataService::new(config).into_server();

    Server::builder()
        .add_service(metadata_server)
        .serve(addr)
        .await?;

    Ok(())
}
