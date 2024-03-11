/* use super::data::*; */
use async_trait::async_trait;
use error::Error;
use super::data::SlackMessageData;
use support::store::models::slack_message::SlackMessage;


#[async_trait]
pub trait SlackMessageContract {
    async fn create(&self, attributes: SlackMessageData) -> Result<usize, Error>;
}
    
// setters
#[async_trait]
pub trait PgServiceContract {
    async fn create(&self, data: SlackMessageData) -> Result<SlackMessage, Error>;
    async fn send_to_slack(&self, data: SlackMessageData) -> Result<reqwest::Response, reqwest::Error>;
    async fn update(&self, message_id: &str) -> Result<usize, Error>;
}

    
