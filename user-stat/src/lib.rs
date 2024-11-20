mod abi;
mod config;
#[allow(clippy::all)]
pub mod pb;
pub use config::AppConfig;

use anyhow::Result;
use futures::Stream;
use pb::{user_stats_server::UserStats, QueryRequest, RawQueryRequest};
use sqlx::PgPool;
use std::{pin::Pin, sync::Arc};
use tonic::{async_trait, Request, Response, Status};

use crate::pb::User;

type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;
type ServiceResult<T> = Result<Response<T>, Status>;

struct UserStatsSeverInner {
    pool: PgPool,
}
pub struct UserStatsService {
    inner: Arc<UserStatsSeverInner>,
}

impl UserStatsService {
    pub async fn new(config: AppConfig) -> Result<Self> {
        let pool = PgPool::connect(&config.database.get_url_with_database()).await?;

        Ok(Self {
            inner: Arc::new(UserStatsSeverInner { pool }),
        })
    }
}

#[async_trait]
impl UserStats for UserStatsService {
    type queryStream = ResponseStream;
    type raw_queryStream = ResponseStream;
    async fn query(&self, request: Request<QueryRequest>) -> ServiceResult<Self::queryStream> {
        self.query(request.into_inner()).await
    }

    async fn raw_query(
        &self,
        request: Request<RawQueryRequest>,
    ) -> ServiceResult<Self::raw_queryStream> {
        self.raw_query(request.into_inner()).await
    }
}
