use std::sync::Arc;

use sqlx::{postgres::PgRow, FromRow, Row};

use crate::{
    db::DbConnPool,
    domain::model::{User, UserEntry, UserId},
    AppError, AppUseCase,
};

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
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
                id: row.try_get("id").unwrap_or_default(),
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

    pub async fn save(&self, user: &User, pwd: String, salt: String) -> Result<i64, AppError> {
        match sqlx::query(
            "INSERT INTO accounts(email, username, password, salt) VALUES ($1, $2, $3, $4) RETURNING id",
        )
        .bind(&user.email)
        .bind(&user.username)
        .bind(pwd)
        .bind(salt)
        .fetch_one(self.dbcp.as_ref())
        .await
        {
            Ok(row) => Ok(row.get("id")),
            Err(err) => Err(AppError::from((err, AppUseCase::UserRegister))),
        }
    }

    pub async fn get_by_email(
        &self,
        email: &String,
        usecase: AppUseCase,
    ) -> Result<UserEntry, AppError> {
        let entry = sqlx::query_as::<_, UserEntry>(
            "SELECT id, email, username, password, salt, bio, image FROM accounts WHERE email = $1",
        )
        .bind(&email)
        .fetch_one(self.dbcp.as_ref())
        .await;
        match entry {
            Ok(entry) => Ok(entry),
            Err(err) => Err(AppError::from((err, usecase))),
        }
    }

    pub async fn get_by_id(&self, id: UserId, usecase: AppUseCase) -> Result<UserEntry, AppError> {
        let entry = sqlx::query_as::<_, UserEntry>(
            "SELECT email, username, password, salt, bio, image FROM accounts WHERE id = $1",
        )
        .bind(id.as_value())
        .fetch_one(self.dbcp.as_ref())
        .await;
        match entry {
            Ok(entry) => Ok(entry),
            Err(err) => Err(AppError::from((err, usecase))),
        }
    }
}
