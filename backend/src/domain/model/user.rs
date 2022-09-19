use serde::Serialize;

/// The (public) id of the User.
#[derive(Debug, Serialize)]
pub struct UserId(i64);

impl UserId {
    pub fn as_value(&self) -> i64 {
        self.0
    }
}

impl From<i64> for UserId {
    fn from(id: i64) -> Self {
        UserId(id)
    }
}

/// The main representation of the User.<br/>
/// It contains most of the details (except of the password).
#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
}

/// This struct represents the whole user instance, as persisted in the database.
pub struct UserEntry {
    pub user: User,
    pub password: String,
    pub salt: String,
}

impl Into<User> for UserEntry {
    fn into(self) -> User {
        self.user
    }
}

/// A common and concise representation of a `User`,
/// included into various use cases responses.
#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
    pub following: bool,
}
