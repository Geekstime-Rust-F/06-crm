use anyhow::Result;
use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::{
    filter::LevelFilter, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _,
};
use user_stat::{AppConfig, UserStatsServer};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load()?;
    let port = config.server.port;
    let addr = format!("[::1]:{}", port).parse().unwrap();
    info!("UserStats Server listening on {}", addr);

    let user_server = UserStatsServer::new(config).await?.into_service();

    Server::builder()
        .add_service(user_server)
        .serve(addr)
        .await?;

    Ok(())
}
