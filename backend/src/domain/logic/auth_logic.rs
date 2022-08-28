use std::sync::Arc;

use crate::{domain::model::User, repo::UserRepo, AppError};

pub struct AuthMgr {
    user_repo: Arc<UserRepo>,
}

impl AuthMgr {
    /// Create a new instance of `AuthMgr`.
    pub fn new(user_repo: Arc<UserRepo>) -> Self {
        Self { user_repo }
    }

    /// Register a new `User`.
    pub async fn register_user(&self, user: &User, pwd: String) -> Result<(), AppError> {
        match self.user_repo.save(&user, pwd).await {
            Ok(()) => {
                log::debug!("Registered user with username {:?}.", user.username);
                Ok(())
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
}
