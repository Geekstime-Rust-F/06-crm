use anyhow::Result;
use crm::{
    pb::{crm_client::CrmClient, WelcomeRequestBuilder},
    AppConfig,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load().unwrap();
    let addr = format!("http://[::1]:{}", config.server.port);
    let mut client = CrmClient::connect(addr).await?;
    let welcome_request = WelcomeRequestBuilder::default()
        .id(Uuid::new_v4().to_string())
        .interval(7u32)
        .content_ids([8735])
        .build()?;

    let res = client.welcome(welcome_request).await?.into_inner();
    println!("{:?}", res);

    Ok(())
}
