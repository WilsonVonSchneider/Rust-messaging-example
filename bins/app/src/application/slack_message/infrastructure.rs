use super::{
    contract::PgServiceContract,
    data::SlackMessageData 
};
use std::env;
use async_trait::async_trait;
use support::store::models::slack_message::SlackMessage;
use error::Error;
use infrastructure::{
    db::Pg,
    schema::slack_messages,
};
use std::sync::Arc;
use diesel::{
    RunQueryDsl, ExpressionMethods
};

pub struct PgService {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgServiceContract for PgService {
    /// Create new slack message
    async fn create(&self, data: SlackMessageData) -> Result<SlackMessage, Error> {
        let mut connection = self.pg_pool.connection()?;
        diesel::insert_into(slack_messages::table)
            .values(data)
            .get_result::<SlackMessage>(&mut connection)
            .map_err(Error::from)

    }

    /// Update slack message
    async fn update(&self, message_id: &str) -> Result<usize, Error> {
        let mut connection = self.pg_pool.connection()?;

        diesel::update(slack_messages::table)
            .filter(slack_messages::id.eq(message_id))
            .set(slack_messages::checked.eq(true))
            .execute(&mut connection)
            .map_err(Error::from)
            .map_err(|e| e.add_cause_if_not_found("Message not found"))
    }

    /// Send slack message to slack app
    async fn send_to_slack(&self, data: SlackMessageData) -> Result<reqwest::Response, reqwest::Error> {
        let (message, name, channel, icon_emoji) = (data.message, data.name, data.channel, data.icon_emoji);
        let body = format!(
            "{{\"channel\": \"{channel}\", \"username\": \"{name}\", \"text\": \"{message}\", \"icon_emoji\": \":{icon_emoji}:\"}}",
            message = message, name = name, channel = channel, icon_emoji = icon_emoji
        );
    
        let client = reqwest::Client::new();
        client
            .post(env::var("SLACK_URL").unwrap())
            .body(body)
            .send()
            .await
    }
}
