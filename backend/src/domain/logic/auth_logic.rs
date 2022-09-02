use std::sync::Arc;

use crate::{domain::model::User, repo::UserRepo, AppError, AppUseCase};

pub struct AuthMgr {
    user_repo: Arc<UserRepo>,
}

impl AuthMgr {
    /// Create a new instance of `AuthMgr`.
    pub fn new(user_repo: Arc<UserRepo>) -> Self {
        Self { user_repo }
    }

    /// Register a new `User`.
    pub async fn register_user(&self, user: &User, pwd: String) -> Result<i64, AppError> {
        let (pwd, salt) = Self::generate_password(pwd.into());
        match self.user_repo.save(&user, pwd, salt).await {
            Ok(id) => {
                log::debug!("Registered user with username {:?}.", user.username);
                Ok(id)
            }
            Err(e) => {
                log::error!(
                    "Failed to register user with username {:?}: {}",
                    user.username,
                    e
                );
                Err(e)
            }
        }
    }

    /// Login a `User`.
    pub async fn login_user(&self, email: String, pwd: String) -> Result<User, AppError> {
        match self
            .user_repo
            .get_by_email(&email, AppUseCase::UserLogin)
            .await
        {
            Ok(user_entry) => {
                match Self::check_password(&pwd, &user_entry.password, &user_entry.salt) {
                    true => {
                        log::debug!("Successful login for email {:?}.", email);
                        Ok(user_entry.into())
                    }
                    false => {
                        log::debug!("Wrong login credentials for email {:?}", email);
                        Err(AppError::LoginWrongCredentialsErr)
                    }
                }
            }
            Err(err) => {
                log::debug!("Failed login for email {:?}: {}", email, err);
                Err(err)
            }
        }
    }

    fn generate_password(pwd: String) -> (String, String) {
        let salt: String = std::iter::repeat_with(fastrand::alphanumeric)
            .take(12)
            .collect();
        let digest = md5::compute(format!("@{salt}${pwd}").as_bytes());
        (format!("{:x}", digest), salt)
    }

    fn check_password(input_pwd: &str, pwd: &str, salt: &str) -> bool {
        let digest = md5::compute(format!("@{salt}${input_pwd}").as_bytes());
        pwd == format!("{:x}", digest)
    }
}
