use chrono::{DateTime, TimeZone, Utc};
use futures::{stream, Stream};
use prost_types::Timestamp;

use anyhow::Result;
use std::pin::Pin;
use tonic::{Response, Status};

use crate::{
    pb::{user_stats_server::UserStatsServer, QueryRequest, RawQueryRequest, User},
    ServiceResult, UserStatsService,
};

#[allow(unused)]
type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;

#[allow(unused)]
impl UserStatsService {
    pub async fn query(&self, request: QueryRequest) -> ServiceResult<ResponseStream> {
        let mut sql = "SELECT * FROM user_stats WHERE ".to_string();

        let time_conditions = request
            .timestamps
            .iter()
            .map(|(k, v)| timestamp_query(k, v.start, v.end))
            .collect::<Vec<String>>()
            .join(" AND ");

        let id_conditions = request
            .ids
            .iter()
            .map(|(k, v)| ids_query(k, v.ids.clone()))
            .collect::<Vec<String>>()
            .join(" AND ");

        sql.push_str(&time_conditions);
        sql.push_str(" AND ");
        sql.push_str(&id_conditions);

        println!("SQL: {}", sql);
        self.raw_query(RawQueryRequest { query: sql }).await
    }

    pub async fn raw_query(&self, request: RawQueryRequest) -> ServiceResult<ResponseStream> {
        let ret: Vec<User> = sqlx::query_as(&request.query)
            .fetch_all(&self.inner.pool)
            .await
            .unwrap();

        Ok(Response::new(Box::pin(stream::iter(
            ret.into_iter().map(Ok),
        ))))
    }
}

impl UserStatsService {
    pub fn into_server(self) -> UserStatsServer<UserStatsService> {
        UserStatsServer::new(self)
    }
}

fn timestamp_query(name: &str, start: Option<Timestamp>, end: Option<Timestamp>) -> String {
    if start.is_none() && end.is_none() {
        return "TRUE".to_string();
    }

    if start.is_none() {
        let end = timestamp_to_datetime(end.unwrap());
        format!("{} < '{}'", name, end)
    } else if end.is_none() {
        let start = timestamp_to_datetime(start.unwrap());
        format!("{} > '{}'", name, start)
    } else {
        let start = timestamp_to_datetime(start.unwrap());
        let end = timestamp_to_datetime(end.unwrap());
        format!("{} > '{}' AND {} < '{}'", name, start, name, end)
    }
}

fn timestamp_to_datetime(ts: Timestamp) -> DateTime<Utc> {
    Utc.timestamp_opt(ts.seconds, ts.nanos as u32).unwrap()
}

fn ids_query(name: &str, ids: Vec<u32>) -> String {
    if ids.is_empty() {
        return "TRUE".to_string();
    }

    format!("array{:?} <@ {}", ids, name)
}

#[cfg(test)]
mod tests {
    use stream::StreamExt;

    use crate::{
        pb::QueryRequestBuilder,
        test_utils::{form_time_query, id},
    };

    use super::*;

    #[tokio::test]
    async fn raw_query_should_work() -> Result<()> {
        let (_testdb, svc) = UserStatsService::new_for_test().await?;
        let mut ret = svc.raw_query(RawQueryRequest { query: "SELECT * FROM user_stats WHERE created_at BETWEEN '2023-01-01' AND '2024-01-02' LIMIT 5;".to_string() }).await?.into_inner();

        while let Some(user) = ret.next().await {
            println!("{:?}", user);
        }

        Ok(())
    }

    #[tokio::test]
    async fn query_should_work() -> Result<()> {
        let (_testdb, svc) = UserStatsService::new_for_test().await.unwrap();
        let query_request = QueryRequestBuilder::default()
            .timestamp((
                "created_at".to_string(),
                form_time_query(Some(1000), Some(0)),
            ))
            .timestamp((
                "last_visited_at".to_string(),
                form_time_query(Some(100), Some(0)),
            ))
            .id(("viewed_but_not_started".to_string(), id(&[16857])))
            .build()?;
        let mut ret = svc.query(query_request).await?.into_inner();

        while let Some(user) = ret.next().await {
            println!("{:?}", user);
        }

        Ok(())
    }
}
