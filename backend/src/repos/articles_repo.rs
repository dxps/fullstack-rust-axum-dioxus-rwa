use crate::{
    db::DbConnPool,
    domain::model::{Article, UserProfile},
    AppError,
};

use log::warn;
use sqlx::{postgres::PgRow, Row};
use std::sync::Arc;

/// A Postgres specific implementation of `UserRepo`.
#[derive(Clone)]
pub struct ArticlesRepo {
    dbcp: Arc<DbConnPool>,
}

impl ArticlesRepo {
    pub fn new(dbcp: Arc<DbConnPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get_articles(&self) -> Result<Vec<Article>, AppError> {
        //
        let res = sqlx::query(
            "select count(fa.user_id) as favorites_count,
                    a.id, a.slug, a.title, a.description, a.body, a.created_at, a.updated_at,
                    u.id as user_id, u.username, u.bio, u.image, count(f.user_id) as following 
            from articles a
            join accounts u on a.author_id = u.id 
            left outer join followings f on u.id = f.user_id 
            left outer join favorited_articles fa on a.id = fa.article_id
            group by a.id, u.id, u.username, u.bio, u.image;",
        )
        .map(|r: PgRow| {
            let following = r.get::<i64, _>("following") > 0;
            let author: UserProfile = UserProfile {
                user_id: r.get("user_id"),
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

    pub async fn get_article(&self, slug: String) -> Result<Option<Article>, AppError> {
        //
        sqlx::query(
            "SELECT COUNT(fa.user_id), a.id, a.slug, a.title, a.description, a.body, a.created_at, a.updated_at,
             u.id as user_id, u.username, u.bio, u.image, COUNT(f.user_id) as following
             FROM articles a
             JOIN accounts u ON a.author_id = u.id
             LEFT OUTER JOIN followings f ON u.id = f.user_id
             LEFT OUTER JOIN favorited_articles fa ON a.id = fa.article_id
             WHERE a.slug = $1
             GROUP BY a.id, u.id, u.username, u.bio, u.image"
        )
        .bind(slug)
        .map(|r: PgRow| {
            let following = r.get::<i64, _>("following") > 0;
            let author: UserProfile = UserProfile {
                user_id: r.get("user_id"),
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
        }).fetch_optional(self.dbcp.as_ref()).await.map_err(|e| AppError::from(e))
    }

    /// Add an `Article` into the store. It updates its `id`, `created_at` and `updated_at` attributes.
    pub async fn add(&self, a: &mut Article) -> Result<(), AppError> {
        //
        let mut txn = self.dbcp.begin().await?;

        match sqlx::query(
            "INSERT INTO articles(slug, title, description, body, author_id) 
            VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at",
        )
        .bind(&a.slug)
        .bind(&a.title)
        .bind(&a.description)
        .bind(&a.body)
        .bind(&a.author.user_id)
        .fetch_one(&mut txn)
        .await
        {
            Ok(row) => {
                a.id = row.get("id");
                a.created_at = row.get("created_at");
            }
            Err(err) => {
                dbg!(&err);
                let mut res_err = AppError::Ignorable;
                if let Some(e) = err.as_database_error() {
                    if let Some(code) = e.code() {
                        if code == "23505" && e.message().contains("slug") {
                            res_err = AppError::AlreadyExists(format!("slug '{}'", a.slug))
                        }
                    }
                } else {
                    res_err = AppError::from(err)
                };

                if let Err(e) = txn.rollback().await {
                    log::error!(
                        "Txn rollback of 1st insert on ArticlesRepo::add(_) failed: {}",
                        e
                    )
                };
                return Err(res_err);
            }
        }

        for tag in &a.tag_list {
            if let Err(err) =
                sqlx::query("INSERT INTO tags_articles(tag, article_id) VALUES ($1, $2)")
                    .bind(tag)
                    .bind(a.id)
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

        Ok(())
    }

    pub async fn delete(&self, slug: String) -> Result<(), AppError> {
        //
        sqlx::query("DELETE FROM articles WHERE slug=$1")
            .bind(slug)
            .execute(self.dbcp.as_ref())
            .await?;
        Ok(())
    }

    pub async fn update(&self, a: &mut Article) -> Result<(), AppError> {
        //
        let mut txn = self.dbcp.begin().await?;
        if let Err(err) = sqlx::query(
            "UPDATE articles SET slug=$1, title=$2, description=$3, body=$4, updated_at=$5 
            WHERE slug=$1 RETURNING id",
        )
        .bind(&a.slug)
        .bind(&a.title)
        .bind(&a.description)
        .bind(&a.body)
        .bind(a.updated_at)
        .map(|r: PgRow| a.id = r.get("id"))
        .fetch_one(&mut txn)
        .await
        {
            log::debug!("Error on update: {}", err);
            // Same error handling as in `add`.
            // TODO: Include the following logic into `AppError::from(err)`.
            let mut res_err = AppError::Ignorable;
            if let Some(e) = err.as_database_error() {
                if let Some(code) = e.code() {
                    if code == "23505" && e.message().contains("slug") {
                        res_err = AppError::AlreadyExists(format!("slug '{}'", a.slug))
                    }
                }
            } else {
                res_err = AppError::from(err)
            };

            if let Err(e) = txn.rollback().await {
                log::error!("Txn rollback after update on articles failed: {}", e)
            };
            return Err(res_err);
        }

        if let Err(err) = sqlx::query("DELETE FROM tags_articles WHERE article_id=$1")
            .bind(a.id)
            .execute(&mut txn)
            .await
        {
            log::debug!("Error on delete tags: {}", err);
            let res_err = AppError::from(err);
            if let Err(e) = txn.rollback().await {
                log::error!("Txn rollback after delete from tags_articles failed: {}", e)
            };
            return Err(res_err);
        }

        for tag in &a.tag_list {
            if let Err(err) =
                sqlx::query("INSERT INTO tags_articles(tag, article_id) VALUES ($1, $2)")
                    .bind(tag)
                    .bind(a.id)
                    .execute(&mut txn)
                    .await
            {
                match txn.rollback().await {
                    Ok(()) => (),
                    Err(e) => {
                        warn!(
                            "Txn rollback after inserting into tags_articles failed: {}",
                            e
                        )
                    }
                }
                return Err(AppError::from(err));
            }
        }

        txn.commit().await?;
        Ok(())
    }
}
