use std::sync::Arc;

use sqlx::{postgres::PgRow, FromRow, Row};

use crate::{db::DbConnPool, domain::model::User};

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            email: row.get("email"),
            username: row.get("username"),
            bio: row.get("bio"),
            image: row.get("image"),
        })
    }
}

/// A Postgres specific implementation of `UserRepo`.
pub struct UserRepoPg {
    dbcp: Arc<DbConnPool>,
}

impl UserRepoPg {
    pub fn new(dbcp: Arc<DbConnPool>) -> Self {
        Self { dbcp }
    }

    pub async fn save(&self, user: &User, password: String) -> Result<(), crate::AppError> {
        log::debug!("Saving {:?}", user);
        let (pwd, salt) = Self::generate_password(password.into());
        let _ = sqlx::query("INSERT INTO accounts(email, username, password, salt, bio, image) VALUES ($1, $2, $3, $4, $5, $6);")
        .bind(&user.email).bind(&user.username)
        .bind(pwd).bind(salt).bind(&user.bio).bind(&user.image)
        .execute(self.dbcp.as_ref()).await?;
        Ok(())
    }

    fn generate_password(pwd: String) -> (String, String) {
        let salt: String = std::iter::repeat_with(fastrand::alphanumeric)
            .take(12)
            .collect();
        let digest = md5::compute(format!("@{salt}${pwd}").as_bytes());
        (format!("{:x}", digest), salt)
    }
}
