use std::sync::Arc;

use sqlx::{postgres::PgRow, FromRow, Row};

use crate::{
    db::DbConnPool,
    domain::model::{User, UserEntry},
    AppError, AppUseCase,
};

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

impl FromRow<'_, PgRow> for UserEntry {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user: User {
                email: row.get("email"),
                username: row.get("username"),
                bio: row.get("bio"),
                image: row.try_get("image").unwrap_or_default(),
            },
            password: row.get("password"),
            salt: row.get("salt"),
        })
    }
}

/// A Postgres specific implementation of `UserRepo`.
pub struct UserRepo {
    dbcp: Arc<DbConnPool>,
}

impl UserRepo {
    pub fn new(dbcp: Arc<DbConnPool>) -> Self {
        Self { dbcp }
    }

    pub async fn save(&self, user: &User, pwd: String, salt: String) -> Result<(), AppError> {
        match sqlx::query(
            "INSERT INTO accounts(email, username, password, salt) VALUES ($1, $2, $3, $4);",
        )
        .bind(&user.email)
        .bind(&user.username)
        .bind(pwd)
        .bind(salt)
        .execute(self.dbcp.as_ref())
        .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(AppError::from((err, AppUseCase::UserRegister))),
        }
    }

    pub async fn get_by_email(
        &self,
        email: &String,
        usecase: AppUseCase,
    ) -> Result<UserEntry, AppError> {
        let entry = sqlx::query_as::<_, UserEntry>(
            "SELECT email, username, password, salt, bio, image FROM accounts WHERE email = $1",
        )
        .bind(&email)
        .fetch_one(self.dbcp.as_ref())
        .await;
        match entry {
            Ok(entry) => Ok(entry),
            Err(err) => Err(AppError::from((err, usecase))),
        }
    }
}
