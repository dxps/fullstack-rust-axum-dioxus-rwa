use serde::Serialize;

/// The (public) id of the User.
#[derive(Debug, Serialize, Default)]
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

/// The main representation of the User. <br/>
/// It contains most of the details (except for password).
#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
}

/// It includes all user attributes that are persisted in the database.
pub struct UserEntry {
    pub user: User,
    pub password: String,
    pub salt: String,
}

impl From<UserEntry> for User {
    fn from(val: UserEntry) -> Self {
        Self {
            id: val.user.id,
            email: val.user.email,
            username: val.user.username,
            bio: val.user.bio,
            image: val.user.image,
        }
    }
}

/// A common representation of a `User`, used in multiple use cases.
#[derive(Clone, Debug, Serialize)]
pub struct UserProfile {
    #[serde(skip_serializing)]
    pub user_id: i64,
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
    pub following: bool,
}

impl UserProfile {
    pub fn new_basic(user_id: i64) -> Self {
        Self {
            user_id,
            username: "".into(),
            bio: "".into(),
            image: None,
            following: false,
        }
    }
}
