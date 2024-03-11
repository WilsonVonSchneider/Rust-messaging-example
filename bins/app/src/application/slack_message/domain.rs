use super::{contract::{PgServiceContract, SlackMessageContract},
data::SlackMessageData
};
use async_trait::async_trait;
use http::StatusCode;
use error::Error;

pub struct SlackMessages<A: PgServiceContract> {
    pub service: A,
}

#[async_trait]
impl<A> SlackMessageContract for SlackMessages<A>
where
    A: PgServiceContract + Sync + Send,
{
    /// Create user expertise
    async fn create(&self, data: SlackMessageData ) -> Result<usize, Error> {
        let message_created = self.service.create(data.clone()).await?;

        match self.service.send_to_slack(data).await?.status() {
            StatusCode::OK => {
                self.service.update(&message_created.id).await
            },
            StatusCode::NOT_FOUND => Err(Error::NotFoundWithCause("channel not found".to_string())),
            _ => Err(Error::InternalServerError("Something went wrong".to_string()))
        }
    }
}
