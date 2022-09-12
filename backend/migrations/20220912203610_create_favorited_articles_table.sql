
CREATE TABLE IF NOT EXISTS favorited_articles (
    article_id            BIGSERIAL,
    user_id               BIGSERIAL,

    PRIMARY KEY(article_id, user_id),

    CONSTRAINT fk_article_id   FOREIGN KEY(article_id)  REFERENCES articles(id),
    CONSTRAINT fk_user_id      FOREIGN KEY(user_id)     REFERENCES accounts(id)
);
