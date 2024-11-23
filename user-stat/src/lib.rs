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

#[cfg(feature = "test_utils")]
pub mod test_utils {
    use anyhow::Result;
    use sqlx::{Executor, PgPool};
    use std::{path::Path, sync::Arc};

    use chrono::{DateTime, Timelike, Utc};
    use prost_types::Timestamp;
    use sqlx_db_tester::TestPg;

    use crate::{
        pb::{IdQuery, TimeQuery},
        AppConfig, UserStatsService, UserStatsSeverInner,
    };

    impl UserStatsService {
        pub async fn new_for_test() -> Result<(TestPg, Self)> {
            let config = AppConfig::load()?;
            let db_url = config.database.get_url_without_database();

            let (tdb, pool) = get_test_pool(Some(&db_url)).await;

            let svc = UserStatsService {
                inner: Arc::new(UserStatsSeverInner { pool }),
            };

            Ok((tdb, svc))
        }
    }

    pub async fn get_test_pool(db_url: Option<&str>) -> (TestPg, PgPool) {
        let url = db_url.unwrap_or("postgres://postgres:mysecretpassword@localhost");
        let tdb = TestPg::new(url.to_string(), Path::new("./migrations"));
        let pool = tdb.get_pool().await;

        let sqls = include_str!("../fixtures/test.sql").split(";");
        let mut transaction = pool.begin().await.expect("begin transaction failed");
        for sql in sqls {
            if sql.trim().is_empty() {
                continue;
            }
            transaction.execute(sql).await.expect("execute sql failed");
        }
        transaction.commit().await.unwrap();

        (tdb, pool)
    }

    pub fn form_time_query(start: DateTime<Utc>, end: DateTime<Utc>) -> TimeQuery {
        TimeQuery {
            start: Some(Timestamp {
                seconds: start.timestamp(),
                nanos: start.nanosecond() as i32,
            }),
            end: Some(Timestamp {
                seconds: end.timestamp(),
                nanos: start.nanosecond() as i32,
            }),
        }
    }

    pub fn id(id: &[u32]) -> IdQuery {
        IdQuery { ids: id.to_vec() }
    }
}
