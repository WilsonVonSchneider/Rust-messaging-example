// @generated automatically by Diesel CLI.

diesel::table! {
    slack_messages (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 50]
        channel -> Varchar,
        #[max_length = 50]
        icon_emoji -> Varchar,
        message -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        checked -> Bool,
    }
}
