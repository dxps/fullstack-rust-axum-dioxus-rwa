use std::sync::Arc;

use chrono::{DateTime, Utc};
use log::warn;
use sqlx::{postgres::PgRow, Row};

use crate::{
    db::DbConnPool,
    domain::model::{Article, UserProfile},
    AppError,
};

/// A Postgres specific implementation of `UserRepo`.
pub struct ArticlesRepo {
    dbcp: Arc<DbConnPool>,
}

impl ArticlesRepo {
    pub fn new(dbcp: Arc<DbConnPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get_articles(&self) -> Result<Vec<Article>, AppError> {
        let res = sqlx::query(
            "select count(fa.user_id) as favorites_count,
                    a.id, a.slug, a.title, a.description, a.body, a.created_at, a.updated_at,
                    u.username, u.bio, u.image, count(f.user_id) as following 
            from articles a
            join accounts u on a.author_id = u.id 
            left outer join followings f on u.id = f.user_id 
            left outer join favorited_articles fa on a.id = fa.article_id
            group by a.id, u.username, u.bio, u.image;",
        )
        .map(|r: PgRow| {
            let following = r.get::<i64, _>("following") > 0;

            let author: UserProfile = UserProfile {
                username: r.get("username"),
                bio: r.get("bio"),
                image: r.try_get("image").unwrap_or_default(),
                following,
            };
            Article::new(
                r.get("slug"),
                r.get("title"),
                r.get("description"),
                r.get("body"),
                r.get("created_at"),
                r.get("updated_at"),
                author,
            )
        })
        .fetch_all(self.dbcp.as_ref())
        .await;
        match res {
            Ok(entry) => Ok(entry),
            Err(err) => Err(AppError::from(err)),
        }
    }

    pub async fn add(
        &self,
        slug: &String,
        title: &String,
        description: &String,
        body: &String,
        tag_list: &Vec<String>,
        author_id: i64,
    ) -> Result<DateTime<Utc>, AppError> {
        //
        let mut txn = self.dbcp.begin().await?;
        let article_id: i64;
        let created_at: DateTime<Utc>;

        match sqlx::query(
            "INSERT INTO articles(slug, title, description, body, author_id) 
            VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at",
        )
        .bind(slug)
        .bind(title)
        .bind(description)
        .bind(body)
        .bind(author_id)
        .fetch_one(&mut txn)
        .await
        {
            Ok(row) => {
                article_id = row.get("id");
                created_at = row.get("created_at");
            }
            Err(err) => {
                if let Err(e) = txn.rollback().await {
                    warn!(
                        "Txn rollback of 1st insert on ArticlesRepo::add failed: {}",
                        e
                    )
                };
                return Err(AppError::from(err));
            }
        }

        for tag in tag_list {
            if let Err(err) =
                sqlx::query("INSERT INTO tags_articles(tag,article_id) VALUES ($1, $2)")
                    .bind(tag)
                    .bind(article_id)
                    .execute(&mut txn)
                    .await
            {
                match txn.rollback().await {
                    Ok(()) => (),
                    Err(e) => {
                        warn!(
                            "Txn rollback of 2nd insert on ArticlesRepo::add failed: {}",
                            e
                        )
                    }
                }
                return Err(AppError::from(err));
            }
        }

        txn.commit().await?;

        Ok(created_at)
    }
}
