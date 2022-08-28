use std::sync::Arc;

use crate::{db::DbConnPool, repo::UserRepoPg};

/// The (global) state of the app.
pub struct AppState {
    pub dbcp: Arc<DbConnPool>,
    pub user_repo: UserRepoPg,
}

impl AppState {
    pub fn new(dbcp: DbConnPool) -> Self {
        let dbcp = Arc::new(dbcp);
        Self {
            dbcp: dbcp.clone(),
            user_repo: UserRepoPg::new(dbcp),
        }
    }
}
