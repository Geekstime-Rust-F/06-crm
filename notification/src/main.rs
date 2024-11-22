use anyhow::Result;
use notification::{AppConfig, NotificationService};
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
    info!("Notification Server listening on {}", addr);

    let notification_server = NotificationService::default().into_service();

    Server::builder()
        .add_service(notification_server)
        .serve(addr)
        .await?;

    Ok(())
}
