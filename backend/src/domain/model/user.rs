/// The (public) id of the User.
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

/// The main representation of the User, containing most (except password) of the details of a user.
#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
}

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