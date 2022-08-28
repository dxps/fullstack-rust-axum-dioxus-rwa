-- Add migration script here
create table if not exists accounts (
    id              serial          not null    primary key,
    eid             char(36)        not null,
    email           varchar(255)    default ''  unique,
    username        varchar(255)    not null,
    gender          varchar(10)     default '',
    created_at      date            default current_date,
    updated_at      date            default current_date,
    password        varchar(255)    not null,
    salt            varchar(20)     not null,
    bio             text            default '',
    image           varchar(255)    default '/assets/images/default_avatar.png'
);
