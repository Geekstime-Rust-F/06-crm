use chrono::{DateTime, Timelike, Utc};
use fake::{
    faker::{chrono::zh_cn::DateTimeBetween, lorem::zh_cn::Sentence, name::zh_cn::Name},
    Dummy, Fake,
};
use futures::{Stream, StreamExt};
use prost_types::Timestamp;
use rand::{seq::SliceRandom, Rng};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::Response;

use crate::{
    pb::{Content, ContentType, MaterializeRequest, Publisher},
    MetadataService, ResponseStream, ServiceResult,
};

const CHANNEL_SIZE: usize = 128;

impl MetadataService {
    pub async fn meterialize(
        &self,
        mut in_stream: impl Stream<Item = Result<MaterializeRequest, tonic::Status>>
            + Send
            + Unpin
            + 'static,
    ) -> ServiceResult<ResponseStream> {
        let (tx, rx) = mpsc::channel(CHANNEL_SIZE);

        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(v) => {
                        tx.send(Ok(Content::new(v.id))).await.expect("working rx");
                    }
                    Err(e) => {
                        tx.send(Err(e)).await.expect("working rx");
                    }
                }
            }
        });

        let out_stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(out_stream) as ResponseStream))
    }
}

impl Content {
    pub fn new(id: u32) -> Self {
        let mut rng = rand::thread_rng();
        Content {
            id,
            name: Name().fake(),
            description: Sentence(1..3).fake(),
            publishers: (1..rng.gen_range(1..10))
                .map(|_| Publisher::new())
                .collect(),
            url: "https://placehold.co/1600x900".to_string(),
            image: "https://placehold.co/1600x900".to_string(),
            r#type: (0..6).fake(),
            views: rng.gen_range(12344..1000000),
            likes: rng.gen_range(1234..10000),
            dislikes: rng.gen_range(123..10000),
            created_at: created_at(),
        }
    }

    pub fn to_body(&self) -> String {
        format!("Content: {:?}", self)
    }
}

pub struct Template<'a>(pub &'a [Content]);

impl<'a> Template<'a> {
    pub fn to_body(&self) -> String {
        format!("Contents: {:?}", self.0)
    }
}

impl Publisher {
    pub fn new() -> Self {
        Publisher {
            id: (10000..200000).fake(),
            name: Name().fake(),
            avatar: "https://placehold.co/400x400".to_string(),
        }
    }
}

struct RandomContentType;
impl Dummy<RandomContentType> for ContentType {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &RandomContentType, rng: &mut R) -> ContentType {
        [
            ContentType::Unspecified,
            ContentType::Short,
            ContentType::Vlog,
            ContentType::Movie,
            ContentType::Series,
            ContentType::Other,
        ]
        .choose(rng)
        .unwrap()
        .to_owned()
    }
}

fn before(days: u64) -> DateTime<Utc> {
    Utc::now() - chrono::Duration::days(days as i64)
}

fn now() -> DateTime<Utc> {
    Utc::now()
}

fn created_at() -> Option<Timestamp> {
    let date: DateTime<Utc> = DateTimeBetween(before(365), now()).fake();
    Some(Timestamp {
        seconds: date.timestamp(),
        nanos: date.nanosecond() as i32,
    })
}

#[cfg(test)]
mod tests {
    use crate::config;

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn meterialize_should_work() -> Result<()> {
        let config = config::AppConfig::load()?;
        let service = MetadataService::new(config);

        let req_stream = tokio_stream::iter(vec![
            Ok(MaterializeRequest { id: 1 }),
            Ok(MaterializeRequest { id: 2 }),
            Ok(MaterializeRequest { id: 3 }),
        ]);

        let resp = service.meterialize(req_stream).await?;
        let ret = resp.into_inner().collect::<Vec<_>>().await;

        assert_eq!(ret.len(), 3);

        ret.iter().for_each(|content| {
            println!("{:?}", content);
        });

        Ok(())
    }
}
