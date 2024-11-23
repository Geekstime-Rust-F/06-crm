mod abi;
mod config;
pub mod pb;
pub use config::AppConfig;

use anyhow::Result;
use crm_metadata::pb::metadata_client::MetadataClient;
use notification::pb::notification_client::NotificationClient;
use pb::{
    crm_server::{Crm, CrmServer},
    RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest, WelcomeResponse,
};
use tonic::Status;
use tonic::{async_trait, transport::Channel, Request, Response};
use user_stat::pb::user_stats_client::UserStatsClient;

pub struct CrmService {
    config: AppConfig,
    user_stat: UserStatsClient<Channel>,
    metadata: MetadataClient<Channel>,
    notification: NotificationClient<Channel>,
}

impl CrmService {
    pub async fn try_new(config: AppConfig) -> Result<Self> {
        let user_stat = UserStatsClient::connect(config.server.user_stat.clone()).await?;
        let metadata = MetadataClient::connect(config.server.metadata.clone()).await?;
        let notification = NotificationClient::connect(config.server.notification.clone()).await?;
        Ok(Self {
            config,
            user_stat,
            metadata,
            notification,
        })
    }

    pub fn into_service(self) -> CrmServer<CrmService> {
        CrmServer::new(self)
    }
}

#[async_trait]
impl Crm for CrmService {
    async fn welcome(
        &self,
        request: Request<WelcomeRequest>,
    ) -> Result<Response<WelcomeResponse>, Status> {
        self.welcome(request.into_inner()).await
    }

    async fn recall(
        &self,
        _request: Request<RecallRequest>,
    ) -> Result<Response<RecallResponse>, Status> {
        todo!()
    }

    async fn remind(
        &self,
        _request: Request<RemindRequest>,
    ) -> Result<Response<RemindResponse>, Status> {
        todo!()
    }
}
