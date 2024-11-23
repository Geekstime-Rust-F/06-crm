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
    // let addr= "http://localhost:8080";
    // TODO fix Error: status: Internal, message: "protocol error: received message with invalid compression flag: 60 (valid flags are 0 and 1) while receiving response with status: 502 Bad Gateway", details: [], metadata: MetadataMap { headers: {"server": "nginx/1.27.2", "date": "Sat, 23 Nov 2024 08:42:01 GMT", "content-type": "text/html", "content-length": "497", "etag": "\"66fd630f-1f1\""} } when using docker, local nginx works fine
    let mut client = CrmClient::connect(addr)
        .await
        .expect("Connect to crm error");
    let welcome_request = WelcomeRequestBuilder::default()
        .id(Uuid::new_v4().to_string())
        .interval(7u32)
        .content_ids([8735])
        .build()?;

    let res = client.welcome(welcome_request).await?.into_inner();
    println!("{:?}", res);

    Ok(())
}
