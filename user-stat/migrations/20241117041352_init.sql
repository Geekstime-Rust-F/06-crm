-- Add migration script here
create type gender_type as enum (
    'female',
    'male',
    'unknown'
);

create table user_stats(
    email varchar(64) NOT NULL PRIMARY KEY,
    name varchar(64) NOT NULL,
    gender gender_type DEFAULT 'unknown',
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    last_visited_at timestamptz,
    last_watched_at timestamptz,
    recent_watched int[],
    viewed_but_not_started int[],
    started_but_not_finished int[],
    finished int[],
    last_email_notification timestamptz,
    last_push_notification timestamptz,
    last_sms_notification timestamptz
);
