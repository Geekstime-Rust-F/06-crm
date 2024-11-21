mod abi;
mod config;
pub use config::AppConfig;
use tokio::sync::mpsc;
pub mod pb;

use std::{ops::Deref, pin::Pin, sync::Arc};

use futures::Stream;
use pb::{notification_server::Notification, send_request::Msg, SendRequest, SendResponse};
use tonic::{async_trait, Request, Response, Status, Streaming};

#[derive(Clone)]
pub struct NotificationService {
    inner: Arc<NotificationServerInner>,
}

pub struct NotificationServerInner {
    sender: mpsc::Sender<Msg>,
}

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<SendResponse, Status>> + Send>>;

#[async_trait]
impl Notification for NotificationService {
    type sendStream = ResponseStream;
    async fn send(&self, req: Request<Streaming<SendRequest>>) -> ServiceResult<ResponseStream> {
        self.send(req).await
    }
}

impl Deref for NotificationService {
    type Target = Arc<NotificationServerInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
