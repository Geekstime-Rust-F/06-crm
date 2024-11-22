mod abi;
mod config;

pub mod pb;

use std::pin::Pin;

pub use config::AppConfig;
use futures::Stream;
use pb::{
    metadata_server::{Metadata, MetadataServer},
    Content, MaterializeRequest,
};
use tonic::{async_trait, Request, Response, Status, Streaming};

type ResponseStream = Pin<Box<dyn Stream<Item = Result<Content, Status>> + Send>>;
type ServiceResult<T> = Result<Response<T>, Status>;

#[allow(unused)]
pub struct MetadataService {
    config: AppConfig,
}

impl MetadataService {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn into_server(self) -> MetadataServer<Self> {
        MetadataServer::new(self)
    }
}

#[async_trait]
impl Metadata for MetadataService {
    type MaterializeStream = ResponseStream;

    async fn materialize(
        &self,
        request: Request<Streaming<MaterializeRequest>>,
    ) -> ServiceResult<Self::MaterializeStream> {
        self.meterialize(request.into_inner()).await
    }
}
