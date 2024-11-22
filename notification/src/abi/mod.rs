mod email;
mod in_app;
mod sms;

use std::{sync::Arc, time::Duration};

use chrono::Timelike;
use futures::Stream;
use tokio::{sync::mpsc, time::sleep};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};

use crate::{
    pb::{notification_server::NotificationServer, send_request::Msg, SendRequest, SendResponse},
    NotificationServerInner, NotificationService, ResponseStream, ServiceResult,
};
use futures::StreamExt;

const CHANNEL_SIZE: usize = 1024;

impl NotificationService {
    pub fn new() -> Self {
        NotificationService {
            inner: Arc::new(NotificationServerInner {
                sender: dummy_send(),
            }),
        }
    }

    pub fn into_service(self) -> NotificationServer<Self> {
        NotificationServer::new(self)
    }

    pub async fn send(
        &self,
        mut req: impl Stream<Item = Result<SendRequest, Status>> + Send + Unpin + 'static,
    ) -> ServiceResult<ResponseStream> {
        let (tx, rx) = mpsc::channel(CHANNEL_SIZE);

        let notif = self.clone();

        tokio::spawn(async move {
            while let Some(Ok(result)) = req.next().await {
                let notif_clone = notif.clone();
                let res = match result.msg {
                    Some(Msg::Email(email)) => email.send(notif_clone).await,
                    Some(Msg::InApp(in_app)) => in_app.send(notif_clone).await,
                    Some(Msg::Sms(sms)) => sms.send(notif_clone).await,
                    None => Err(Status::invalid_argument("Invalid request")),
                };
                tx.send(res).await.expect("working rx");
            }
        });
        let out_stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(out_stream)))
    }
}

impl Default for NotificationService {
    fn default() -> Self {
        Self::new()
    }
}
pub trait Sender {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status>;
}

fn dummy_send() -> mpsc::Sender<Msg> {
    let (tx, mut rx) = mpsc::channel(CHANNEL_SIZE * 100);
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("dummy send msg: {:?}", msg);
            sleep(Duration::from_millis(300)).await;
        }
    });
    tx
}

fn now_timestamp() -> prost_types::Timestamp {
    let now = chrono::Utc::now();
    prost_types::Timestamp {
        seconds: now.timestamp(),
        nanos: now.nanosecond() as i32,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        pb::{EmailMessage, InAppMessage, SmsMessage},
        NotificationService,
    };
    use anyhow::Result;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn send_should_work() -> Result<()> {
        let service = NotificationService::new();
        let stream = tokio_stream::iter(vec![
            Ok(EmailMessage::fake().into()),
            Ok(SmsMessage::fake().into()),
            Ok(InAppMessage::fake().into()),
        ]);

        let res = service.send(stream).await?;
        let ret = res.into_inner().collect::<Vec<_>>().await;

        assert_eq!(ret.len(), 3);
        ret.iter().for_each(|content| {
            println!("{:?}", content);
        });

        Ok(())
    }
}
