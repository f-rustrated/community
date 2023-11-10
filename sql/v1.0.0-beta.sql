create table if not exists "user"
(
    id         bigserial
    primary key,
    name       varchar(100)                        not null,
    status     char      default 'N'::bpchar       not null,
    password   varchar(255)                        not null,
    created_at timestamp default CURRENT_TIMESTAMP not null,
    updated_at timestamp default CURRENT_TIMESTAMP not null
    );

comment on column "user".name is 'user name';

comment on column "user".status is 'N: normal, D: deleted, A: abnormal';

create unique index if not exists uidx_user_name
    on "user" (name);

create table if not exists notification
(
    id         bigserial
    primary key,
    user_id    bigint                              not null,
    type       char                                not null,
    body       json                                not null,
    view_yn    char      default 'N'::bpchar       not null,
    created_at timestamp default CURRENT_TIMESTAMP not null
);

comment on column notification.user_id is 'the target user to send notification to';

comment on column notification.type is 'the type of notification e.g. C: Someone replied with comment';

comment on column notification.body is 'the main data of the notification';

comment on column notification.view_yn is 'whether the user has viewed the notification(Y/N)';

create index if not exists idx_target_user_id
    on notification (user_id);

create table if not exists post
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

comment on column post.user_id is 'maps to user.id';

comment on column post.category is 'category of the post, e.g. DEFAULT, KNOWLEDGE, COMMUNITY, etc';

comment on column post.type is 'type of the post, e.g. ARTICLE, LINK, etc';

comment on column post.deleted_yn is 'whether the post is deleted (Y/N)';

create table if not exists post_reaction
(
    post_id       bigint                              not null,
    user_id       bigint                              not null,
    reaction_type char                                not null,
    created_at    timestamp default CURRENT_TIMESTAMP not null,
    primary key (post_id, user_id)
    );

comment on column post_reaction.reaction_type is 'L: Like, D: Dislike';

create table if not exists post_statistic
(
    post_id       bigserial,
    like_count    integer default 0 not null,
    dislike_count integer default 0 not null,
    comment_count integer default 0 not null
);

comment on column post_statistic.like_count is 'number of likes';

comment on column post_statistic.dislike_count is 'number of dislikes';

comment on column post_statistic.comment_count is 'number of comments';

create table if not exists comment
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

comment on column comment.target_id is 'the id of the target in which current comment is subject to';

comment on column comment.target_type is 'the type of the target in which current is subject to, e.g. P: POST, C: COMMENT';

comment on column comment.edited_yn is 'whether the comment has been edited(Y/N)';

comment on column comment.deleted_yn is 'whether the comment has been deleted(Y/N)';

create index if not exists idx_target_id_target_type
    on comment (target_id, target_type);

create table if not exists comment_reaction
(
    comment_id    bigserial,
    user_id       bigserial,
    reaction_type char                                not null,
    created_at    timestamp default CURRENT_TIMESTAMP not null,
    primary key (comment_id, user_id)
    );

comment on column comment_reaction.reaction_type is 'L: LIKE, D: DISLIKE';

create table if not exists comment_statistic
(
    comment_id    bigserial,
    like_count    integer default 0 not null,
    dislike_count integer default 0 not null,
    reply_count   integer default 0 not null
);

comment on column comment_statistic.like_count is 'number of likes';

comment on column comment_statistic.dislike_count is 'number of dislikes';

comment on column comment_statistic.reply_count is 'number of replies';



