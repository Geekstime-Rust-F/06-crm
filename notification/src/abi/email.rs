use tonic::Status;
use tracing::warn;

use crate::{
    pb::{send_request::Msg, EmailMessage, SendRequest, SendResponse},
    NotificationService,
};

use super::{now_timestamp, Sender};

impl Sender for EmailMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Msg::Email(self)).await.map_err(|e| {
            warn!("Error sending SMS: {}", e);
            Status::internal(e.to_string())
        })?;
        Ok(SendResponse {
            message_id,
            sent_at: Some(now_timestamp()),
        })
    }
}

impl From<EmailMessage> for SendRequest {
    fn from(msg: EmailMessage) -> Self {
        SendRequest {
            msg: Some(Msg::Email(msg)),
        }
    }
}

#[cfg(feature = "test_utils")]
impl EmailMessage {
    pub fn fake() -> Self {
        use fake::faker::internet::en::SafeEmail;
        use fake::Fake;
        use uuid::Uuid;
        EmailMessage {
            message_id: Uuid::new_v4().to_string(),
            from: SafeEmail().fake(),
            recipients: vec![SafeEmail().fake()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
        }
    }
}
