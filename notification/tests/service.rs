use std::{net::SocketAddr, time::Duration};

use anyhow::Result;
use futures::StreamExt;
use notification::{
    pb::{
        notification_client::NotificationClient, EmailMessage, InAppMessage, SendRequest,
        SmsMessage,
    },
    AppConfig, NotificationService,
};
use tokio::time::sleep;
use tonic::{transport::Server, Request};

#[tokio::test]
async fn send_should_work() -> Result<()> {
    let addr = start_server().await?;

    let mut client =
        NotificationClient::connect(format!("http://[{}]:{}", addr.ip(), addr.port())).await?;

    let messages: Vec<SendRequest> = vec![
        EmailMessage::fake().into(),
        SmsMessage::fake().into(),
        InAppMessage::fake().into(),
    ];
    let req = Request::new(tokio_stream::iter(messages));
    let res = client.send(req).await?.into_inner();
    let ret = res
        .then(|x| async move { x.unwrap() })
        .collect::<Vec<_>>()
        .await;

    assert_eq!(ret.len(), 3);
    sleep(Duration::from_secs(1)).await;
    Ok(())
}

async fn start_server() -> Result<SocketAddr> {
    let config = AppConfig::load()?;
    let port = config.server.port;
    let addr = format!("[::1]:{}", port).parse().unwrap();

    let user_server = NotificationService::new().into_service();

    tokio::spawn(async move {
        Server::builder()
            .add_service(user_server)
            .serve(addr)
            .await
            .unwrap();
    });
    sleep(Duration::from_micros(1)).await;
    Ok(addr)
}
