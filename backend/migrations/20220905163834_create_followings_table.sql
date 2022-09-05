-- Add migration script here
create table if not exists followings (
    user_id                BIGSERIAL,
    followed_user_id       BIGSERIAL,

    PRIMARY KEY(user_id, followed_user_id),

    CONSTRAINT fk_user_id            FOREIGN KEY(user_id)           REFERENCES accounts(id),
    CONSTRAINT fk_followed_user_id   FOREIGN KEY(followed_user_id)  REFERENCES accounts(id)
);

