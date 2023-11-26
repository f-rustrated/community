-- Add down migration script here
drop table if exists public.notification;

drop table if exists public.comment_reaction;
drop table if exists public.comment_statistic;
drop table if exists public.comment;

drop table if exists public.post_reaction;
drop table if exists public.post_statistic;
drop table if exists public.post;

drop table if exists public.account;
drop type account_status;
drop type post_category;