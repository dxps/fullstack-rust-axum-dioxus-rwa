-- Add migration script here
create table if not exists accounts (
    id              BIGSERIAL                PRIMARY KEY,
    email           VARCHAR(255)                          DEFAULT '' UNIQUE,
    username        VARCHAR(255)             NOT NULL,
    created_at      TIMESTAMP WITH TIME ZONE               DEFAULT current_timestamp,
    updated_at      TIMESTAMP WITH TIME ZONE               DEFAULT current_timestamp,
    password        VARCHAR(255)             NOT NULL,
    salt            CHAR(12)                 NOT NULL,
    bio             TEXT                                  DEFAULT '',
    image           VARCHAR(255)
);
