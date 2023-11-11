create table public."user"
(
    id         bigserial
        primary key,
    uuid       uuid                                not null,
    name       varchar(100)                        not null,
    status     char      default 'N'::bpchar       not null,
    password   varchar(255)                        not null,
    created_at timestamp default CURRENT_TIMESTAMP not null,
    updated_at timestamp default CURRENT_TIMESTAMP not null
);

comment on column public."user".uuid is 'public id to uniquely identify a user';

comment on column public."user".name is 'user name';

comment on column public."user".status is 'N: normal, D: deleted, A: abnormal';

alter table public."user"
    owner to seonwoo960000;

create unique index uidx_user_uuid
    on public."user" (uuid);

create unique index uidx_user_name
    on public."user" (name);

create table public.notification
(
    id             bigserial
        primary key,
    target_user_id bigint                              not null,
    type           char                                not null,
    body           json                                not null,
    view_yn        char      default 'N'::bpchar       not null,
    created_at     timestamp default CURRENT_TIMESTAMP not null
);

comment on column public.notification.target_user_id is 'the target user to send notification to';

comment on column public.notification.type is 'the type of notification e.g. C: Someone replied with comment';

comment on column public.notification.body is 'the main data of the notification';

comment on column public.notification.view_yn is 'whether the user has viewd the notification(Y/N)';

alter table public.notification
    owner to seonwoo960000;

create index idx_target_user_id
    on public.notification (target_user_id);

create table public.post
(
    id         bigserial
        primary key,
    user_id    bigint                                           not null,
    title      varchar(64)                                      not null,
    subtitle   varchar(100),
    image_url  varchar(100),
    category   varchar(10) default 'DEFAULT'::character varying not null,
    body       text                                             not null,
    type       varchar(10) default 'POST'::character varying    not null,
    deleted_yn char        default 'N'::bpchar                  not null,
    created_at timestamp   default CURRENT_TIMESTAMP            not null,
    updated_at timestamp   default CURRENT_TIMESTAMP            not null
);

comment on column public.post.user_id is 'maps to user.id';

comment on column public.post.category is 'category of the post, e.g. DEFAULT, KNOWLEDGE, COMMUNITY, etc';

comment on column public.post.type is 'type of the post, e.g. ARTICLE, LINK, etc';

comment on column public.post.deleted_yn is 'whether the post is deleted (Y/N)';

alter table public.post
    owner to seonwoo960000;

create table public.post_reaction
(
    post_id       bigint                              not null,
    user_id       bigint                              not null,
    reaction_type char                                not null,
    created_at    timestamp default CURRENT_TIMESTAMP not null,
    primary key (post_id, user_id)
);

comment on column public.post_reaction.reaction_type is 'L: Like, D: Dislike';

alter table public.post_reaction
    owner to seonwoo960000;

create table public.post_statistic
(
    post_id       bigserial,
    like_count    integer default 0 not null,
    dislike_count integer default 0 not null,
    comment_count integer default 0 not null
);

comment on column public.post_statistic.like_count is 'number of likes';

comment on column public.post_statistic.dislike_count is 'number of dislikes';

comment on column public.post_statistic.comment_count is 'number of comments';

alter table public.post_statistic
    owner to seonwoo960000;

create table public.comment
(
    id          bigserial
        primary key,
    user_id     bigserial,
    target_id   varchar(50)                         not null,
    target_type char                                not null,
    message     text                                not null,
    edited_yn   char      default 'N'::bpchar       not null,
    deleted_yn  char      default 'N'::bpchar       not null,
    created_at  timestamp default CURRENT_TIMESTAMP not null,
    updated_at  timestamp default CURRENT_TIMESTAMP not null
);

comment on column public.comment.target_id is 'the id of the target in which current comment is subject to';

comment on column public.comment.target_type is 'the type of the target in which current is subject to, e.g. P: POST, C: COMMENT';

comment on column public.comment.edited_yn is 'whether the comment has been edited(Y/N)';

comment on column public.comment.deleted_yn is 'whether the comment has been deleted(Y/N)';

alter table public.comment
    owner to seonwoo960000;

create index idx_target_id_target_type
    on public.comment (target_id, target_type);

create table public.comment_reaction
(
    comment_id    bigserial,
    user_id       bigserial,
    reaction_type char                                not null,
    created_at    timestamp default CURRENT_TIMESTAMP not null,
    primary key (comment_id, user_id)
);

comment on column public.comment_reaction.reaction_type is 'L: LIKE, D: DISLIKE';

alter table public.comment_reaction
    owner to seonwoo960000;

create table public.comment_statistic
(
    comment_id    bigserial,
    like_count    integer default 0 not null,
    dislike_count integer default 0 not null,
    reply_count   integer default 0 not null
);

comment on column public.comment_statistic.like_count is 'number of likes';

comment on column public.comment_statistic.dislike_count is 'number of dislikes';

comment on column public.comment_statistic.reply_count is 'number of replies';

alter table public.comment_statistic
    owner to seonwoo960000;

