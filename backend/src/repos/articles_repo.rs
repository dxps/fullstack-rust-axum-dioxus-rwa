use crate::{
    db::DbConnPool,
    domain::model::{Article, UserProfile},
    AppError,
};
use sqlx::{postgres::PgRow, Pool, Postgres, Row, Transaction};
use std::sync::Arc;

#[derive(Clone)]
pub struct ArticlesRepo {
    dbcp: Arc<DbConnPool>,
}

impl ArticlesRepo {
    //
    pub fn new(dbcp: Arc<DbConnPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get_articles(&self) -> Result<Vec<Article>, AppError> {
        //
        let conn = self.dbcp.as_ref();
        let mut res = sqlx::query(
            "SELECT count(fa.user_id) AS favorites_count,
                    a.id, a.slug, a.title, a.description, a.body, a.created_at, a.updated_at,
                    u.id as user_id, u.username, u.bio, u.image, count(f.user_id) as following 
            FROM articles a
            JOIN accounts u on a.author_id = u.id 
            LEFT OUTER JOIN followings f ON u.id = f.user_id 
            LEFT OUTER JOIN favorited_articles fa ON a.id = fa.article_id
            GROUP BY a.id, u.id, u.username, u.bio, u.image;",
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
                r.get("id"),
                r.get("slug"),
                r.get("title"),
                r.get("description"),
                r.get("body"),
                r.get("created_at"),
                r.get("updated_at"),
                author,
            )
        })
        .fetch_all(conn)
        .await
        .map_err(AppError::from);

        if let Ok(ref mut articles) = res {
            for a in articles {
                self.get_tags(conn, a).await?
            }
        }

        res
    }

    pub async fn get_article(&self, slug: &String) -> Result<Option<Article>, AppError> {
        //
        let conn = self.dbcp.as_ref();
        let mut article = sqlx::query(
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
                r.get("id"),
                r.get("slug"),
                r.get("title"),
                r.get("description"),
                r.get("body"),
                r.get("created_at"),
                r.get("updated_at"),
                author,
            )
        }).fetch_optional(conn).await?;

        if let Some(ref mut a) = article {
            self.get_tags(conn, a).await?;
            return Ok(Some(a.clone()));
        }

        Ok(article)
    }

    async fn get_tags(&self, conn: &Pool<Postgres>, a: &mut Article) -> Result<(), AppError> {
        //
        sqlx::query("SELECT tag FROM tags_articles WHERE article_id = $1")
            .bind(a.id)
            .map(|r: PgRow| a.tag_list.push(r.get("tag")))
            .fetch_all(conn)
            .await?;
        Ok(())
    }

    /// Add an `Article` into the store. It updates its `id`, `created_at` and `updated_at` attributes.
    pub async fn add(&self, a: &mut Article) -> Result<(), AppError> {
        //
        let mut txn = self.dbcp.begin().await?;

        match sqlx::query(
            "INSERT INTO articles (slug, title, description, body, author_id) 
            VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at",
        )
        .bind(&a.slug)
        .bind(&a.title)
        .bind(&a.description)
        .bind(&a.body)
        .bind(a.author.user_id)
        .fetch_one(&mut *txn)
        .await
        {
            Ok(row) => {
                a.id = row.get("id");
                a.created_at = row.get("created_at");
                a.updated_at = a.created_at
            }
            Err(err) => {
                let res_err = ArticlesRepo::render_app_error(err, &a.slug);

                if let Err(e) = txn.rollback().await {
                    log::error!("Txn rollback after insert into articles failed: {}", e)
                };
                return Err(res_err);
            }
        }

        if self
            .set_tags(&mut txn, a.id, &a.tag_list, true)
            .await
            .is_err()
        {
            if let Err(err) = &mut txn.rollback().await {
                log::error!("Txn rollback after set_tags(_) failed: {}", err);
                return Err(AppError::InternalErr);
            }
        } else {
            txn.commit().await?;
        }
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
        .fetch_one(&mut *txn)
        .await
        {
            let res_err = ArticlesRepo::render_app_error(err, &a.slug);

            if let Err(e) = txn.rollback().await {
                log::error!("Txn rollback after update on articles failed: {}", e)
            };
            return Err(res_err);
        }

        if self
            .set_tags(&mut txn, a.id, &a.tag_list, false)
            .await
            .is_err()
        {
            if let Err(err) = &mut txn.rollback().await {
                log::error!("Txn rollback failed: {}", err);
                return Err(AppError::InternalErr);
            }
        } else {
            txn.commit().await?;
        }
        Ok(())
    }

    async fn set_tags<'a>(
        &self,
        txn: &mut Transaction<'a, Postgres>,
        article_id: i64,
        tag_list: &Vec<String>,
        is_new: bool,
    ) -> Result<(), sqlx::Error> {
        //
        if !is_new {
            if let Err(err) = sqlx::query("DELETE FROM tags_articles WHERE article_id=$1")
                .bind(article_id)
                .execute(&mut **txn)
                .await
            {
                log::error!("Failed to delete tags: {}", err);
                return Err(err);
            }
        }
        for tag in tag_list {
            if let Err(err) =
                sqlx::query("INSERT INTO tags_articles (tag, article_id) VALUES ($1, $2)")
                    .bind(tag)
                    .bind(article_id)
                    .execute(&mut **txn)
                    .await
            {
                log::error!("Failed to insert tags: {}", err);
                return Err(err);
            }
        }
        Ok(())
    }

    fn render_app_error(from_db_err: sqlx::Error, slug: &String) -> AppError {
        //
        let mut res_err = AppError::Ignorable;
        if let Some(e) = from_db_err.as_database_error() {
            if let Some(code) = e.code() {
                if code == "23505" && e.message().contains("slug") {
                    res_err = AppError::AlreadyExists(format!("slug '{}'", slug))
                }
            }
        } else {
            res_err = AppError::from(from_db_err)
        };
        res_err
    }
}
