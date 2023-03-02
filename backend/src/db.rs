use crate::config::AppConfig;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub type DbConnPool = sqlx::Pool<sqlx::Postgres>;

pub async fn init_db_pool(cfg: &AppConfig) -> Result<DbConnPool, sqlx::Error> {
    //
    let db_url = cfg.database.connection_string();
    let db_url = db_url.expose_secret();
    PgPoolOptions::new()
        .idle_timeout(Duration::from_secs(3))
        .min_connections(1)
        .max_connections(30)
        .max_lifetime(Duration::from_secs(60 * 60))
        .connect_lazy(db_url)
}

pub async fn ping_db(conn: &DbConnPool) -> bool {
    //
    let z = sqlx::query("SELECT 1").execute(conn).await;
    match z {
        Ok(_) => true,
        Err(e) => {
            log::error!("Failed to ping the database: {e}");
            false
        }
    }
}
