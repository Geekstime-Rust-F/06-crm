use anyhow::Result;
use crm_metadata::{
    pb::{metadata_client::MetadataClient, MaterializeRequest},
    AppConfig, MetadataService,
};
use std::{net::SocketAddr, time::Duration};
use tokio::time::sleep;
use tokio_stream::StreamExt;
use tonic::{transport::Server, Request};

#[tokio::test]
async fn test_metadata() -> Result<()> {
    let addr = start_server().await?;
    println!("Metadata Server listening on {}", addr);

    let mut client =
        MetadataClient::connect(format!("http://[{}]:{}", addr.ip(), addr.port())).await?;

    let messages: Vec<MaterializeRequest> = vec![
        MaterializeRequest { id: 1 },
        MaterializeRequest { id: 2 },
        MaterializeRequest { id: 3 },
    ];
    let req = Request::new(tokio_stream::iter(messages));
    let res = client.materialize(req).await?.into_inner();
    let ret = res
        .then(|x| async move { x.unwrap() })
        .collect::<Vec<_>>()
        .await;

    assert_eq!(ret.len(), 3);

    Ok(())
}

async fn start_server() -> Result<SocketAddr> {
    let config = AppConfig::load()?;
    let port = config.server.port;
    let addr = format!("[::1]:{}", port).parse().unwrap();

    let metadata_server = MetadataService::new(config).into_server();

    tokio::spawn(async move {
        Server::builder()
            .add_service(metadata_server)
            .serve(addr)
            .await
            .unwrap();
    });
    sleep(Duration::from_micros(1)).await;
    Ok(addr)
}
