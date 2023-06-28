use crate::{
    db::DbConnPool,
    domain::model::{User, UserEntry, UserId, UserProfile},
    AppError, AppUseCase,
};
use sqlx::{postgres::PgRow, FromRow, Row};
use std::sync::Arc;

pub struct UsersRepo {
    dbcp: Arc<DbConnPool>,
}

impl UsersRepo {
    //
    pub fn new(dbcp: Arc<DbConnPool>) -> Self {
        Self { dbcp }
    }

    pub async fn save(&self, user: &User, pwd: String, salt: String) -> Result<i64, AppError> {
        //
        match sqlx::query(
            "INSERT INTO accounts (email, username, password, salt) 
             VALUES ($1, $2, $3, $4) RETURNING id",
        )
        .bind(&user.email)
        .bind(&user.username)
        .bind(pwd)
        .bind(salt)
        .fetch_one(self.dbcp.as_ref())
        .await
        {
            Ok(row) => Ok(row.get("id")),
            Err(err) => Err(AppError::from((err, AppUseCase::UserRegistration))),
        }
    }

    pub async fn get_by_email(
        &self,
        email: &String,
        usecase: AppUseCase,
    ) -> Result<UserEntry, AppError> {
        //
        sqlx::query_as::<_, UserEntry>(
            "SELECT id, email, username, password, salt, bio, image FROM accounts 
             WHERE email = $1",
        )
        .bind(email)
        .fetch_one(self.dbcp.as_ref())
        .await
        .map_err(|err| AppError::from((err, usecase)))
    }

    pub async fn get_by_id(&self, id: &UserId, usecase: AppUseCase) -> Result<UserEntry, AppError> {
        //
        let entry = sqlx::query_as::<_, UserEntry>(
            "SELECT email, username, password, salt, bio, image FROM accounts 
             WHERE id = $1",
        )
        .bind(id.as_value())
        .fetch_one(self.dbcp.as_ref())
        .await;
        match entry {
            Ok(entry) => Ok(entry),
            Err(err) => Err(AppError::from((err, usecase))),
        }
    }

    pub async fn follow_user(
        &self,
        curr_user_id: &UserId,
        followed_username: &String,
    ) -> Result<UserProfile, AppError> {
        //
        let followed_user_id: UserId;
        match sqlx::query_as::<_, UserId>("SELECT id FROM accounts WHERE username = $1")
            .bind(followed_username)
            .fetch_one(self.dbcp.as_ref())
            .await
        {
            Ok(id) => followed_user_id = id,
            Err(err) => match err {
                sqlx::Error::RowNotFound => {
                    return Err(AppError::NotFound("followed username was not found".into()))
                }
                _ => return Err(AppError::InternalErr),
            },
        };

        if curr_user_id.as_value() == followed_user_id.as_value() {
            return Err(AppError::InvalidRequest(
                "a user cannot follow himself".into(),
            ));
        }

        match sqlx::query("INSERT INTO followings VALUES($1, $2)")
            .bind(curr_user_id.as_value())
            .bind(followed_user_id.as_value())
            .execute(self.dbcp.as_ref())
            .await
        {
            Ok(_) => {
                self.get_profile_by_username(
                    curr_user_id,
                    followed_username,
                    AppUseCase::FollowUser,
                )
                .await
            }
            Err(err) => Err(AppError::from((err, AppUseCase::FollowUser))),
        }
    }

    pub async fn unfollow_user(
        &self,
        curr_user_id: &UserId,
        followed_username: &String,
    ) -> Result<UserProfile, AppError> {
        //
        let followed_user_id =
            sqlx::query_as::<_, UserId>("SELECT id FROM accounts WHERE username = $1")
                .bind(followed_username)
                .fetch_one(self.dbcp.as_ref())
                .await?;
        match sqlx::query("DELETE FROM followings WHERE user_id = $1 AND followed_user_id = $2")
            .bind(curr_user_id.as_value())
            .bind(followed_user_id.as_value())
            .execute(self.dbcp.as_ref())
            .await
        {
            Ok(_) => {
                self.get_profile_by_username(
                    curr_user_id,
                    followed_username,
                    AppUseCase::FollowUser,
                )
                .await
            }
            Err(err) => Err(AppError::from((err, AppUseCase::FollowUser))),
        }
    }

    pub async fn get_profile_by_username(
        &self,
        curr_user_id: &UserId,
        username: &String,
        usecase: AppUseCase,
    ) -> Result<UserProfile, AppError> {
        //
        let mut user_id = 0_i64;
        let res = sqlx::query(
            "SELECT id, bio, image, COUNT(f.user_id) AS following FROM accounts a
             LEFT OUTER JOIN followings f ON f.followed_user_id = a.id AND f.user_id = $2
             WHERE a.username = $1
             GROUP BY a.id",
        )
        .bind(username)
        .bind(curr_user_id.as_value())
        .map(|row: PgRow| {
            user_id = row.get("id");
            UserProfile {
                user_id,
                username: username.clone(),
                bio: row.get("bio"),
                image: row.get("image"),
                following: row.get::<i64, _>("following") == 1,
            }
        })
        .fetch_one(self.dbcp.as_ref())
        .await;
        match res {
            Ok(result) => Ok(result),
            Err(err) => Err(AppError::from((err, usecase))),
        }
    }

    pub async fn get_profile_by_id(&self, user_id: i64) -> Result<UserProfile, AppError> {
        //
        let res = sqlx::query(
            "SELECT username, bio, image, COUNT(f.user_id) AS following FROM accounts a
             LEFT OUTER JOIN followings f ON f.followed_user_id = a.id AND f.user_id = $1
             WHERE a.id = $1
             GROUP BY a.username, a.bio, a.image",
        )
        .bind(user_id)
        .map(|row: PgRow| UserProfile {
            user_id,
            username: row.get("username"),
            bio: row.get("bio"),
            image: row.get("image"),
            following: row.get::<i64, _>("following") == 1,
        })
        .fetch_one(self.dbcp.as_ref())
        .await;
        match res {
            Ok(result) => Ok(result),
            Err(err) => Err(AppError::from(err)),
        }
    }

    async fn _get_followings(&self, user_id: i64) -> Result<Vec<UserId>, AppError> {
        //
        let result = sqlx::query("SELECT followed_user_id FROM followings WHERE user_id = $1")
            .bind(user_id)
            .map(|row: PgRow| UserId::from(row.get::<i64, _>("followed_user_id")))
            .fetch_all(self.dbcp.as_ref())
            .await?;
        Ok(result)
    }

    pub async fn update_by_id(
        &self,
        id: UserId,
        email: Option<String>,
        bio: Option<String>,
        image: Option<String>,
    ) -> Result<UserEntry, AppError> {
        //
        if email.is_none() && bio.is_none() && image.is_none() {
            return Err(AppError::InvalidRequest(
                "email, bio, and image is missing from request body".into(),
            ));
        }
        match self.get_by_id(&id, AppUseCase::UpdateUser).await {
            Ok(mut entry) => {
                entry.user.email = email.unwrap_or(entry.user.email);
                entry.user.bio = bio.unwrap_or(entry.user.bio);
                entry.user.image = if image.is_some() {
                    image
                } else {
                    entry.user.image
                };
                match sqlx::query(
                    "UPDATE accounts SET email = $1, bio = $2, image = $3 WHERE id = $4",
                )
                .bind(&entry.user.email)
                .bind(&entry.user.bio)
                .bind(&entry.user.image)
                .bind(id.as_value())
                .execute(self.dbcp.as_ref())
                .await
                {
                    Ok(_) => Ok(entry),
                    Err(err) => Err(AppError::from((err, AppUseCase::UpdateUser))),
                }
            }
            Err(err) => Err(err),
        }
    }
}

// ---------------------------------------
//    sqlx::FromRow implementations
// ---------------------------------------

impl FromRow<'_, PgRow> for User {
    //
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

impl FromRow<'_, PgRow> for UserId {
    //
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(UserId::from(row.get::<i64, _>("id")))
    }
}

impl FromRow<'_, PgRow> for UserEntry {
    //
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
