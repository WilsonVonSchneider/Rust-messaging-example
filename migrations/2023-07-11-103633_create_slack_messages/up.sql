-- Your SQL goes here
CREATE TABLE slack_messages (
    id varchar(36) DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
    "name" varchar(50) NOT NULL,
    channel varchar(50) NOT NULL,
    icon_emoji varchar(50) NOT NULL,
    "message" TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"checked" BOOLEAN DEFAULT false NOT NULL
);
