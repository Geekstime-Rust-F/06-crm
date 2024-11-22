use tonic::Status;
use tracing::warn;

use crate::{
    pb::{send_request::Msg, SendRequest, SendResponse, SmsMessage},
    NotificationService,
};

use super::{now_timestamp, Sender};

impl Sender for SmsMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Msg::Sms(self)).await.map_err(|e| {
            warn!("Error sending SMS: {}", e);
            Status::internal(e.to_string())
        })?;

        Ok(SendResponse {
            message_id,
            sent_at: Some(now_timestamp()),
        })
    }
}

impl From<SmsMessage> for SendRequest {
    fn from(msg: SmsMessage) -> Self {
        SendRequest {
            msg: Some(Msg::Sms(msg)),
        }
    }
}

#[cfg(feature = "test_utils")]
impl SmsMessage {
    pub fn fake() -> Self {
        use fake::{faker::phone_number::en::PhoneNumber, Fake};
        use uuid::Uuid;
        SmsMessage {
            message_id: Uuid::new_v4().to_string(),
            body: "Hello, world!".to_string(),
            phone_number: PhoneNumber().fake(),
        }
    }
}
