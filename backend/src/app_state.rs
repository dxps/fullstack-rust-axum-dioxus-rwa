use std::sync::Arc;

use crate::{db::DbConnPool, domain::logic::AuthMgr, repo::UserRepo};

/// The (global) state of the app.
pub struct AppState {
    pub dbcp: Arc<DbConnPool>,
    pub user_repo: Arc<UserRepo>,
    pub auth_mgr: AuthMgr,
}

impl AppState {
    pub fn new(dbcp: DbConnPool) -> Self {
        let dbcp = Arc::new(dbcp);
        let user_repo = Arc::new(UserRepo::new(dbcp.clone()));
        let auth_mgr = AuthMgr::new(user_repo.clone());
        Self {
            dbcp,
            user_repo,
            auth_mgr,
        }
    }
}
