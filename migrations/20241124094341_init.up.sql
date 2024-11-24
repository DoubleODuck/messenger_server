CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS user_table (
    "user_id" uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "login" varchar UNIQUE,
    "password" varchar,
    "email" varchar UNIQUE,
    "created_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    "updated_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS message_table (
    "message_id" uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "message_text" TEXT NOT NULL,
    "sender" uuid REFERENCES user_table(user_id),
    "recipient" uuid REFERENCES user_table(user_id),
    "created_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
-----------------------------------------------------------------------------
INSERT INTO user_table ("login", "password", "email") VALUES 
("","","")