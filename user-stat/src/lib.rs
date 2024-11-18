mod abi;
mod config;
pub mod pb;
pub use config::AppConfig;

use anyhow::Result;
use futures::Stream;
use pb::{user_stats_service_server::UserStatsService, QueryRequest, RawQueryRequest};
use sqlx::PgPool;
use std::{pin::Pin, sync::Arc};
use tonic::{async_trait, Request, Response, Status};

use crate::pb::User;

type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;
type ServiceResult<T> = Result<Response<T>, Status>;

struct UserStatsSeverInner {
    pool: PgPool,
}
pub struct UserStatsServer {
    inner: Arc<UserStatsSeverInner>,
}

impl UserStatsServer {
    pub async fn new(config: AppConfig) -> Result<Self> {
        let pool = PgPool::connect(&config.database.get_url_with_database()).await?;

        Ok(Self {
            inner: Arc::new(UserStatsSeverInner { pool }),
        })
    }
}

#[async_trait]
impl UserStatsService for UserStatsServer {
    type QueryStream = ResponseStream;
    type RawQueryStream = ResponseStream;
    async fn query(&self, request: Request<QueryRequest>) -> ServiceResult<Self::QueryStream> {
        self.query(request).await
    }

    async fn raw_query(
        &self,
        request: Request<RawQueryRequest>,
    ) -> ServiceResult<Self::QueryStream> {
        self.raw_query(request).await
    }
}
