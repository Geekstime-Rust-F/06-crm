use tonic::Status;
use tracing::warn;

use crate::{
    pb::{send_request::Msg, InAppMessage, SendRequest, SendResponse},
    NotificationService,
};

use super::{now_timestamp, Sender};

impl Sender for InAppMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Msg::InApp(self)).await.map_err(|e| {
            warn!("Error sending SMS: {}", e);
            Status::internal(e.to_string())
        })?;
        Ok(SendResponse {
            message_id,
            sent_at: Some(now_timestamp()),
        })
    }
}

impl From<InAppMessage> for SendRequest {
    fn from(msg: InAppMessage) -> Self {
        SendRequest {
            msg: Some(Msg::InApp(msg)),
        }
    }
}

#[cfg(feature = "test_utils")]
impl InAppMessage {
    pub fn fake() -> Self {
        use uuid::Uuid;
        InAppMessage {
            message_id: Uuid::new_v4().to_string(),
            device_id: Uuid::new_v4().to_string(),
            body: "Hello, world!".to_string(),
            title: "Hello".to_string(),
        }
    }
}
