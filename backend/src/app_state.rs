use crate::db::DbConnPool;

/// The (global) state of the app.
pub struct AppState {
    pub db_conn_pool: DbConnPool,
}
