use std::collections::HashSet;

use anyhow::Result;
use chrono::{DateTime, Timelike, Utc};
use crm_metadata::{
    pb::{Content, MaterializeRequest},
    Template,
};
use notification::pb::{send_request::Msg, EmailMessage, SendRequest};
use prost_types::Timestamp;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{IntoStreamingRequest, Request, Response, Status};
use user_stat::pb::{QueryRequest, QueryRequestBuilder, TimeQuery};
use uuid::Uuid;

use crate::{
    pb::{
        RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest,
        WelcomeResponse,
    },
    CrmService,
};

const CHANNEL_SIZE: usize = 1024;

#[allow(unused)]
impl CrmService {
    pub async fn welcome(&self, req: WelcomeRequest) -> Result<Response<WelcomeResponse>, Status> {
        let query_req = gen_user_stats_request("created_at", req.interval);
        let mut user_stream = self.user_stat.clone().query(query_req).await?.into_inner();

        let contents_req = gen_materialize_request(req.content_ids);
        let content_stream = self
            .metadata
            .clone()
            .materialize(contents_req)
            .await?
            .into_inner();
        let contents: Vec<Content> = content_stream
            .then(|x| async move { x.unwrap() })
            .collect()
            .await;

        let (tx, rx) = mpsc::channel(CHANNEL_SIZE);
        let sender_email = self.config.server.sender_email.clone();
        tokio::spawn(async move {
            while let Some(Ok(user)) = user_stream.next().await {
                let send_req =
                    gen_send_request("Welcome", sender_email.clone(), user.email, &contents);
                if tx.send(send_req).await.is_err() {
                    println!("Failed to send a msg");
                    return;
                }
            }
        });

        let send_reqs = ReceiverStream::new(rx);
        self.notification.clone().send(send_reqs).await.unwrap();

        let ret = WelcomeResponse { id: req.id };
        Ok(Response::new(ret))
    }

    async fn recall(&self, _req: RecallRequest) -> Result<Response<RecallResponse>, Status> {
        todo!()
    }

    async fn remind(&self, _req: RemindRequest) -> Result<Response<RemindResponse>, Status> {
        todo!()
    }
}

fn gen_user_stats_request(name: &str, days: u32) -> QueryRequest {
    let start = Utc::now() - chrono::Duration::days(days as i64);
    let end = Utc::now();
    QueryRequestBuilder::default()
        .timestamp((name.to_string(), form_time_query(start, end)))
        .build()
        .expect("failed to build user statsquery")
}

fn gen_materialize_request(
    content_ids: Vec<u32>,
) -> impl IntoStreamingRequest<Message = MaterializeRequest> {
    let materialize_requests: HashSet<MaterializeRequest> = content_ids
        .iter()
        .map(|x| MaterializeRequest { id: *x })
        .collect();
    Request::new(tokio_stream::iter(materialize_requests))
}

fn gen_send_request(
    subject: impl Into<String>,
    sender_email: String,
    recipient: String,
    contents: &[Content],
) -> SendRequest {
    let body = Template(contents).to_body();
    let msg = Msg::Email(EmailMessage {
        message_id: Uuid::new_v4().to_string(),
        subject: subject.into(),
        from: sender_email,
        recipients: vec![recipient],
        body,
    });

    SendRequest { msg: Some(msg) }
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
