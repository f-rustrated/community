CREATE TYPE public.account_status AS ENUM ('active', 'deleted', 'abnormal');

-- Add up migration script here
CREATE TABLE IF NOT EXISTS public.account
(
    id              bigserial primary key,
    uuid            uuid                                     not null,
    name            text                                     not null,
    status          account_status DEFAULT 'active'          not null,
    hashed_password text                                     not null,
    created_at      timestamptz    default CURRENT_TIMESTAMP not null,
    updated_at      timestamptz    default CURRENT_TIMESTAMP not null
);

comment on column public.account.uuid is 'public id to uniquely identify a account';

comment on column public.account.name is 'account name';

comment on column public.account.status is 'N: normal, D: deleted, A: abnormal';

create unique index uidx_account_uuid on public.account (uuid);

create unique index uidx_account_name on public.account (name);

-- CREATE TABLE IF NOT EXISTS public.notification
-- (
--     id             bigserial
--         primary key,
--     target_account_id bigint                              not null,
--     type           char                                not null,
--     body           json                                not null,
--     view_yn        char      default 'N'::bpchar       not null,
--     created_at     timestamp default CURRENT_TIMESTAMP not null
-- );

-- comment on column public.notification.target_account_id is 'the target account to send notification to';

-- comment on column public.notification.type is 'the type of notification e.g. C: Someone replied with comment';

-- comment on column public.notification.body is 'the main data of the notification';

-- comment on column public.notification.view_yn is 'whether the account has viewd the notification(Y/N)';

-- create index idx_target_account_id
--     on public.notification (target_account_id);


CREATE TYPE public.post_category AS ENUM ('default', 'knowledge');

CREATE TYPE public.post_status AS ENUM ('created', 'deleted' , 'edited');

CREATE TABLE IF NOT EXISTS public.post
(
    id         bigserial primary key,
    account_id bigint                                  not null,
    title      text                                    not null,
    thumbnail  text,
    category   post_category default 'default'         not null,
    body       text                                    not null,
    status     post_status   default 'created'         not null,
    created_at timestamptz   default CURRENT_TIMESTAMP not null,
    updated_at timestamptz   default CURRENT_TIMESTAMP not null
);

comment on column public.post.account_id is 'maps to account.id';

comment on column public.post.category is 'category of the post, e.g. DEFAULT, KNOWLEDGE,  etc';


-- CREATE TABLE IF NOT EXISTS public.post_reaction
-- (
--     post_id       bigint                              not null,
--     account_id       bigint                              not null,
--     reaction_type char                                not null,
--     created_at    timestamp default CURRENT_TIMESTAMP not null,
--     primary key (post_id, account_id)
-- );

-- comment on column public.post_reaction.reaction_type is 'L: Like, D: Dislike';

-- CREATE TABLE IF NOT EXISTS public.post_statistic
-- (
--     post_id       bigserial,
--     like_count    integer default 0 not null,
--     dislike_count integer default 0 not null,
--     comment_count integer default 0 not null
-- );

-- comment on column public.post_statistic.like_count is 'number of likes';

-- comment on column public.post_statistic.dislike_count is 'number of dislikes';

-- comment on column public.post_statistic.comment_count is 'number of comments';

CREATE TABLE IF NOT EXISTS public.comment
(
    id          bigserial
        primary key,
    account_id  bigserial,
    target_id   varchar(50)                           not null,
    target_type char                                  not null,
    message     text                                  not null,
    edited_yn   char        default 'N'::bpchar       not null,
    deleted_yn  char        default 'N'::bpchar       not null,
    created_at  timestamptz default CURRENT_TIMESTAMP not null,
    updated_at  timestamptz default CURRENT_TIMESTAMP not null
);

comment on column public.comment.target_id is 'the id of the target in which current comment is subject to';

comment on column public.comment.target_type is 'the type of the target in which current is subject to, e.g. P: POST, C: COMMENT';

comment on column public.comment.edited_yn is 'whether the comment has been edited(Y/N)';

comment on column public.comment.deleted_yn is 'whether the comment has been deleted(Y/N)';

create index idx_target_id_target_type
    on public.comment (target_id, target_type);

-- CREATE TABLE IF NOT EXISTS public.comment_reaction
-- (
--     comment_id    bigserial,
--     account_id       bigserial,
--     reaction_type char                                not null,
--     created_at    timestamp default CURRENT_TIMESTAMP not null,
--     primary key (comment_id, account_id)
-- );

-- comment on column public.comment_reaction.reaction_type is 'L: LIKE, D: DISLIKE';

-- CREATE TABLE IF NOT EXISTS public.comment_statistic
-- (
--     comment_id    bigserial,
--     like_count    integer default 0 not null,
--     dislike_count integer default 0 not null,
--     reply_count   integer default 0 not null
-- );

-- comment on column public.comment_statistic.like_count is 'number of likes';

-- comment on column public.comment_statistic.dislike_count is 'number of dislikes';

-- comment on column public.comment_statistic.reply_count is 'number of replies';