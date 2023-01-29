
CREATE TABLE IF NOT EXISTS tags_articles (
    tag                   VARCHAR(32),
    article_id            BIGSERIAL,

    PRIMARY KEY(tag, article_id),

    CONSTRAINT fk_article_id   FOREIGN KEY(article_id)  REFERENCES articles(id) ON DELETE CASCADE
)
