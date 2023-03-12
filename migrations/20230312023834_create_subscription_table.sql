-- Add migration script here
--migrations/20230312023834_create_subscription_table.sql
-- Create subscriptions table
CREATE TABLE subscriptions (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL
);