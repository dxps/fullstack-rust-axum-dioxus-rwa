-- Add migration script here
create table if not exists accounts (
    id              serial          not null    primary key,
    email           varchar(255)    default ''  unique,
    username        varchar(255)    not null,
    gender          varchar(10)     default 'unknown',
    created_at      date            default current_date,
    updated_at      date            default current_date,
    password        varchar(255)    not null,
    salt            varchar(20)     not null,
    introduction    text            default '',
    avatar          varchar(255)    default '/assets/images/default_avatar.png'
);
