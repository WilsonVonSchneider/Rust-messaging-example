
use infrastructure::schema::slack_messages;
use serde::{Deserialize, Serialize};
use diesel::{
    AsChangeset, Queryable, QueryableByName, 
};

use chrono::NaiveDateTime;


#[derive(
    Queryable,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    QueryableByName,
    AsChangeset,
    Identifiable,
)]

#[diesel(table_name = slack_messages)]
#[diesel(treat_none_as_null = true)]
pub struct SlackMessage {
    pub id: String,
    pub name: String,
    pub channel: String,
    pub icon_emoji: String,
    pub message: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub checked: bool,
}


