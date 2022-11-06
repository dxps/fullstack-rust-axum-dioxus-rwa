use std::sync::Arc;

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
            join followings f on u.id = f.user_id 
            join favorited_articles fa on a.id = fa.article_id
            group by a.id, u.username, u.bio, u.image;",
        )
        .map(|r: PgRow| {
            let following = r.get("following");

            let author: UserProfile = UserProfile {
                username: r.get("username"),
                bio: r.get("bio"),
                image: Some(r.get("image")),
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
}
