-- Add migration script here
create index user_stats_created_at on user_stats(created_at);
create index user_stats_last_visited_at on user_stats(last_visited_at);
create index user_stats_last_watched_at on user_stats(last_watched_at);
create index user_stats_recent_watched on user_stats using GIN(recent_watched);
create index user_stats_viewed_but_not_started on user_stats using GIN(viewed_but_not_started);
create index user_stats_started_but_not_finished on user_stats using GIN(started_but_not_finished);
create index user_stats_finished on user_stats using GIN(finished);
create index user_stats_last_email_notification on user_stats(last_email_notification);
create index user_stats_last_push_notification on user_stats(last_push_notification);
create index user_stats_last_sms_notification on user_stats(last_sms_notification);
