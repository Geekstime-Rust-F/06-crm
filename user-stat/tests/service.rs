use std::{net::SocketAddr, time::Duration};

use anyhow::Result;
use chrono::{TimeZone, Utc};
use futures::StreamExt;
use sqlx_db_tester::TestPg;
use tokio::{net::TcpListener, time::sleep};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::{transport::Server, Request};
use user_stat::{
    pb::{user_stats_client::UserStatsClient, QueryRequestBuilder, RawQueryRequest, User},
    test_utils::{form_time_query, id},
    UserStatsService,
};

#[tokio::test]
async fn raw_query_should_work() -> Result<()> {
    let (_testpg, addr) = start_server().await?;
    let mut client =
        UserStatsClient::connect(format!("http://[{}]:{}", addr.ip(), addr.port())).await?;

    let request = Request::new(
        RawQueryRequest {
             query: "SELECT * FROM user_stats WHERE last_visited_at > '2024-10-08 13:43:45.411498 UTC' AND last_visited_at < '2024-11-18 13:43:45.411498 UTC' AND created_at > '2023-05-10 13:43:45.411472 UTC' AND created_at < '2024-11-18 13:43:45.411472 UTC' AND array[16052] <@ viewed_but_not_started;".to_string()
            }
        );
    let stream = client.raw_query(request).await?.into_inner();

    let ret: Vec<User> = stream.then(|x| async move { x.unwrap() }).collect().await;

    assert_eq!(ret.len(), 3);

    Ok(())
}

#[tokio::test]
async fn query_should_work() -> Result<()> {
    let (_testpg, addr) = start_server().await?;
    let mut client =
        UserStatsClient::connect(format!("http://[{}]:{}", addr.ip(), addr.port())).await?;

    let start = Utc.with_ymd_and_hms(2023, 5, 10, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2023, 7, 10, 0, 0, 0).unwrap();

    let query_request = QueryRequestBuilder::default()
        .timestamp(("created_at".to_string(), form_time_query(start, end)))
        .id(("viewed_but_not_started".to_string(), id(&[12856])))
        .build()?;

    let request = Request::new(query_request);
    let stream = client.query(request).await?.into_inner();

    let ret: Vec<User> = stream.then(|x| async move { x.unwrap() }).collect().await;

    assert!(!ret.is_empty());

    Ok(())
}

async fn start_server() -> Result<(TestPg, SocketAddr)> {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let (_testdb, user_stats_service) = UserStatsService::new_for_test().await?;

    let user_stats_server = user_stats_service.into_server();

    tokio::spawn(async move {
        Server::builder()
            .add_service(user_stats_server)
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .unwrap();
    });
    sleep(Duration::from_millis(10)).await;
    Ok((_testdb, addr))
}
