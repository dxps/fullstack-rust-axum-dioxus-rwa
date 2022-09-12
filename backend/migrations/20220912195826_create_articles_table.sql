
create table if not exists articles (
    id              BIGSERIAL,
    slug            VARCHAR(64),
    title           VARCHAR(64),
    description     VARCHAR(256),
    body            VARCHAR(4096),
    created_at      TIMESTAMP WITH TIME ZONE    DEFAULT current_timestamp,
    updated_at      TIMESTAMP WITH TIME ZONE    DEFAULT current_timestamp,
    author_id       BIGSERIAL,

    PRIMARY KEY(id),

    CONSTRAINT fk_author_id    FOREIGN KEY(author_id)   REFERENCES accounts(id),
    CONSTRAINT unique_slug     UNIQUE(slug)
);
